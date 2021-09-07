// To run these tests locally:
// * add ic-repl to you PATH, it can be downloaded from here: https://github.com/chenyan2002/ic-repl/releases
// * deploy the canisters needed for this tests e.g. using ./deploy.sh
// * adapt the canister IDs defined below to the ones that were deployed
// * then run % ic-repl tests.sh

#!ic-repl -r http://localhost:8000

import IndexCanister = "q3fc5-haaaa-aaaaa-aaahq-cai";

// should successfully register a client
call IndexCanister.register_client(record
          { client_canister_id = principal "fjm4k-j73tc-uhkc4-3hhrk-qi4hk-swun5-4xxzd-enw73-p4sm7-q2or5-nqe" });
assert _ ~= (1 : nat);