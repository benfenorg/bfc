module bfc_crosschain::bfc_valut_eventcenter{

    use std::string;
    use sui::event;

    friend bfc_crosschain::bfc_vault;

    //
    struct Bfc_vault_event has copy, drop, store {
        name: string::String,
    }



    public(friend) fun sendEvent(eventContent:vector<u8>) {
        event::emit(
            Bfc_vault_event{
                name: string::utf8(eventContent)
            }
        );
    }

}