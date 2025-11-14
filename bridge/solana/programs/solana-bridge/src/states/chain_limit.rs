
use anchor_lang::prelude::*;
use crate::errors::BridgeLimiterError;
pub const CHAIN_LIMIT_SEED: &str = "chain_limit";
pub const HOUR_TRANSFER_NUM: usize = 24;
pub const USD_PRECISION: u64 = 1_00_000_000;

#[account(zero_copy)]
#[repr(C, packed)]
#[derive(Default,Debug)]
pub struct ChainLimit {
    pub bump: [u8;1],
    pub config : Pubkey,
    pub chain_id: [u8;1],
    //single transfer limit
    pub max_usd_limit: u64, 
    //24-hour total limit
    pub total_limit: u64, 
    //current hour index
    pub hourly_transfer_index: u8, 
    //24-hour sliding window
    pub hourly_transfer_amount: [ChainHourlyTransferAmount; HOUR_TRANSFER_NUM], 
    //padding upgrade
    pub padding: [u64; 8],
}

#[zero_copy]
#[repr(C, packed)]
#[derive(Default, Debug)]
pub struct ChainHourlyTransferAmount {
    pub hour: u64,  
    pub amount: u64,
}


impl ChainLimit {
    pub const SPACE: usize = 8 + std::mem::size_of::<Self>();

    pub fn initialize(
        &mut self,
        bump: u8,
        config: Pubkey,
        chain_id: u8,
        max_usd_limit: u64,
        limit: u64,
    ) {
        self.bump = [bump];
        self.config = config;
        self.chain_id = [chain_id];
        self.max_usd_limit=max_usd_limit;
        self.total_limit = limit;
    }
    

    pub fn key(&self) -> Pubkey {
        Pubkey::create_program_address(
            &self.seeds(),
            &crate::id(),
        ).unwrap()
    }

    pub fn seeds(&self) -> [&[u8]; 4] {
        [
            CHAIN_LIMIT_SEED.as_bytes(),
            self.chain_id.as_ref(),
            self.config.as_ref(),
            self.bump.as_ref(),
        ]
    }

    pub fn get_single_transfer_limit(&self) -> u64 {
        self.max_usd_limit 
    }

    pub fn get_chain_id(&self) -> u8 {
        self.chain_id[0]
    }

    pub fn set_single_transfer_limit(&mut self, limit: u64) {
        self.max_usd_limit = limit;
    }

    pub fn set_total_limit(&mut self, limit: u64) {
        self.total_limit = limit;
    }

    pub fn current_hour(&self) -> Result<u64> {
        let clock = Clock::get()?;
        Ok((clock.unix_timestamp / 3600) as u64)
    }

    pub fn calculate_amount_in_usd(&self, amount: u64,price: u64, decimal: u8) -> u64 {
        ((amount as u128 * price as u128) / 10u128.pow(decimal as u32)) as u64
    }

    /// Calculate total transfer amount within sliding window (24 hours from current time) in USD
    pub fn calculate_window_limit(&mut self) -> u64 {
        let current_hour = match self.current_hour() {
            Ok(hour) => hour,
            Err(_) => return 0,
        };
        self.calculate_window_limit_internal(current_hour)
    }
    
    fn calculate_window_limit_internal(&self, current_hour: u64) -> u64 {
        let mut total = 0;
        let last_hour_index = self.hourly_transfer_index as usize;
        
        //Check if the latest recorded time is valid
        let time = self.hourly_transfer_amount[last_hour_index].hour;
        
        if current_hour < time {
            return 0
        }

        let mut index = (last_hour_index + 1) % HOUR_TRANSFER_NUM;

        //include 24 hours containing current time
        let window_start = current_hour.saturating_sub(23);
        
        for _ in 0..HOUR_TRANSFER_NUM {
            let h = &self.hourly_transfer_amount[index];
            
            // Only count valid data within the sliding window
            if h.hour >= window_start && h.hour <= current_hour && h.hour != 0 {
                total += h.amount;
            }
            
            //move to next index 
            index = (index + 1) % HOUR_TRANSFER_NUM;
        }
        
        total
    }

