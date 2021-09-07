var candid = require("generated/api.js");
var dfnAgent = require("@dfinity/agent");

const canisterId = process.env.CLIENT_CANISTER_ID;

const agent = new dfnAgent.HttpAgent();

// Fetch root key for certificate validation during development
if (process.env.NODE_ENV !== "production") {
    agent.fetchRootKey().catch(err => {
        console.warn("Unable to fetch root key. Check to ensure that your local replica is running");
        console.error(err);
    });
}

// Creates an actor with using the candid interface and the HttpAgent
const client = dfnAgent.Actor.createActor(candid.idlFactory, {
    agent,
    canisterId
});

exports.toFixed = decimals => n => n.toFixed(decimals);

exports.listDonationsImpl = (onError, onSuccess) => {
    client.list_donations()
        .then(res => {
            console.log(res); onSuccess(res)
        })
        .catch(onError);

    return (cancelError, onCancelerError, onCancelerSuccess) => onCancelerSuccess();
};
