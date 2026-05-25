const pet = document.querySelector(".pet");
const petImg = document.getElementById("pet-image");
const input = document.getElementById("input");
const submit = document.getElementById("submit");
const response = document.getElementById("response");

const setupScreen = document.getElementById("setup-screen");
const saveConfigBtn = document.getElementById("save-config");
const setupStatus = document.getElementById("setup-status");
const setupName = document.getElementById("setup-name");
const setupPetName = document.getElementById("setup-pet-name");
const setupPetType = document.getElementById("setup-pet-type");
const setupApiKey = document.getElementById("setup-api-key");
const setupPassword = document.getElementById("setup-password");

const loginModal = document.getElementById("login-modal");
const loginBtn = document.getElementById("login-btn");
const loginPasswordInput = document.getElementById("login-password");
const loginStatus = document.getElementById("login-status");
const app = document.getElementById("app");

let resetTimer = null;

function showSetup() {
  setupScreen.style.display = "flex";
  loginModal.style.display = "none";
  app.style.display = "none";
  setupName?.focus();
}

function showLogin(message = "") {
  setupScreen.style.display = "none";
  loginModal.style.display = "flex";
  app.style.display = "none";
  loginStatus.textContent = message;
  loginPasswordInput.value = "";
  loginPasswordInput?.focus();
}

function showApp() {
  setupScreen.style.display = "none";
  loginModal.style.display = "none";
  app.style.display = "flex";
  input?.focus();
}

async function bootstrap() {
  try {
    const exists = await window.__TAURI__.core.invoke("config_exists");
    if (exists) {
      showLogin();
    } else {
      showSetup();
    }
  } catch (error) {
    showSetup();
    setupStatus.textContent = `Error: ${error}`;
  }
}

async function saveSetup() {
  const payload = {
    name: setupName.value.trim(),
    petName: setupPetName.value.trim(),
    petType: setupPetType.value.trim(),
    apiKey: setupApiKey.value.trim(),
    password: setupPassword.value.trim()
  };

  if (!payload.name || !payload.petName || !payload.petType || !payload.apiKey || !payload.password) {
    setupStatus.textContent = "Fill all setup fields first.";
    return;
  }

  saveConfigBtn.disabled = true;
  setupStatus.textContent = "Saving...";

  try {
    await window.__TAURI__.core.invoke("save_config", payload);
    showLogin("Setup saved. Login now.");
  } catch (error) {
    setupStatus.textContent = `Error: ${error}`;
  } finally {
    saveConfigBtn.disabled = false;
  }
}

async function login() {
  const password = loginPasswordInput?.value.trim();
  if (!password) {
    loginStatus.textContent = "Enter your password first.";
    return;
  }

  loginBtn.disabled = true;
  loginStatus.textContent = "Checking...";

  try {
    const ok = await window.__TAURI__.core.invoke("check_password", { password });
    if (ok) {
      showApp();
    } else {
      loginStatus.textContent = "Wrong password";
    }
  } catch (error) {
    loginStatus.textContent = `Error: ${error}`;
  } finally {
    loginBtn.disabled = false;
  }
}

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
    if (answer === "CLOSE_APP") {
      await window.__TAURI__.core.invoke("close_app");
      return;
    }
    response.textContent = answer;
    input.value = "";
  } catch (error) {
    response.textContent = `AI error: ${error}`;
  } finally {
    submit.disabled = false;
  }
}

saveConfigBtn?.addEventListener("click", saveSetup);
setupPassword?.addEventListener("keydown", (event) => {
  if (event.key === "Enter") {
    event.preventDefault();
    saveSetup();
  }
});

loginBtn?.addEventListener("click", login);
loginPasswordInput?.addEventListener("keydown", (event) => {
  if (event.key === "Enter") {
    event.preventDefault();
    login();
  }
});

pet?.addEventListener("click", waveOnce);
submit?.addEventListener("click", sendPrompt);
input?.addEventListener("keydown", (event) => {
  if (event.key === "Enter") {
    event.preventDefault();
    sendPrompt();
  }
});

bootstrap();
