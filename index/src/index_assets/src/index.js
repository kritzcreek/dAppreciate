import { Principal } from "@dfinity/principal";
import { index } from "../../declarations/index";

document.getElementById("dAppr").addEventListener("click", async () => {
  const name = document.getElementById("name").value.toString();
  const canisterId = document.getElementById("canisterId").value.toString();
  const principal = Principal.fromText(canisterId)

  await index.forwardDAppr(
    { name, canisterId: principal }, principal
  );
});
