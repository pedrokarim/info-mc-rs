const form = document.getElementById("lookup-form");
const input = document.getElementById("identifier-input");
const serverOptions = document.getElementById("server-options");
const editionSelect = document.getElementById("edition-select");
const serverState = document.getElementById("server-state");
const playerState = document.getElementById("player-state");
const serverResult = document.getElementById("server-result");
const playerResult = document.getElementById("player-result");
const modeInputs = Array.from(document.querySelectorAll("input[name='mode']"));
const chips = Array.from(document.querySelectorAll(".chip"));

function getMode() {
  const checked = modeInputs.find((radio) => radio.checked);
  return checked ? checked.value : "server";
}

function setState(el, kind, text) {
  el.className = `state-tag ${kind}`;
  el.textContent = text;
}

function escapeHtml(text) {
  return String(text)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;");
}

function updateModeUi() {
  const mode = getMode();
  serverOptions.classList.toggle("hidden", mode !== "server");
  input.placeholder =
    mode === "server" ? "play.hypixel.net ou 172.65.128.35:25565" : "Notch ou UUID";
}

modeInputs.forEach((radio) => radio.addEventListener("change", updateModeUi));
updateModeUi();

chips.forEach((chip) => {
  chip.addEventListener("click", () => {
    const mode = chip.dataset.mode;
    const value = chip.dataset.value;
    if (!mode || !value) {
      return;
    }

    const targetRadio = modeInputs.find((radio) => radio.value === mode);
    if (targetRadio) {
      targetRadio.checked = true;
      updateModeUi();
    }
    input.value = value;
    form.requestSubmit();
  });
});

function renderServer(data) {
  const players = data.players ?? { online: 0, max: 0, sample: [] };
  const ratio = players.max > 0 ? Math.min(100, (players.online / players.max) * 100) : 0;
  const motd = data.motd?.clean ?? "Aucun MOTD";

  serverResult.classList.remove("empty");
  serverResult.innerHTML = `
    <dl class="kv-grid">
      <dt>Adresse</dt><dd>${escapeHtml(data.address.hostname)}:${data.address.port}</dd>
      <dt>IP</dt><dd>${escapeHtml(data.address.ip)}</dd>
      <dt>Edition</dt><dd>${escapeHtml(data.edition)}</dd>
      <dt>Version</dt><dd>${escapeHtml(data.version?.name ?? "N/A")}</dd>
      <dt>Latency</dt><dd>${data.latency_ms ?? "N/A"} ms</dd>
      <dt>SRV</dt><dd>${data.address.srv_record ? "oui" : "non"}</dd>
    </dl>
    <div class="motd-block">${escapeHtml(motd)}</div>
    <div>${players.online} / ${players.max} joueurs</div>
    <div class="player-bar"><span style="width:${ratio}%"></span></div>
    ${
      players.sample?.length
        ? `<p class="samples">Sample: ${players.sample
            .slice(0, 6)
            .map((p) => escapeHtml(p.name))
            .join(", ")}</p>`
        : ""
    }
  `;
}

function renderServerError(message) {
  serverResult.classList.remove("empty");
  serverResult.innerHTML = `<div class="error-box">${escapeHtml(message)}</div>`;
}

function renderPlayer(data, identifier) {
  const renderUrl = `/api/v1/render/${encodeURIComponent(
    identifier
  )}?type=full&size=256&overlay=true`;

  playerResult.classList.remove("empty");
  playerResult.innerHTML = `
    <div class="player-wrap">
      <dl class="kv-grid">
        <dt>Username</dt><dd>${escapeHtml(data.username)}</dd>
        <dt>UUID</dt><dd>${escapeHtml(data.uuid)}</dd>
        <dt>Model</dt><dd>${escapeHtml(data.skin?.model ?? "unknown")}</dd>
        <dt>Cape</dt><dd>${data.cape?.url ? "oui" : "non"}</dd>
        <dt>Skin URL</dt><dd>${escapeHtml(data.skin?.url ?? "N/A")}</dd>
      </dl>
      <img class="skin-preview" alt="Rendu skin ${escapeHtml(
        data.username
      )}" src="${renderUrl}" loading="lazy" />
    </div>
  `;
}

function renderPlayerError(message) {
  playerResult.classList.remove("empty");
  playerResult.innerHTML = `<div class="error-box">${escapeHtml(message)}</div>`;
}

async function lookupServer(identifier) {
  setState(serverState, "loading", "Chargement");
  const edition = editionSelect.value || "auto";
  const url = `/api/v1/server/${encodeURIComponent(identifier)}?type=${encodeURIComponent(edition)}`;

  try {
    const res = await fetch(url);
    const body = await res.json();
    if (!res.ok) {
      throw new Error(body.message ?? body.error ?? "Erreur serveur");
    }

    if (body.online) {
      setState(serverState, "online", "En ligne");
    } else {
      setState(serverState, "offline", "Hors ligne");
    }
    renderServer(body);
  } catch (err) {
    setState(serverState, "offline", "Erreur");
    renderServerError(err.message || "Lookup impossible");
  }
}

async function lookupPlayer(identifier) {
  setState(playerState, "loading", "Chargement");
  try {
    const res = await fetch(`/api/v1/player/${encodeURIComponent(identifier)}`);
    const body = await res.json();
    if (!res.ok) {
      throw new Error(body.message ?? body.error ?? "Erreur joueur");
    }

    setState(playerState, "online", "Trouve");
    renderPlayer(body, identifier);
  } catch (err) {
    setState(playerState, "offline", "Erreur");
    renderPlayerError(err.message || "Lookup impossible");
  }
}

form.addEventListener("submit", async (event) => {
  event.preventDefault();
  const identifier = input.value.trim();
  if (!identifier) {
    return;
  }

  const mode = getMode();
  if (mode === "server") {
    await lookupServer(identifier);
    return;
  }
  await lookupPlayer(identifier);
});
