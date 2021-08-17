actor {

    type DAppr = {
        canisterId : Principal;
        name : Text;
    };

    type DApprClient = actor { acceptDapr : (dappr : DAppr) -> async () };

    public shared (msg) func forwardDAppr(dAppr : DAppr, client : DApprClient) : async () {
        await client.acceptDapr(dAppr)
    };
};
