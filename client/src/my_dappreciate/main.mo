import Array "mo:base/Array";
import Cycles "mo:base/ExperimentalCycles";
actor class (indexCanister : Principal) {

    var dApprs : [DAppr] = [];
    var dailyBudget : Nat = 1_000_000_000;

    type DAppr = {
        canisterId : Principal;
        name : Text;
    };

    type Today = {
        dailyBudget : Nat;
        balance : Nat;
        dApprs : [DAppr];
    };

    public func greet(name : Text) : async Text {
        return "Hello, " # name # "!";
    };

    public shared (msg) func acceptDapr(dappr : DAppr) {
        // TODO Also allow owner
        // if(msg.caller != indexCanister) {
        //     return
        // };
        dApprs := Array.append(dApprs, [dappr]);
    };

    public query (msg) func today() : async Today {
        {
            dailyBudget;
            balance = Cycles.balance();
            dApprs;
        }
    };
};