    pub fn will_amount_exceed_limit(&mut self, amount: u64,price: u64, decimal: u8) ->bool{
        let amount_in_usd = self.calculate_amount_in_usd(amount, price, decimal);
        self.calculate_window_limit() + amount_in_usd > self.total_limit
    }

    pub fn will_usd_amount_exceed_limit_internal(&mut self, current_hour: u64,amount_in_usd: u64) -> bool {
        self.calculate_window_limit_internal(current_hour) + amount_in_usd > self.total_limit
    }


    pub fn record_bridge_transfers(&mut self, amount: u64, price: u64, decimal: u8) -> Result<()> {
        let current_hour = self.current_hour().unwrap();
        self.record_bridge_transfers_internal(current_hour,amount, price, decimal)
    }



    pub fn record_bridge_transfers_internal(&mut self, current_hour: u64,amount: u64, price: u64, decimal: u8) -> Result<()> {
        require!(amount >0, BridgeLimiterError::InvalidAmountIsZero);
        let amount_in_usd = self.calculate_amount_in_usd(amount, price, decimal);
        require!(amount_in_usd > 0, BridgeLimiterError::InvalidAmountIsZero);

        require!(!self.will_usd_amount_exceed_limit_internal(current_hour,amount_in_usd), BridgeLimiterError::ExceedWindowLimit);

        let current_index = self.hourly_transfer_index as usize;
        let current_hour_in_array = self.hourly_transfer_amount[current_index].hour;
        
        if current_hour_in_array == current_hour {
            // same hour, accumulate amount
            self.hourly_transfer_amount[current_index].amount = 
            self.hourly_transfer_amount[current_index].amount.saturating_add(amount_in_usd);        
        } else if current_hour > current_hour_in_array {
            // new hour, need to move to next position
            let hours_diff =current_hour - current_hour_in_array;
            
            if hours_diff >= HOUR_TRANSFER_NUM as u64 {
                //  exceeds 24 hours, clear all data
                for i in 0..HOUR_TRANSFER_NUM {
                    self.hourly_transfer_amount[i] = ChainHourlyTransferAmount::default();
                }
                self.hourly_transfer_index = 0;
            } else {
                //  normal sliding window
                for i in 1..=hours_diff {
                    self.hourly_transfer_index = (self.hourly_transfer_index + 1) % HOUR_TRANSFER_NUM as u8;
                    // Clear data at new position, but set correct hour timestamp
                    let new_index = self.hourly_transfer_index as usize;
                    let hour_for_this_slot = current_hour_in_array + i;
                    self.hourly_transfer_amount[new_index] = ChainHourlyTransferAmount {
                        hour: hour_for_this_slot,
                        amount: 0,
                    };
                }
            }
            
            // Set data for new hour
            let final_index = self.hourly_transfer_index as usize;
            self.hourly_transfer_amount[final_index].hour = current_hour;
            self.hourly_transfer_amount[final_index].amount = amount_in_usd;
        }else{
            //abort
            require!(false,BridgeLimiterError::InvalidHour);
        }

        Ok(())
    }

    pub fn get_total_transfer_amount(&self) -> u64 {
        self.hourly_transfer_amount
            .iter()
            .map(|h| h.amount)
            .sum()
    }
}


#[cfg(test)]    
pub mod chain_limit_test{
    use super::*;

    #[test]
    fn test_calculate_amount_in_usd() {
        let mut chain_limit = ChainLimit::default();

        let price = 1*USD_PRECISION;
        let decimal = 9;
        let amount = 1000*USD_PRECISION;
        let result = amount/10;

        chain_limit.set_single_transfer_limit(amount);
        let amount_in_usd = chain_limit.calculate_amount_in_usd(amount, price, decimal);
        println!("amount_in_usd:{}",amount_in_usd);
       assert_eq!(amount_in_usd, result);
    }


