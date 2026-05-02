<script>
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";
  import logo from "@assets/logo.svg";

  let phase = "checking_updates";
  let releaseUrl = "";
  let latestVersion = "";
  let currentVersion = "";

  let url = "";
  let mode = "video_with_audio";
  let preset = "compatibility";
  let videoQuality = "best";
  let audioQuality = "best";
  let progress = 0;
  let status = "Checking for updates...";
  let dependencyInfo = "";
  let busy = true;

  let unlistenProgress;
  let unlistenComplete;

  function withTimeout(promise, timeoutMs, timeoutMessage) {
    let timer;
    return Promise.race([
      promise,
      new Promise((_, reject) => {
        timer = setTimeout(() => reject(new Error(timeoutMessage)), timeoutMs);
      }),
    ]).finally(() => clearTimeout(timer));
  }

  function yieldToPaint() {
    return new Promise((resolve) => setTimeout(resolve, 0));
  }

  const modeOptions = [
    { value: "video_with_audio", label: "Video + Audio" },
    { value: "audio_only_mp3", label: "Audio only (MP3)" },
  ];

  const presetOptions = [
    { value: "compatibility", label: "Compatibility (H.264/AAC)" },
    { value: "max_quality", label: "Max Quality" },
  ];

  const videoOptions = [
    { value: "best", label: "Best" },
    { value: "p1080", label: "1080p" },
    { value: "p720", label: "720p" },
    { value: "p480", label: "480p" },
  ];

  const audioOptions = [
    { value: "best", label: "Best" },
    { value: "k320", label: "320k" },
    { value: "k192", label: "192k" },
    { value: "k128", label: "128k" },
  ];

  async function startApp() {
    busy = true;
    status = "Checking for updates...";
    await yieldToPaint();
    console.info("[startup] begin update check");
    let update;
    try {
      const startedAt = Date.now();
      update = await withTimeout(
        invoke("check_for_updates"),
        8000,
        "Update check timed out"
      );
      console.info(`[startup] update check finished in ${Date.now() - startedAt}ms`, update);
    } catch (_) {
      console.warn("[startup] update check failed or timed out; continuing");
      await continueAfterUpdateGate();
      return;
    }
    currentVersion = update.currentVersion;

    if (update.updateAvailable) {
      phase = "update_gate";
      releaseUrl = update.releaseUrl ?? "";
      latestVersion = update.latestVersion ?? "";
      status = "A new version is available.";
      busy = false;
      return;
    }

    await continueAfterUpdateGate();
  }

  async function continueAfterUpdateGate() {
    phase = "main";
    busy = true;
    status = "Bootstrapping dependencies...";
    await yieldToPaint();
    console.info("[startup] bootstrap dependencies begin");
    try {
      const startedAt = Date.now();
      const report = await invoke("bootstrap_dependencies");
      dependencyInfo = `Dependencies\nyt-dlp: ${report.ytDlp}\nffmpeg: ${report.ffmpeg}\nffprobe: ${report.ffprobe}`;
      status = "Ready";
      console.info(`[startup] bootstrap dependencies finished in ${Date.now() - startedAt}ms`, report);
    } catch (error) {
      console.error("[startup] bootstrap dependencies failed", error);
      status = `Dependency error: ${String(error)}`;
    } finally {
      busy = false;
    }
  }

  async function onDownload() {
    busy = true;
    progress = 0;
    status = "Preparing download...";

    try {
      await invoke("start_download", {
        payload: {
          url,
          mode,
          preset,
          videoQuality,
          audioQuality,
        },
      });
    } catch (error) {
      busy = false;
      status = `Download failed: ${String(error)}`;
    }
  }

  async function onOpenRelease() {
    await invoke("open_release_link", { url: releaseUrl });
  }

  async function onOpenGithub() {
    await invoke("open_github");
  }

  onMount(async () => {
    console.info("[startup] onMount");

    try {
      unlistenProgress = await listen("download-progress", (event) => {
        progress = event.payload.fraction;
        status = event.payload.message;
      });
      console.info("[startup] listen download-progress ok");
    } catch (error) {
      console.warn("[startup] listen download-progress failed", error);
    }

    try {
      unlistenComplete = await listen("download-complete", (event) => {
        busy = false;
        if (event.payload.ok) {
          progress = 1;
          status = "Finished";
        } else {
          status = `Download failed: ${event.payload.error}`;
        }
      });
      console.info("[startup] listen download-complete ok");
    } catch (error) {
      console.warn("[startup] listen download-complete failed", error);
    }

    try {
      await startApp();
    } catch (error) {
      console.error("[startup] startApp failed", error);
      busy = false;
      status = `Startup failed: ${String(error)}`;
      phase = "main";
    }
  });

  onDestroy(() => {
    if (unlistenProgress) {
      unlistenProgress();
    }
    if (unlistenComplete) {
      unlistenComplete();
    }
  });
</script>

{#if phase === "update_gate"}
  <main class="shell center">
    <section class="update-card">
      <img class="logo update-logo" src={logo} alt="Pullyt" />
      <h1>Update available</h1>
      <p>Installed version: v{currentVersion}</p>
      <p>Latest version: v{latestVersion}</p>
      <div class="actions">
        <button class="primary" on:click={onOpenRelease}>Download update</button>
        <button on:click={continueAfterUpdateGate}>Not now</button>
      </div>
    </section>
  </main>
{:else}
  <main class="shell">
    <section class="app-card">
      <img class="logo" src={logo} alt="Pullyt" />

      <div class="provider">YouTube</div>

      <input class="url-input" type="text" bind:value={url} placeholder="Paste YouTube URL" />

      <div class="row radios">
        {#each modeOptions as option}
          <label><input type="radio" bind:group={mode} value={option.value} /> {option.label}</label>
        {/each}
      </div>

      {#if mode === "video_with_audio"}
        <div class="row radios">
          <span class="field">Preset</span>
          {#each presetOptions as option}
            <label><input type="radio" bind:group={preset} value={option.value} /> {option.label}</label>
          {/each}
        </div>
      {/if}

      <div class="row radios">
        <span class="field">Video</span>
        {#each videoOptions as option}
          <label><input type="radio" bind:group={videoQuality} value={option.value} /> {option.label}</label>
        {/each}
      </div>

      <div class="row radios">
        <span class="field">Audio</span>
        {#each audioOptions as option}
          <label><input type="radio" bind:group={audioQuality} value={option.value} /> {option.label}</label>
        {/each}
      </div>

      <p class="status">{status}</p>

      <progress max="1" value={progress}></progress>

      <button class="primary" disabled={busy} on:click={onDownload}>{busy ? "Working..." : "Download"}</button>

      <footer>
        <pre>{dependencyInfo}</pre>
        <div class="meta-row">
          <span class="version">v{currentVersion || "1.0.0"}</span>
          <div class="dev">
            <span>developed by Pau Segarra</span>
            <button class="link" on:click={onOpenGithub}>Github</button>
          </div>
        </div>
      </footer>
    </section>
  </main>
{/if}
