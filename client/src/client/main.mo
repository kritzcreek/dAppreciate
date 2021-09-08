import Array "mo:base/Array";
import Cycles "mo:base/ExperimentalCycles";
import Debug "mo:base/Debug";
import Hash "mo:base/Hash";
import HashMap "mo:base/HashMap";
import Iter "mo:base/Iter";
import Principal "mo:base/Principal";

actor {
//actor class (indexCanister : Principal) {

    var receivers : [DonationReceiver] = [];
    var dailyBudget : Nat = 1_000_000_000;

    type Cycles = Nat;

    type DonationReceiver = {
        receiver : Donatee; 
        beneficiaries : [Donatee]; // additional receivers of a donation.
    };

    type PendingDonations = {
        pending : [PendingDonation]; 
        amount : DonationAmount;  // for the UI to display
        balance : Cycles; // for the UI to display
    };

    type PendingDonation = {
        // the receiver of the donation
        receiver : DonationReceiver;
        // the number of button clicks for this receiver
        count : Nat; 
    };

    type DonationAmount = {
        // the amount of cycles to be distributed in the pending donations. 
        amount : Nat; 
    };

    type Donatee = actor {
        accept_cycles : () -> async ()
    };

    public shared (msg) func donate(receiver : DonationReceiver) {
        // TODO Also allow owner
        // if(msg.caller != indexCanister) {
        //     return
        // };
        receivers := Array.append(receivers, [receiver]);
    };

    public shared (msg) func set_donation_amount(amount : Cycles) {
        dailyBudget := amount;
    };

    func donateeEq(d1 : Donatee, d2 : Donatee) : Bool = Principal.equal(Principal.fromActor(d1), Principal.fromActor(d2));
    func donateeHash(d : Donatee) : Hash.Hash = Principal.hash(Principal.fromActor(d));

    func pending_donations() : PendingDonations {
        let countMap : HashMap.HashMap<Donatee, Nat> = HashMap.HashMap(16, donateeEq, donateeHash);

        for ({ receiver } in receivers.vals()) {
            switch (countMap.get(receiver)) {
                case null {
                    countMap.put(receiver, 1);
                };
                case (?old) {
                    countMap.put(receiver, old + 1);
                };
            };
        };

        let pending: [PendingDonation] = Array.map(Iter.toArray(countMap.entries()), func ((principal : Donatee, count : Nat)) : PendingDonation {
            { receiver = { receiver = principal; beneficiaries = [] }; count }
        });

        {
            amount = { amount = dailyBudget };
            balance = Cycles.balance();
            pending;
        }
    };

    public query (msg) func list_donations() : async PendingDonations {
        pending_donations()
    };

    // TODO: Properly weigh the different receivers
    public shared func approve_donations() : async () {
        for (donatee in pending_donations().pending.vals()) {
            Cycles.add(1_000_000_000_000);
            try { 
                await donatee.receiver.receiver.accept_cycles() 
            } catch (_) {
                Debug.print("Failed to send cycles to: " # Principal.toText(Principal.fromActor(donatee.receiver.receiver)));
            };
        };
    }
};


/*

type DonationReceiver = record {
  receiver: principal; 
  beneficiaries : vec principal; // additional receivers of a donation.
};

type PendingDonations = record {
  pending: vec PendingDonation; 
  amount: DonationAmount;  // for the UI to display
};

type PendingDonation = record {
  // the receiver of the donation
  receiver: DonationReceiver;
  // the number of button clicks for this receiver
  count: nat; 
};

type DonationAmount = record {
  // the amount of cycles to be distributed in the pending donations. 
  amount: nat; 
};

service : {

  // Registers the caller's self-authenticating principal as the unique user of 
  // this client. Should be called only once to set up this client. 
  // 
  // Authentication: the call must be made using a self-authenticating principal
  // 
  // Traps if the caller is anonymous or if a user has already been registered. 
  register : () -> (); 

  // Add a donation to the `DonationReceiver` to the list of pending donations.  
  //
  // Authentication: The caller must be the index canister 
  // or the hardcoded self-authenticating principal
  // 
  // Traps if the caller is not the index canister. 
  donate : (DonationReceiver) -> ();

  // Sets the total donation amount that will be distributed to PendingDonations
  // upon `approve_donations`. 
  // 
  // If this method has never beencalled, the DonationAmount is set to the default of
  // value of ???.??? cycles. 
  // 
  // Authentication: the call must be made using a the self-authenticating 
  // principal that was used for `register`. 
  // 
  // Traps if the caller is not the registered one, also if it is anonymous. 
  set_donation_amount : (DonationAmount) -> ();  

  // Return the list of pending donations and the currently set `DonationAmount`. 
  // 
  // Authentication: the call must be made using a the self-authenticating 
  // principal that was used for `register`. 
  // 
  // Traps if the caller is not the registered one, also if it is anonymous. 
  list_donations : () -> (PendingDonations) query; 

  // Pay out the pending donations. This will distribute the DonationAmount of 
  // cycles set using `set_donation_amount` according to `PendingDonations`. 
  // The amount of paid out cylces are weighted according to the counts in 
  // `PendingDonation`. 
  // 
  // TODO: maybe return an error if the client canister does not have enough 
  // cycles? 
  // 
  // Authentication: the call must be made using a the self-authenticating 
  // principal that was used for `register`. 
  // 
  // Traps if the caller is not the registered one, also if it is anonymous. 
  approve_donations : () -> (); 
}

*/
