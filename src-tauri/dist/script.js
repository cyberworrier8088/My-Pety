const pet = document.querySelector(".pet");
const petSprite = document.getElementById("pet-sprite");
const input = document.getElementById("input");
const submit = document.getElementById("submit");
const response = document.getElementById("response");

const setupScreen = document.getElementById("setup-screen");
const saveConfigBtn = document.getElementById("save-config");
const setupStatus = document.getElementById("setup-status");
const setupName = document.getElementById("setup-name");
const setupPetName = document.getElementById("setup-pet-name");
const setupPetType = document.getElementById("setup-pet-type");
const setupPetAsset = document.getElementById("setup-pet-asset");
const setupApiKey = document.getElementById("setup-api-key");
const setupPassword = document.getElementById("setup-password");

const loginModal = document.getElementById("login-modal");
const loginBtn = document.getElementById("login-btn");
const loginPasswordInput = document.getElementById("login-password");
const loginStatus = document.getElementById("login-status");
const app = document.getElementById("app");
const topDragZone = document.querySelector(".top-drag-zone");



let petAnimationTimer = null;
let petAnimationName = "idle";
let petFrameIndex = 0;
let petPointerStart = null;
let petPointerLastX = null;
let petStopTimer = null;
let petWindowStartPosition = null;
let petMoveFrame = null;
let petMoveTarget = null;
let petManualDragAvailable = true;
let petDidDrag = false;

const DRAG_THRESHOLD = 6;
const DRAG_DIRECTION_THRESHOLD = 2;
const DRAG_STOP_DELAY = 180;
const MIN_PASSWORD_LENGTH = 8;
const PET_ASSETS = new Set(["ferris", "dog", "cat"]);
const DEFAULT_PET_ASSET = "ferris";
let currentPetAsset = DEFAULT_PET_ASSET;
const PET_COLUMNS = 8;
const PET_ROWS = 9;
// Atlas rows here are 1-based to match the visible spritesheet rows.
const PET_ANIMATIONS = {
  idle: {
    row: 1,
    durations: [280, 110, 110, 140, 140, 320]
  },
  runningRight: {
    row: 2,
    durations: [120, 120, 120, 120, 120, 120, 120, 220]
  },
  runningLeft: {
    row: 3,
    durations: [120, 120, 120, 120, 120, 120, 120, 220]
  },
  waving: {
    row: 4,
    durations: [140, 140, 140, 280]
  },
  failed: {
    row: 6,
    durations: [140, 140, 140, 140, 140, 140, 140, 240]
  },
  waiting: {
    row: 7,
    durations: [150, 150, 150, 150, 150, 260]
  }
};

function formatTauriError(error) {
  if (typeof error === "string") {
    return error;
  }

  if (error && typeof error.message === "string") {
    return error.message;
  }

  return String(error);
}

function showSetup() {
  setupScreen.style.display = "flex";
  loginModal.style.display = "none";
  app.style.display = "none";
  setupStatus.textContent = "";
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
  loadPetManifest();
  input?.focus();
}

async function loadPetManifest(assetName = null) {
  if (!petSprite) {
    return;
  }

  const selectedAsset = assetName || await loadSelectedPetAsset();
  currentPetAsset = normalizePetAsset(selectedAsset);
  const petBasePath = `assets/${currentPetAsset}/`;

  try {
    const manifestResponse = await fetch(`${petBasePath}pet.json`);
    const petManifest = await manifestResponse.json();

    if (petManifest.spritesheetPath) {
      petSprite.style.backgroundImage =
        `url("${petBasePath}${petManifest.spritesheetPath}")`;
    }
  } catch (error) {
    console.warn("Could not load pet manifest:", error);
    petSprite.style.backgroundImage = `url("${petBasePath}spritesheet.webp")`;
  }

  playPetAnimation("idle");
}

async function loadSelectedPetAsset() {
  try {
    return await window.__TAURI__.core.invoke("load_pet_asset");
  } catch (error) {
    console.warn("Could not load saved pet asset:", formatTauriError(error));
    return DEFAULT_PET_ASSET;
  }
}

function normalizePetAsset(asset) {
  const value = String(asset || "").trim().toLowerCase();
  return PET_ASSETS.has(value) ? value : DEFAULT_PET_ASSET;
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
    petAsset: normalizePetAsset(setupPetAsset?.value),
    apiKey: setupApiKey.value.trim(),
    password: setupPassword.value.trim()
  };

  if (!payload.name || !payload.petName || !payload.petType || !payload.apiKey || !payload.password) {
    setupStatus.textContent = "Fill all setup fields first.";
    return;
  }

  if (payload.password.length < MIN_PASSWORD_LENGTH) {
    setupStatus.textContent = "Password must be at least 8 characters long.";
    setupPassword.focus();
    return;
  }

  saveConfigBtn.disabled = true;
  setupStatus.textContent = "Saving...";

  try {
    await window.__TAURI__.core.invoke("save_config", payload);
    loadPetManifest(payload.petAsset);
    showLogin("Setup saved. Login now.");
    setupPassword.value = "";
  } catch (error) {
    setupStatus.textContent = formatTauriError(error);
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
      loginPasswordInput.select();
    }
  } catch (error) {
    loginStatus.textContent = formatTauriError(error);
  } finally {
    loginBtn.disabled = false;
  }
}

