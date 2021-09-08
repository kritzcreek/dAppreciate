#!ic-repl -r http://localhost:8000

// To run these tests locally:
// * add ic-repl to you PATH, it can be downloaded from here: https://github.com/chenyan2002/ic-repl/releases
// * deploy the canisters needed for this tests e.g. using ./deploy.sh
// * adapt the canister IDs defined below to the ones that were deployed
// * then run % ic-repl tests.sh

import IndexCanister = "t6rzw-2iaaa-aaaaa-aaama-cai";
import ClientCanister = "txssk-maaaa-aaaaa-aaanq-cai";
let client_canister_id = principal "txssk-maaaa-aaaaa-aaanq-cai";
let donation_receiver_0 = principal "fjm4k-j73tc-uhkc4-3hhrk-qi4hk-swun5-4xxzd-enw73-p4sm7-q2or5-nqe";
let donation_receiver_1 = principal "ezyem-v2qbz-rlp6h-6pors-n7rq7-gnkzd-xbfes-vxwsu-22uig-4aeyo-pqe";

identity Alice;

// should successfully register a client
call IndexCanister.register_client(record
          { client_canister_id = client_canister_id });
assert _ != (0 : nat);

// should successfully query the current client
call IndexCanister.current_client();
assert _ == opt record { client_canister_id = client_canister_id };

// should successfully make donation to donation_receiver_0
call IndexCanister.donate(record
          { receiver = donation_receiver_0;
            beneficiaries = vec {}
          }
);
assert _ != (null : opt null);

// should successfully make dontation to donation_receiver_1
call IndexCanister.donate(record
          { receiver = donation_receiver_1;
            beneficiaries = vec {}
          }
);
assert _ != (null : opt null);

// should see the donations in the client canister
let donations = call ClientCanister.list_donations();
assert donations.pending[0].receiver == donation_receiver_1;
assert donations.pending[1].receiver == donation_receiver_0;

// should see the donations in the client canister
call ClientCanister.approve_donations();
assert _ != (null : opt null);