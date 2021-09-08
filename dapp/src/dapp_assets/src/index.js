import { dapp } from "../../declarations/dapp";

const index_canister_url = process.env.INDEX_CANISTER_URL;
const canister_id = process.env.DAPP_CANISTER_ID;

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  // Interact with dapp actor, calling the greet method
  const balance = await dapp.balance();

  document.getElementById("balance").innerText = balance.toString();
});

document.getElementById("donateButton").addEventListener("click", async () => {
  // Open a popup with the dAppreciate index canister
  window.open(index_canister_url + "?receiver=" + canister_id, "dAppreciate Index", "");
});
