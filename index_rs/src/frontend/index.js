import { index_rs } from "../declarations/index_rs";
import { AuthClient } from "@dfinity/auth-client";
import { canisterId, createActor } from "../declarations/index_rs";
import {index} from "../../../index/.dfx/local/canisters/index";
import { Principal } from "@dfinity/principal";

const $ = (s) => document.querySelector(s);

run();

async function run() {
    const authClient = await AuthClient.create();
    if (await authClient.isAuthenticated()) {
        handleAuthenticated(authClient);
    }
    $("#loginButton").onclick = async () => {
        await authClient.login({
            onSuccess: async () => {
                handleAuthenticated(authClient);
            },
            identityProvider:
                process.env.NODE_ENV === "development"
                    ? process.env.LOCAL_II_CANISTER
                    : "https://identity.ic0.app/#authorize",
        });
    }
}

async function handleAuthenticated(authClient) {
    const identity = await authClient.getIdentity();
    const indexCanister = createActor(canisterId, {
        agentOptions: {
            identity,
        },
    });
    await renderPage(indexCanister);

    $("#registerClientButton").onclick = async () => {
        console.log("registerClientInput: " + $("#registerClientInput").value);
        await indexCanister.register_client({
            client_canister_id: Principal.fromText($("#registerClientInput").value),
        });
        await renderPage(indexCanister);
    }

    // parse query parameter of the form
    // ?receiver=rrkah-asdads-asdasd&beneficiaries=asdlasd-asdasd,asdlkjasd,adslads
    const searchParams = new URLSearchParams(window.location.search)
    const receiver = searchParams.get("receiver");
    const beneficiaries = searchParams.get("beneficiaries")?.split(",") ?? [];
    $("#receiver").innerText = receiver;
    $("#beneficiaries").innerText = beneficiaries?.join(", ");

    $("#approveButton").onclick = async () => {
        await indexCanister.donate({
            receiver: Principal.fromText(receiver),
            beneficiaries: beneficiaries.map(b => Principal.fromText(b)),
        });
        alert("Successfully issued donation!");
        window.close();
    }
}

async function renderPage(indexCanister) {
    $("#loginSection").classList.add("hidden");
    let current_client = await indexCanister.current_client();
    if (current_client.length === 0) {
        $("#approveSection").classList.add("hidden");
        $("#registerSection").classList.remove("hidden");
    } else {
        $("#approveSection").classList.remove("hidden");
        $("#registerSection").classList.add("hidden");
    }
}

