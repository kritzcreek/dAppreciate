import Array "mo:base/Array";
import Cycles "mo:base/ExperimentalCycles";

actor {

    type Cycles = Nat;
    var donationLog : [(?Principal, Cycles)] = [];

    public shared func balance() : async Nat {
        return Cycles.balance();
    };

    public shared func accept_cycles(sender: ?Principal) {
        let accepted : Nat = Cycles.accept(Cycles.available());
        donationLog := Array.append(donationLog, [ (sender, accepted) ]);
    };
};
