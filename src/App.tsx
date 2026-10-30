import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import "./App.css";

type NowPlaying = { title: string; artist: string; artwork: string; isPlaying: boolean };
type Notch = { width: number; height: number };
type Size = { width: number; height: number };

const EXPANDED_WIDTH = 380;
const EXPANDED_BODY_HEIGHT = 104;
const WING_PADDING = 8;
const COMPACT_RADIUS = 10;
const EXPANDED_RADIUS = 24;

const reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)");

function setIslandSize(size: Size) {
  return invoke("set_island_size", { ...size });
}

function Island({ notch }: { notch: Notch }) {
  const [track, setTrack] = useState<NowPlaying>();
  const [open, setOpen] = useState(false);
  const full = { width: EXPANDED_WIDTH, height: notch.height + EXPANDED_BODY_HEIGHT };
  const wing = notch.height + WING_PADDING;
  const compact = {
    width: track?.isPlaying ? notch.width + 2 * wing : notch.width,
    height: notch.height,
  };
  const expanded = open && Boolean(track?.title);
  const target = expanded ? full : compact;
  const [shape, setShape] = useState<Size>(compact);
  const shapeRef = useRef(shape);

  useEffect(() => {
    invoke<NowPlaying>("get_now_playing").then(setTrack);
    const unlisten = listen<NowPlaying>("now-playing", (event) => setTrack(event.payload));
    const unlistenFocus = getCurrentWindow().onFocusChanged(({ payload: focused }) => {
      if (!focused) setOpen(false);
    });
    return () => {
      unlisten.then((stop) => stop());
      unlistenFocus.then((stop) => stop());
    };
  }, []);

  // Eerst het venster groot maken, dan de vorm laten groeien, zodat de rand van het venster nooit door de animatie snijdt.
  useEffect(() => {
    let cancelled = false;
    const current = shapeRef.current;
    setIslandSize({
      width: Math.max(current.width, target.width),
      height: Math.max(current.height, target.height),
    }).then(() => {
      if (cancelled) return;
      shapeRef.current = target;
      setShape(target);
      if (reducedMotion.matches) setIslandSize(target);
    });
    return () => {
      cancelled = true;
    };
  }, [target.width, target.height]);

  const inset = `${(full.width - shape.width) / 2}px`;
  const radius = expanded ? EXPANDED_RADIUS : COMPACT_RADIUS;
  const clipPath = `inset(0 ${inset} ${full.height - shape.height}px round 0 0 ${radius}px ${radius}px)`;

  return (
    <div
      className="island"
      data-expanded={expanded}
      data-playing={Boolean(track?.isPlaying)}
      style={{ width: full.width, height: full.height, clipPath }}
      onClick={() => setOpen(true)}
      onTransitionEnd={(event) => {
        if (event.target === event.currentTarget && event.propertyName === "clip-path") {
          setIslandSize(shapeRef.current);
        }
      }}
    >
      {track && (
        <>
          <div className="compact" style={{ height: notch.height, paddingInline: inset }}>
            <img className="compact-cover" src={track.artwork || undefined} alt="" />
            <Bars />
          </div>
          <div className="expanded" style={{ paddingTop: notch.height }}>
            <div className="track">
              <img className="cover" src={track.artwork || undefined} alt="" />
              <div className="meta">
                <div className="title">{track.title}</div>
                <div className="artist">{track.artist}</div>
              </div>
            </div>
            <div className="controls">
              <button aria-label="Vorige" onClick={() => invoke("media_action", { action: "previoustrack" })}>
                <PreviousIcon />
              </button>
              <button
                aria-label={track.isPlaying ? "Pauzeren" : "Afspelen"}
                onClick={() => invoke("media_action", { action: track.isPlaying ? "pause" : "play" })}
              >
                {track.isPlaying ? <PauseIcon /> : <PlayIcon />}
              </button>
              <button aria-label="Volgende" onClick={() => invoke("media_action", { action: "nexttrack" })}>
                <NextIcon />
              </button>
            </div>
          </div>
        </>
      )}
    </div>
  );
}

function Bars() {
  return (
    <div className="bars">
      <span className="bar" />
      <span className="bar" />
      <span className="bar" />
      <span className="bar" />
    </div>
  );
}

function PlayIcon() {
  return (
    <svg viewBox="0 0 24 24" aria-hidden="true">
      <path d="M7 4.5v15a1 1 0 0 0 1.5.86l12.5-7.5a1 1 0 0 0 0-1.72L8.5 3.64A1 1 0 0 0 7 4.5z" />
    </svg>
  );
}

function PauseIcon() {
  return (
    <svg viewBox="0 0 24 24" aria-hidden="true">
      <rect x="6" y="4" width="4" height="16" rx="1.2" />
      <rect x="14" y="4" width="4" height="16" rx="1.2" />
    </svg>
  );
}

function NextIcon() {
  return (
    <svg viewBox="0 0 24 24" aria-hidden="true">
      <path d="M3 6.5v11a1 1 0 0 0 1.55.83L12 13.3v4.2a1 1 0 0 0 1.55.83l8.25-5.5a1 1 0 0 0 0-1.66l-8.25-5.5A1 1 0 0 0 12 6.5v4.2L4.55 5.67A1 1 0 0 0 3 6.5z" />
    </svg>
  );
}

function PreviousIcon() {
  return (
    <svg viewBox="0 0 24 24" aria-hidden="true" style={{ transform: "scaleX(-1)" }}>
      <path d="M3 6.5v11a1 1 0 0 0 1.55.83L12 13.3v4.2a1 1 0 0 0 1.55.83l8.25-5.5a1 1 0 0 0 0-1.66l-8.25-5.5A1 1 0 0 0 12 6.5v4.2L4.55 5.67A1 1 0 0 0 3 6.5z" />
    </svg>
  );
}

function App() {
  const [notch, setNotch] = useState<Notch>();

  useEffect(() => {
    invoke<Notch>("get_notch").then(setNotch);
  }, []);

  return notch ? <Island notch={notch} /> : null;
}

export default App;
