const demoButton = document.getElementById("demo-button");
const demoMessage = document.getElementById("demo-message");

demoButton.addEventListener("click", () => {
  demoMessage.textContent = "Welcome to the demo! Your sample project is ready to explore below.";
  demoButton.textContent = "Demo opened ✓";
  document.getElementById("preview").scrollIntoView({ block: "center" });
});
