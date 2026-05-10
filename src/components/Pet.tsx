import { useEffect, useState, useRef, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface StateConfig {
  frames: string[];
  fps?: number;
  loopType?: string;
}

interface SkinConfig {
  id: string;
  name: string;
  states: Record<string, StateConfig>;
  canvas?: { width: number; height: number };
}

interface PetSettings {
  current_skin_id: string;
  window_x: number;
  window_y: number;
}

export function Pet() {
  const [skin, setSkin] = useState<SkinConfig | null>(null);
  const [settings, setSettings] = useState<PetSettings | null>(null);
  const [frameBase64, setFrameBase64] = useState<string>("");
  const [currentState, setCurrentState] = useState<string>("idle");
  const frameIndexRef = useRef<number>(0);
  const isDraggingRef = useRef<boolean>(false);
  const hoverTimeoutRef = useRef<number | null>(null);
  const sleepTimerRef = useRef<number | null>(null);

  // Load settings and skin on startup
  useEffect(() => {
    async function init() {
      try {
        // Load saved settings
        const s = await invoke<PetSettings>("get_settings");
        setSettings(s);

        // Load last used skin
        const skinPath = `assets/skins/${s.current_skin_id}`;
        const skin = await invoke<SkinConfig>("load_skin", { path: skinPath });
        setSkin(skin);
      } catch (e) {
        // Fallback to default skin
        console.warn("Failed to load settings, using default");
        try {
          const skin = await invoke<SkinConfig>("load_skin", {
            path: "assets/skins/default",
          });
          setSkin(skin);
        } catch (err) {
          console.error("Failed to load default skin:", err);
        }
      }
    }
    init();
  }, []);

  // 监听托盘皮肤切换事件
  useEffect(() => {
    const unlisten = listen<string>('switch_skin', async (event) => {
      const skinId = event.payload;
      try {
        const skinPath = `assets/skins/${skinId}`;
        const skin = await invoke<SkinConfig>('load_skin', { path: skinPath });
        setSkin(skin);
        setCurrentState('idle');
        frameIndexRef.current = 0;

        // 保存设置
        if (settings) {
          const newSettings = { ...settings, current_skin_id: skinId };
          await invoke('save_settings_cmd', { settings: newSettings });
          setSettings(newSettings);
        }
      } catch (e) {
        console.error('Failed to switch skin:', e);
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [settings]);

  // 帧播放（setTimeout 递归，修复帧堆积 TECH_DEBT #7）
  const playNextFrame = useCallback(async () => {
    if (!skin || !skin.states[currentState]) return;

    const state = skin.states[currentState];
    const totalFrames = state.frames.length;
    const frameName = state.frames[frameIndexRef.current];

    // Use the current skin path from settings or default
    const skinPath = settings
      ? `assets/skins/${settings.current_skin_id}`
      : "assets/skins/default";

    try {
      const base64 = await invoke<string>("get_frame_base64", {
        skinPath,
        frameName,
      });
      setFrameBase64(`data:image/png;base64,${base64}`);

      frameIndexRef.current++;
      if (frameIndexRef.current >= totalFrames) {
        if (state.loopType === "oneshot") {
          setCurrentState("idle");
          frameIndexRef.current = 0;
          return; // oneshot done, don't schedule next frame
        } else {
          frameIndexRef.current = 0;
        }
      }

      // setTimeout recursion (avoid setInterval frame buildup)
      const fps = state.fps || 1;
      setTimeout(playNextFrame, 1000 / fps);
    } catch (e) {
      console.error("Frame load error:", e);
    }
  }, [skin, currentState, settings]);

  useEffect(() => {
    if (!skin) return;
    frameIndexRef.current = 0;
    playNextFrame();
    // 不需要清理 setTimeout，playNextFrame 链在状态切换时自然终止
  }, [skin, currentState, playNextFrame]);

  // idle 30s → sleep
  useEffect(() => {
    if (currentState !== "idle") {
      if (sleepTimerRef.current) clearTimeout(sleepTimerRef.current);
      return;
    }

    sleepTimerRef.current = window.setTimeout(() => {
      setCurrentState("sleep");
      frameIndexRef.current = 0;
    }, 30000);

    return () => {
      if (sleepTimerRef.current) clearTimeout(sleepTimerRef.current);
    };
  }, [currentState]);

  // 鼠标移动唤醒 sleep → wake
  const handleMouseMove = () => {
    if (currentState === "sleep") {
      setCurrentState("wake");
      frameIndexRef.current = 0;
    }
  };

  // wake 结束 → idle
  useEffect(() => {
    if (
      currentState === "wake" &&
      skin?.states.wake?.loopType === "oneshot"
    ) {
      const wakeDuration =
        skin.states.wake.frames.length *
        (1000 / (skin.states.wake.fps || 1));
      const timer = window.setTimeout(() => {
        setCurrentState("idle");
        frameIndexRef.current = 0;
      }, wakeDuration);
      return () => clearTimeout(timer);
    }
  }, [currentState, skin]);

  // --- 鼠标交互事件 ---

  const handleMouseEnter = () => {
    if (currentState !== "idle" || isDraggingRef.current) return;
    hoverTimeoutRef.current = window.setTimeout(() => {
      setCurrentState("hover");
      frameIndexRef.current = 0;
    }, 200);
  };

  const handleMouseLeave = () => {
    if (hoverTimeoutRef.current) {
      clearTimeout(hoverTimeoutRef.current);
    }
    if (currentState === "hover") {
      setCurrentState("idle");
      frameIndexRef.current = 0;
    }
  };

  const handleClick = () => {
    if (isDraggingRef.current) return;
    setCurrentState("click");
    frameIndexRef.current = 0;
  };

  const handleMouseDown = async (e: React.MouseEvent) => {
    if (e.button !== 0) return; // 仅左键
    isDraggingRef.current = true;
    // 进入拖拽前先禁用穿透
    await invoke("set_ignore_cursor_events", { ignore: false });
    try {
      await invoke("start_dragging");
    } catch (err) {
      isDraggingRef.current = false;
      await invoke("set_ignore_cursor_events", { ignore: true });
    }
  };

  const handleMouseUp = async () => {
    if (!isDraggingRef.current) return;
    isDraggingRef.current = false;
    try {
      await invoke("stop_dragging");
    } catch (err) {
      // 忽略
    }
    setCurrentState("idle");
    frameIndexRef.current = 0;
  };

  // 全局 mouseup 保底：防止拖拽未正常释放
  useEffect(() => {
    const handleGlobalMouseUp = () => {
      if (isDraggingRef.current) {
        isDraggingRef.current = false;
        invoke("stop_dragging").catch(() => {});
        setCurrentState("idle");
        frameIndexRef.current = 0;
      }
    };

    window.addEventListener("mouseup", handleGlobalMouseUp);
    return () => window.removeEventListener("mouseup", handleGlobalMouseUp);
  }, []);

  if (!frameBase64) {
    return (
      <div
        style={{
          width: skin?.canvas?.width || 512,
          height: skin?.canvas?.height || 512,
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
        }}
      >
        Loading...
      </div>
    );
  }

  return (
    <div
      style={{
        width: skin?.canvas?.width || 512,
        height: skin?.canvas?.height || 512,
      }}
      onMouseMove={handleMouseMove}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onClick={handleClick}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
    >
      <img
        src={frameBase64}
        alt="pet"
        style={{ width: "100%", height: "100%", objectFit: "contain" }}
      />
    </div>
  );
}
