import { dapp } from "../../declarations/dapp";

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  // Interact with dapp actor, calling the greet method
  const balance = await dapp.balance();

  document.getElementById("balance").innerText = balance.toString();
});
