const state = {
  view: "rooms",
  room: null,
  device: null,
};

const els = {
  roomsPanel: document.getElementById("rooms-panel"),
  roomPanel: document.getElementById("room-panel"),
  devicePanel: document.getElementById("device-panel"),
  roomsList: document.getElementById("rooms-list"),
  devicesList: document.getElementById("devices-list"),
  roomTitle: document.getElementById("room-title"),
  deviceTitle: document.getElementById("device-title"),
  deviceDetails: document.getElementById("device-details"),
  error: document.getElementById("error"),
  reportDialog: document.getElementById("report-dialog"),
  reportText: document.getElementById("report-text"),
  deviceType: document.getElementById("device-type"),
  socketFields: document.getElementById("socket-fields"),
  thermometerFields: document.getElementById("thermometer-fields"),
};

function showError(message) {
  els.error.hidden = !message;
  els.error.textContent = message || "";
}

async function api(path, options = {}) {
  const response = await fetch(`/api${path}`, {
    headers: { "Content-Type": "application/json", ...(options.headers || {}) },
    ...options,
  });

  if (response.status === 204) {
    return null;
  }

  const data = await response.json().catch(() => ({}));
  if (!response.ok) {
    throw new Error(data.error || `Ошибка ${response.status}`);
  }
  return data;
}

function renderView() {
  els.roomsPanel.classList.toggle("hidden", state.view !== "rooms");
  els.roomPanel.classList.toggle("hidden", state.view !== "room");
  els.devicePanel.classList.toggle("hidden", state.view !== "device");
}

async function loadRooms() {
  showError("");
  const rooms = await api("/rooms");
  els.roomsList.innerHTML = "";

  if (!rooms.length) {
    els.roomsList.innerHTML = '<li class="meta">Комнат пока нет</li>';
    return;
  }

  for (const room of rooms) {
    const li = document.createElement("li");
    const link = document.createElement("a");
    link.href = "#";
    link.textContent = room.name;
    link.addEventListener("click", (event) => {
      event.preventDefault();
      openRoom(room.name);
    });

    const meta = document.createElement("span");
    meta.className = "meta";
    meta.textContent = `устройств: ${room.device_names.length}`;

    const del = document.createElement("button");
    del.className = "btn ghost";
    del.type = "button";
    del.textContent = "Удалить";
    del.addEventListener("click", async () => {
      try {
        await api(`/rooms/${encodeURIComponent(room.name)}`, { method: "DELETE" });
        await loadRooms();
      } catch (err) {
        showError(err.message);
      }
    });

    const actions = document.createElement("div");
    actions.style.display = "flex";
    actions.style.gap = "0.75rem";
    actions.style.alignItems = "center";
    actions.append(meta, del);

    li.append(link, actions);
    els.roomsList.append(li);
  }
}

async function openRoom(name) {
  showError("");
  state.view = "room";
  state.room = name;
  state.device = null;
  els.roomTitle.textContent = name;
  renderView();
  await loadDevices();
}

async function loadDevices() {
  const devices = await api(`/rooms/${encodeURIComponent(state.room)}/devices`);
  els.devicesList.innerHTML = "";

  if (!devices.length) {
    els.devicesList.innerHTML = '<li class="meta">Устройств пока нет</li>';
    return;
  }

  for (const device of devices) {
    const li = document.createElement("li");
    const link = document.createElement("a");
    link.href = "#";
    link.textContent = device.name;
    link.addEventListener("click", (event) => {
      event.preventDefault();
      openDevice(device.name);
    });

    const meta = document.createElement("span");
    meta.className = "meta";
    meta.textContent =
      device.device_type === "socket" ? "розетка" : "термометр";

    li.append(link, meta);
    els.devicesList.append(li);
  }
}

async function openDevice(name) {
  showError("");
  state.view = "device";
  state.device = name;
  els.deviceTitle.textContent = name;
  renderView();

  const device = await api(
    `/rooms/${encodeURIComponent(state.room)}/devices/${encodeURIComponent(name)}`
  );

  if (device.device_type === "thermometer") {
    els.deviceDetails.textContent = [
      `Имя: ${device.name}`,
      "Тип: термометр",
      `Температура: ${device.temperature}°C`,
    ].join("\n");
  } else {
    els.deviceDetails.textContent = [
      `Имя: ${device.name}`,
      "Тип: розетка",
      `Состояние: ${device.enabled ? "включена" : "выключена"}`,
      `Текущая мощность: ${device.current_power} Вт`,
      `Номинальная мощность: ${device.power_when_on} Вт`,
    ].join("\n");
  }
}

function syncDeviceTypeFields() {
  const isSocket = els.deviceType.value === "socket";
  els.socketFields.classList.toggle("hidden", !isSocket);
  els.thermometerFields.classList.toggle("hidden", isSocket);
}

document.getElementById("add-room-form").addEventListener("submit", async (event) => {
  event.preventDefault();
  showError("");
  const name = document.getElementById("room-name").value.trim();
  try {
    await api("/rooms", {
      method: "POST",
      body: JSON.stringify({ name }),
    });
    event.target.reset();
    await loadRooms();
  } catch (err) {
    showError(err.message);
  }
});

document.getElementById("add-device-form").addEventListener("submit", async (event) => {
  event.preventDefault();
  showError("");
  const body = {
    name: document.getElementById("device-name").value.trim(),
    device_type: els.deviceType.value,
    temperature: Number(document.getElementById("temperature").value),
    power_when_on: Number(document.getElementById("power-when-on").value),
  };
  try {
    await api(`/rooms/${encodeURIComponent(state.room)}/devices`, {
      method: "POST",
      body: JSON.stringify(body),
    });
    event.target.reset();
    els.deviceType.value = "socket";
    syncDeviceTypeFields();
    await loadDevices();
  } catch (err) {
    showError(err.message);
  }
});

document.getElementById("back-btn").addEventListener("click", async () => {
  state.view = "rooms";
  state.room = null;
  renderView();
  await loadRooms();
});

document.getElementById("back-to-room-btn").addEventListener("click", async () => {
  await openRoom(state.room);
});

document.getElementById("delete-device-btn").addEventListener("click", async () => {
  showError("");
  try {
    await api(
      `/rooms/${encodeURIComponent(state.room)}/devices/${encodeURIComponent(state.device)}`,
      { method: "DELETE" }
    );
    await openRoom(state.room);
  } catch (err) {
    showError(err.message);
  }
});

document.getElementById("report-btn").addEventListener("click", async () => {
  showError("");
  try {
    const data = await api("/report");
    els.reportText.textContent = data.report || "Дом пуст";
    els.reportDialog.showModal();
  } catch (err) {
    showError(err.message);
  }
});

els.deviceType.addEventListener("change", syncDeviceTypeFields);

syncDeviceTypeFields();
renderView();
loadRooms().catch((err) => showError(err.message));
