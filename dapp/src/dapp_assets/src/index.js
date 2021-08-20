import { dapp } from "../../declarations/dapp";

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const name = document.getElementById("name").value.toString();
  // Interact with dapp actor, calling the greet method
  const greeting = await dapp.greet(name);

  document.getElementById("greeting").innerText = greeting;
});
