import { ref, computed } from "vue";
import { getVersion } from "@tauri-apps/api/app";

const GITHUB_OWNER = "trivisa-itihasa";
const GITHUB_REPO = "zenypst";
const RELEASES_URL = `https://github.com/${GITHUB_OWNER}/${GITHUB_REPO}/releases`;
const LATEST_API_URL = `https://api.github.com/repos/${GITHUB_OWNER}/${GITHUB_REPO}/releases/latest`;

interface GitHubRelease {
  tag_name: string;
  html_url: string;
  name?: string;
  prerelease?: boolean;
  draft?: boolean;
}

type CheckStatus = "idle" | "checking" | "ok" | "no-release" | "error";

const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

const currentVersion = ref<string | null>(null);
const latestVersion = ref<string | null>(null);
const releaseUrl = ref<string>(RELEASES_URL);
const status = ref<CheckStatus>("idle");
const errorMessage = ref<string | null>(null);
let lastCheckAt = 0;

function parseVersion(v: string): number[] {
  return v
    .replace(/^v/i, "")
    .split("-")[0]
    .split(".")
    .map((n) => parseInt(n, 10) || 0);
}

function isNewer(latest: string, current: string): boolean {
  const a = parseVersion(latest);
  const b = parseVersion(current);
  const len = Math.max(a.length, b.length);
  for (let i = 0; i < len; i++) {
    const x = a[i] ?? 0;
    const y = b[i] ?? 0;
    if (x > y) return true;
    if (x < y) return false;
  }
  return false;
}

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
  const isUpdateAvailable = computed(() => {
    if (status.value !== "ok") return false;
    if (!latestVersion.value || !currentVersion.value) return false;
    return isNewer(latestVersion.value, currentVersion.value);
  });

  async function checkForUpdates(force = false): Promise<void> {
    if (status.value === "checking") return;
    // Cache: skip if checked within last 5 minutes unless forced
    if (!force && status.value === "ok" && Date.now() - lastCheckAt < 5 * 60 * 1000) {
      return;
    }
    status.value = "checking";
    errorMessage.value = null;
    try {
      await resolveCurrentVersion();
      const res = await fetch(LATEST_API_URL, {
        headers: { Accept: "application/vnd.github+json" },
      });
      if (res.status === 404) {
        latestVersion.value = null;
        releaseUrl.value = RELEASES_URL;
        status.value = "no-release";
        lastCheckAt = Date.now();
        return;
      }
      if (!res.ok) {
        throw new Error(`GitHub API responded with ${res.status}`);
      }
      const data = (await res.json()) as GitHubRelease;
      latestVersion.value = data.tag_name.replace(/^v/i, "");
      releaseUrl.value = data.html_url || RELEASES_URL;
      status.value = "ok";
      lastCheckAt = Date.now();
    } catch (e) {
      errorMessage.value = e instanceof Error ? e.message : String(e);
      status.value = "error";
    }
  }

  return {
    currentVersion,
    latestVersion,
    releaseUrl,
    status,
    errorMessage,
    isUpdateAvailable,
    checkForUpdates,
  };
}
