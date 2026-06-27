const message = document.querySelector("#message");
const countButton = document.querySelector("#countButton");

let clicks = 0;

countButton.addEventListener("click", () => {
  clicks += 1;
  const noun = clicks === 1 ? "click" : "clicks";
  message.textContent = `The browser JavaScript is alive: ${clicks} ${noun} handled.`;
});

async function showHealth() {
  try {
    const response = await fetch("/api/health");
    const data = await response.json();
    console.log("Vikas.js health:", data);
  } catch (error) {
    console.error("Could not read Vikas.js health endpoint", error);
  }
}

showHealth();