function waveOnce() {
  playPetAnimation("waving", { loop: false, next: "idle" });
}

function showPetFailure() {
  playPetAnimation("failed", { loop: false, next: "idle" });
}

function showPetWaiting() {
  playPetAnimation("waiting");
}

function playPetAnimation(name, options = {}) {
  const animation = PET_ANIMATIONS[name] || PET_ANIMATIONS.idle;
  const loop = options.loop ?? true;
  const next = options.next || "idle";

  if (!petSprite) {
    return;
  }

  if (petAnimationTimer) {
    clearTimeout(petAnimationTimer);
  }

  petAnimationName = name;
  petFrameIndex = 0;

  const step = () => {
    const column = petFrameIndex;
    setPetFrame(animation.row, column);

    const duration = animation.durations[petFrameIndex];
    petFrameIndex += 1;

    if (petFrameIndex >= animation.durations.length) {
      if (!loop) {
        petAnimationTimer = setTimeout(() => {
          playPetAnimation(next);
        }, duration);
        return;
      }

      petFrameIndex = 0;
    }

    petAnimationTimer = setTimeout(step, duration);
  };

  step();
}

function setPetFrame(row, column) {
  if (!petSprite) {
    return;
  }

  const rowIndex = Math.max(0, Math.min(PET_ROWS - 1, row - 1));
  const columnIndex = Math.max(0, Math.min(PET_COLUMNS - 1, column));
  const x = (columnIndex / (PET_COLUMNS - 1)) * 100;
  const y = (rowIndex / (PET_ROWS - 1)) * 100;
  petSprite.style.backgroundPosition = `${x}% ${y}%`;
}

async function startPetDrag() {
  const currentWindow = getCurrentTauriWindow();
  if (!currentWindow?.startDragging) {
    return;
  }

  petDidDrag = true;

  try {
    await currentWindow.startDragging();
  } catch (error) {
    console.error("Unable to drag window from pet:", error);
  }
}

function getCurrentTauriWindow() {
  return window.__TAURI__?.window?.getCurrentWindow?.();
}

function createPhysicalPosition(x, y) {
  const PhysicalPosition =
    window.__TAURI__?.dpi?.PhysicalPosition ||
    window.__TAURI__?.window?.PhysicalPosition;

  if (PhysicalPosition) {
    return new PhysicalPosition(x, y);
  }

  return { type: "Physical", x, y };
}

async function preparePetWindowDrag() {
  const currentWindow = getCurrentTauriWindow();
  if (!currentWindow?.outerPosition || !currentWindow?.setPosition) {
    petManualDragAvailable = false;
    return;
  }

  petManualDragAvailable = true;

  try {
    const position = await currentWindow.outerPosition();
    if (petPointerStart) {
      petWindowStartPosition = {
        x: position.x,
        y: position.y
      };
    }
  } catch (error) {
    petManualDragAvailable = false;
    console.error("Unable to read window position for pet drag:", error);
  }
}

function queuePetWindowMove(deltaX, deltaY) {
  const currentWindow = getCurrentTauriWindow();
  if (!petManualDragAvailable || !currentWindow?.setPosition) {
    return false;
  }

  if (!petWindowStartPosition) {
    return true;
  }

  petMoveTarget = {
    x: Math.round(petWindowStartPosition.x + deltaX),
    y: Math.round(petWindowStartPosition.y + deltaY)
  };

  if (petMoveFrame) {
    return true;
  }

  petMoveFrame = requestAnimationFrame(async () => {
    const target = petMoveTarget;
    petMoveFrame = null;

    if (!target) {
      return;
    }

    try {
      await currentWindow.setPosition(
        createPhysicalPosition(target.x, target.y)
      );
    } catch (error) {
      petManualDragAvailable = false;
      console.error("Unable to move window from pet drag:", error);
    }
  });

  return true;
}

function showPetDragMovement(deltaX) {
  if (Math.abs(deltaX) < DRAG_DIRECTION_THRESHOLD) {
    return;
  }

  const nextAnimation = deltaX > 0 ? "runningRight" : "runningLeft";
  if (petAnimationName !== nextAnimation) {
    playPetAnimation(nextAnimation);
  }

  if (petStopTimer) {
    clearTimeout(petStopTimer);
  }

  petStopTimer = setTimeout(() => {
    if (petAnimationName === "runningRight" || petAnimationName === "runningLeft") {
      playPetAnimation("idle");
    }
  }, DRAG_STOP_DELAY);
}

