#!ic-repl -r http://localhost:8000

// To run these tests locally:
// * add ic-repl to you PATH, it can be downloaded from here: https://github.com/chenyan2002/ic-repl/releases
// * deploy the canisters needed for this tests e.g. using ./deploy.sh
// * adapt the canister IDs defined below to the ones that were deployed
// * then run % ic-repl tests.sh

import IndexCanister = "q3fc5-haaaa-aaaaa-aaahq-cai";
let client_canister_id = principal "qhbym-qaaaa-aaaaa-aaafq-cai";
let donation_receiver = principal "fjm4k-j73tc-uhkc4-3hhrk-qi4hk-swun5-4xxzd-enw73-p4sm7-q2or5-nqe";

identity Alice;

// should successfully register a client
call IndexCanister.register_client(record
          { client_canister_id = client_canister_id });
assert _ != (0 : nat);

// should successfully donate
call IndexCanister.donate(record
          { receiver = donation_receiver;
            beneficiaries = vec {}
          }
);
assert _ != (null : opt null);