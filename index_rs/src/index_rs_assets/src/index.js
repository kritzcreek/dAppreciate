import { index_rs } from "../../declarations/index_rs";

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const name = document.getElementById("name").value.toString();
  // Interact with index_rs actor, calling the greet method
  const greeting = await index_rs.greet(name);

  document.getElementById("greeting").innerText = greeting;
});
