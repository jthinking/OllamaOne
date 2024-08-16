import { invoke } from "@tauri-apps/api/core";

export async function crawlUrl(url) {
  const result = await invoke("crawl_url", {
    url: url,
  });
  return result;
}
