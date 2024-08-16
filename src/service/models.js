import { Ollama } from "ollama/browser";
import { fetch } from "@tauri-apps/plugin-http";

export async function showModel(model) {
  const ollama_url = localStorage.getItem("ollama_url");
  const ollamaURL = ollama_url ? ollama_url : "http://localhost:11434";
  const ollama = new Ollama({
    host: ollamaURL,
    fetch: fetch,
  });
  try {
    const response = await ollama.show({ model: model });
    return response;
  } catch (error) {
    console.error(error);
    return undefined;
  }
}
