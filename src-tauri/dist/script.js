/*
  This JavaScript file handles the Tauri desktop app.
  
  
  Enjoy!
*/

// get DOM elements
const pet = document.querySelector(".pet"); 
const petImg = document.getElementById("pet-image");
const input = document.getElementById("input");
const submit = document.getElementById("submit");
const response = document.getElementById("response");

const loginModal = document.getElementById("login-modal");
const loginBtn = document.getElementById("login-btn");
const passwordInput = document.getElementById("password");
const loginStatus = document.getElementById("login-status");
const app = document.getElementById("app");

if (loginModal) {
  loginModal.style.display = "flex";
}


// password
loginBtn?.addEventListener("click", async () => {
  const password = passwordInput.value;

  try {
    const ok = await window.__TAURI__.core.invoke(
      "check_password",
      { password }
    );

    if (ok) {
      loginModal.style.display = "none";
      app.style.display = "block";
    } else {
      loginStatus.textContent = "Wrong password";
    }
  } catch (err) {
    loginStatus.textContent = `Error: ${err}`;
  }
});


// timer for resetting pet animation
let resetTimer = null;

// wave pet func
function waveOnce() {
  if (!petImg) {
    return;
  }

  petImg.src = "assets/moxi-waving.gif";

  if (resetTimer) {
    clearTimeout(resetTimer);
  }

  resetTimer = setTimeout(() => {
    petImg.src = "assets/moxi-idle.gif";
  }, 1000);
}

// pet click event
pet?.addEventListener("click", waveOnce);

// send prompt function
async function sendPrompt() {
  const prompt = input?.value.trim();
  if (!prompt) {
    response.textContent = "Type something first.";
    return;
  }

  response.textContent = "Thinking...";
  submit.disabled = true;

  try {
    const answer = await window.__TAURI__.core.invoke("ask_hackclub_ai", { prompt });
    response.textContent = answer;
  } catch (error) {
    response.textContent = `AI error: ${error}`;
  } finally {
    submit.disabled = false;
  }
}

// submit button event
submit?.addEventListener("click", sendPrompt);

// input keydown event
input?.addEventListener("keydown", (event) => {
  if (event.key === "Enter") {
    event.preventDefault();
    sendPrompt();
  }
});




// end of file