    #[test]
    fn test_calculate_window_limit_internal(){
        //case 1
        let mut chain_limit = ChainLimit::default();
        chain_limit.hourly_transfer_amount[0].amount = 100;
        chain_limit.hourly_transfer_amount[0].hour = 1;
        chain_limit.hourly_transfer_amount[1].amount = 200;
        chain_limit.hourly_transfer_amount[1].hour = 2;
        chain_limit.hourly_transfer_amount[2].amount = 300;
        chain_limit.hourly_transfer_amount[2].hour = 3;
        chain_limit.hourly_transfer_amount[3].amount = 400;
        chain_limit.hourly_transfer_amount[3].hour = 4;
        chain_limit.hourly_transfer_amount[4].amount = 500;
        chain_limit.hourly_transfer_amount[4].hour = 5;
        chain_limit.hourly_transfer_amount[5].amount = 0;
        chain_limit.hourly_transfer_amount[5].hour = 6;
        chain_limit.hourly_transfer_amount[6].amount = 0;
        chain_limit.hourly_transfer_amount[6].hour = 7;
        chain_limit.hourly_transfer_amount[7].amount = 0;
        chain_limit.hourly_transfer_amount[7].hour = 8;
        chain_limit.hourly_transfer_amount[8].amount = 800;
        chain_limit.hourly_transfer_amount[8].hour = 9;
        chain_limit.hourly_transfer_amount[9].amount = 900;
        chain_limit.hourly_transfer_amount[9].hour = 10;
        chain_limit.hourly_transfer_amount[10].amount = 1000;
        chain_limit.hourly_transfer_amount[10].hour = 11;
        chain_limit.hourly_transfer_amount[11].amount = 1100;
        chain_limit.hourly_transfer_amount[11].hour = 12;

        let window_limit = chain_limit.calculate_window_limit_internal(15);
        println!("window_limit:{}",window_limit);
        assert_eq!(window_limit,100+200+300+400+500+800+900+1000+1100);
        assert_eq!(window_limit,chain_limit.get_total_transfer_amount());
    }


    #[test] 
    fn test_set_total_limit(){
        let mut chain_limit = ChainLimit::default();
        chain_limit.set_total_limit(1000);
        let total_limit = chain_limit.total_limit;
        assert_eq!(total_limit, 1000);
    }

    #[test]
    fn test_set_single_transfer_limit(){
        let mut chain_limit = ChainLimit::default();
        chain_limit.set_single_transfer_limit(1000);
        let signle_transfer_limit=chain_limit.max_usd_limit;
        assert_eq!(signle_transfer_limit, 1000);
    }


    #[test]
    fn test_record_bridge_transfers_internal(){
        let mut chain_limit = ChainLimit::default();
        let amount=1000*USD_PRECISION;
        let price=1*USD_PRECISION;
        let decimal=8;
        let total_limit=10000*USD_PRECISION;
        chain_limit.set_single_transfer_limit(amount);
        chain_limit.set_total_limit(total_limit);

        let result = chain_limit.record_bridge_transfers_internal(1, amount, price, decimal);
        assert_eq!(result, Ok(()));
        let result = chain_limit.record_bridge_transfers_internal(2, amount, price, decimal);
        assert_eq!(result, Ok(()));
        let result = chain_limit.record_bridge_transfers_internal(3, amount, price, decimal);
        assert_eq!(result, Ok(()));
        let result = chain_limit.record_bridge_transfers_internal(4, amount, price, decimal);
        assert_eq!(result, Ok(()));     
        let result = chain_limit.record_bridge_transfers_internal(5, amount, price, decimal);
        assert_eq!(result, Ok(()));
        let result = chain_limit.record_bridge_transfers_internal(6, amount, price, decimal);
        assert_eq!(result, Ok(()));
        let result = chain_limit.record_bridge_transfers_internal(7, amount, price, decimal);
        assert_eq!(result, Ok(()));
        let result = chain_limit.record_bridge_transfers_internal(8, amount, price, decimal);
        assert_eq!(result, Ok(()));
        let result = chain_limit.record_bridge_transfers_internal(9, amount, price, decimal);
        assert_eq!(result, Ok(()));
        let result = chain_limit.record_bridge_transfers_internal(10, amount, price, decimal);
        assert_eq!(result, Ok(()));
        //Exceed window limit
        let result = chain_limit.record_bridge_transfers_internal(11, 1, price, decimal);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string().contains("Exceed window limit"), true);


        let result = chain_limit.record_bridge_transfers_internal(25, amount, price, decimal);
        assert_eq!(result, Ok(()));
        let result = chain_limit.record_bridge_transfers_internal(25, 1, price, decimal);
        assert_eq!(result.unwrap_err().to_string().contains("Exceed window limit"), true);
    }
}

