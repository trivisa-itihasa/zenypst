import { ref, computed } from "vue";
import { getVersion } from "@tauri-apps/api/app";
import { check, type Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

type CheckStatus =
  | "idle"
  | "checking"
  | "up-to-date"
  | "available"
  | "downloading"
  | "ready"
  | "error";

const isTauri =
  typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

const currentVersion = ref<string | null>(null);
const latestVersion = ref<string | null>(null);
const status = ref<CheckStatus>("idle");
const errorMessage = ref<string | null>(null);
const downloadProgress = ref(0); // 0-100
let pendingUpdate: Update | null = null;
let lastCheckAt = 0;

async function resolveCurrentVersion(): Promise<string> {
  if (currentVersion.value) return currentVersion.value;
  if (isTauri) {
    try {
      currentVersion.value = await getVersion();
      return currentVersion.value;
    } catch {
      // fall through
    }
  }
  currentVersion.value = "0.0.0";
  return currentVersion.value;
}

export function useVersionCheck() {
  const isUpdateAvailable = computed(
    () => status.value === "available" || status.value === "ready",
  );

  async function checkForUpdates(force = false): Promise<void> {
    if (status.value === "checking" || status.value === "downloading") return;
    if (
      !force &&
      (status.value === "up-to-date" || status.value === "available") &&
      Date.now() - lastCheckAt < 5 * 60 * 1000
    ) {
      return;
    }
    if (!isTauri) {
      status.value = "error";
      errorMessage.value = "Updater is only available in the desktop app.";
      return;
    }

    status.value = "checking";
    errorMessage.value = null;

    try {
      await resolveCurrentVersion();
      const update = await check();

      if (update) {
        pendingUpdate = update;
        latestVersion.value = update.version;
        status.value = "available";
      } else {
        pendingUpdate = null;
        latestVersion.value = null;
        status.value = "up-to-date";
      }
      lastCheckAt = Date.now();
    } catch (e) {
      errorMessage.value = e instanceof Error ? e.message : String(e);
      status.value = "error";
    }
  }

  async function downloadAndInstall(): Promise<void> {
    if (!pendingUpdate) return;

    status.value = "downloading";
    downloadProgress.value = 0;

    try {
      let contentLength = 0;
      let downloaded = 0;

      await pendingUpdate.downloadAndInstall((event) => {
        switch (event.event) {
          case "Started":
            contentLength = event.data.contentLength ?? 0;
            break;
          case "Progress":
            downloaded += event.data.chunkLength;
            if (contentLength > 0) {
              downloadProgress.value = Math.round(
                (downloaded / contentLength) * 100,
              );
            }
            break;
          case "Finished":
            downloadProgress.value = 100;
            break;
        }
      });

      status.value = "ready";
    } catch (e) {
      errorMessage.value = e instanceof Error ? e.message : String(e);
      status.value = "error";
    }
  }

  async function restartApp(): Promise<void> {
    await relaunch();
  }

  return {
    currentVersion,
    latestVersion,
    status,
    errorMessage,
    isUpdateAvailable,
    downloadProgress,
    checkForUpdates,
    downloadAndInstall,
    restartApp,
  };
}
