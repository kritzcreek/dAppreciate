var candid = require("generated/api.js");
var dfnAgent = require("@dfinity/agent");

const canisterId = process.env.CLIENT_RS_CANISTER_ID;

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
exports.trillion = 1_000_000_000_000n;

exports.listDonationsImpl = (onError, onSuccess) => {
    client.list_donations()
        .then(res => {
            console.log(res); onSuccess(res)
        })
        .catch(onError);

    return (cancelError, onCancelerError, onCancelerSuccess) => onCancelerSuccess();
};

exports.approveDonationsImpl = (onError, onSuccess) => {
    client.approve_donations()
        .then(_ => {
            onSuccess()
        })
        .catch(onError);

    return (cancelError, onCancelerError, onCancelerSuccess) => onCancelerSuccess();
};

exports.setDonationAmountImpl = amount => (onError, onSuccess) => {
    client.set_donation_amount(amount)
        .then(_ => {
            onSuccess()
        })
        .catch(onError);

    return (cancelError, onCancelerError, onCancelerSuccess) => onCancelerSuccess();
};