async function startTopDrag(event) {
  if (event.button !== 0) {
    return;
  }

  const currentWindow = getCurrentTauriWindow();
  if (!currentWindow?.startDragging) {
    return;
  }

  try {
    await currentWindow.startDragging();
  } catch (error) {
    console.error("Unable to drag window from top area:", error);
  }
}

function handlePetPointerDown(event) {
  if (event.button !== 0) {
    return;
  }

  pet?.setPointerCapture?.(event.pointerId);
  petPointerStart = {
    x: event.screenX,
    y: event.screenY
  };
  petPointerLastX = event.screenX;
  petDidDrag = false;
  petWindowStartPosition = null;
  petMoveTarget = null;
  preparePetWindowDrag();
}

function handlePetPointerMove(event) {
  if (!petPointerStart) {
    return;
  }

  const rawDeltaX = event.screenX - petPointerStart.x;
  const rawDeltaY = event.screenY - petPointerStart.y;
  const movementDeltaX = event.screenX - (petPointerLastX ?? event.screenX);
  const deltaX = Math.abs(rawDeltaX);
  const deltaY = Math.abs(rawDeltaY);

  if (deltaX < DRAG_THRESHOLD && deltaY < DRAG_THRESHOLD) {
    return;
  }

  showPetDragMovement(movementDeltaX || rawDeltaX);
  petPointerLastX = event.screenX;

  const manualMoveStarted = queuePetWindowMove(rawDeltaX, rawDeltaY);

  if (!petDidDrag) {
    petDidDrag = true;
    if (!manualMoveStarted) {
      startPetDrag();
    }
  }
}

function resetPetPointerState(event) {
  if (event?.type === "pointerleave" && petDidDrag) {
    return;
  }

  if (event?.pointerId !== undefined) {
    try {
      if (pet?.hasPointerCapture?.(event.pointerId)) {
        pet.releasePointerCapture(event.pointerId);
      }
    } catch (error) {
      console.warn("Could not release pet pointer capture:", error);
    }
  }

  petPointerStart = null;
  petPointerLastX = null;
  petWindowStartPosition = null;
  petMoveTarget = null;
  petManualDragAvailable = true;

  if (petMoveFrame) {
    cancelAnimationFrame(petMoveFrame);
    petMoveFrame = null;
  }

  if (petStopTimer) {
    clearTimeout(petStopTimer);
    petStopTimer = null;
  }

  if (petAnimationName === "runningRight" || petAnimationName === "runningLeft") {
    playPetAnimation("idle");
  }
}

function handlePetClick() {
  if (petDidDrag) {
    petDidDrag = false;
    return;
  }

  waveOnce();
}

async function sendPrompt() {
  const prompt = input?.value.trim();
  if (!prompt) {
    response.textContent = "Type something first.";
    return;
  }

  response.textContent = "Thinking...";
  submit.disabled = true;
  showPetWaiting();

  try {
    const answer = await window.__TAURI__.core.invoke("ask_hackclub_ai", { prompt });
    if (answer === "CLOSE_APP") {
      await window.__TAURI__.core.invoke("close_app");
      return;
    }
    response.textContent = answer;
    input.value = "";
  } catch (error) {
    showPetFailure();
    response.textContent = `AI error: ${formatTauriError(error)}`;
  } finally {
    if (petAnimationName === "waiting") {
      playPetAnimation("idle");
    }
    submit.disabled = false;
  }
}

async function saveCurrentLocation() {
  if (!navigator.geolocation) {
    console.warn("Geolocation is not supported in this app.");
    return;
  }

  navigator.geolocation.getCurrentPosition(
    async (position) => {
      const lat = position.coords.latitude;
      const lon = position.coords.longitude;

      try {
        await window.__TAURI__.core.invoke("save_location", { lat, lon });
        console.log("Location saved", lat, lon);
      } catch (error) {
        console.error("Could not save location:", formatTauriError(error));
      }
    },
    (error) => {
      console.warn("Location permission denied or unavailable:", error?.message || error);
    },
    {
      enableHighAccuracy: false,
      timeout: 10000,
      maximumAge: 300000
    }
  );
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

topDragZone?.addEventListener("mousedown", startTopDrag);
pet?.addEventListener("pointerdown", handlePetPointerDown);
pet?.addEventListener("pointermove", handlePetPointerMove);
pet?.addEventListener("pointerup", resetPetPointerState);
pet?.addEventListener("pointercancel", resetPetPointerState);
pet?.addEventListener("pointerleave", resetPetPointerState);
pet?.addEventListener("click", handlePetClick);
document.addEventListener("pointerup", resetPetPointerState);
document.addEventListener("pointercancel", resetPetPointerState);
window.addEventListener("blur", resetPetPointerState);
submit?.addEventListener("click", sendPrompt);
input?.addEventListener("keydown", (event) => {
  if (event.key === "Enter") {
    event.preventDefault();
    sendPrompt();
  }
});

saveCurrentLocation();
loadPetManifest();
bootstrap();
