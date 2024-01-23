module bfc_crosschain::bfc_risk_control{

    use sui::tx_context::TxContext;

    struct risk_control_rule{
        total_amount_in_24_hours: u64,
        single_max_amount: u64,
    }


    public(friend) fun verify_risk_control_rule(ctx: &mut TxContext): bool{


        true
    }

}