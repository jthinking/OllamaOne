import { invoke } from "@tauri-apps/api/core";

export async function listConfigs(config) {
  const result = await invoke("list_configs", {
    params: { page: 1, postsPerPage: 100 },
  });
  return result;
}

export async function createConfig(config) {
  const result = await invoke("create_config", {
    form: { id: config.id, value: config.value },
  });
  return result;
}

export async function updateConfig(config) {
  const result = await invoke("update_config", {
    id: config.id,
    form: { value: config.value },
  });
  return result;
}

export async function deleteConfig(config) {
  const result = await invoke("delete_config", {
    id: config.id,
  });
  return result;
}

export async function getConfig(config) {
  const result = await invoke("get_config", {
    id: config.id,
  });
  return result;
}
