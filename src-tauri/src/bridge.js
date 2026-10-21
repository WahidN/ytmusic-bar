(() => {
  if (!("mediaSession" in navigator)) return;

  const handlers = {};
  const setActionHandler = MediaSession.prototype.setActionHandler;
  MediaSession.prototype.setActionHandler = function (action, handler) {
    handlers[action] = handler;
    return setActionHandler.call(this, action, handler);
  };
  window.__ytmBar = { run: (action) => handlers[action]?.({ action }) };

  let isPlaying = false;
  let lastSent = "";

  const send = () => {
    const metadata = navigator.mediaSession.metadata;
    if (!metadata) return;
    const track = {
      title: metadata.title,
      artist: metadata.artist,
      artwork: metadata.artwork.at(-1)?.src ?? "",
      isPlaying,
    };
    const key = JSON.stringify(track);
    if (key === lastSent) return;
    lastSent = key;
    window.__TAURI_INTERNALS__.invoke("now_playing", { track });
  };

  // Een setter in plaats van een timer, want timers lopen traag als de popover verborgen is.
  const metadata = Object.getOwnPropertyDescriptor(MediaSession.prototype, "metadata");
  Object.defineProperty(MediaSession.prototype, "metadata", {
    ...metadata,
    set(value) {
      metadata.set.call(this, value);
      send();
    },
  });

  document.addEventListener("play", () => { isPlaying = true; send(); }, true);
  document.addEventListener("pause", () => { isPlaying = false; send(); }, true);
})();
