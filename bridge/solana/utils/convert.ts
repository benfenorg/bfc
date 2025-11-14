export function convertBenfenToSlpDecimal(benfenDecimal: number, slpDecimal: number, amount: bigint): bigint {
    if (amount <= BigInt(0)) {
        throw new Error('InsufficientAmount: Amount must be greater than 0');
    }

    if (benfenDecimal === slpDecimal) {
        return amount;
    }

    let adjustedAmount: bigint;

    if (slpDecimal > benfenDecimal) {
       
        const factor = BigInt(Math.pow(10, slpDecimal - benfenDecimal));
        adjustedAmount = amount * factor;
    
        if (adjustedAmount < amount) {
            throw new Error('AmountTooLarge: Multiplication overflow');
        }
    } else {
        const factor = BigInt(Math.pow(10, benfenDecimal - slpDecimal));
        adjustedAmount = amount / factor;
        
        if (adjustedAmount === BigInt(0)) {
            throw new Error('AmountTooSmall: Division result is zero');
        }
    }

    if (adjustedAmount <= BigInt(0)) {
        throw new Error('AmountTooSmall: Adjusted amount must be greater than 0');
    }

    return adjustedAmount;
}

