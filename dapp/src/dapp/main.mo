import Array "mo:base/Array";
import Cycles "mo:base/ExperimentalCycles";

actor {

    type Cycles = Nat;
    var donationLog : [(?Principal, Cycles)] = [];

    public func balance() : async Nat {
        return Cycles.balance();
    };

    public shared func accept_cycles(sender: ?Principal) {
        donationLog := Array.append(donationLog, [ (sender, Cycles.accept(Cycles.available())) ]);
    }
};
