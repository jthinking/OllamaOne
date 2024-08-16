import { invoke } from "@tauri-apps/api/core";
import { getCurrentDate } from "../utils/format";

export async function createChat(message) {
  const timestamp = Date.now();
  let title = getCurrentDate();
  if (message.messages.length > 0) {
    let firstMsg = message.messages[0].content;
    title = firstMsg.length > 25 ? firstMsg.substring(0, 25) + "..." : firstMsg;
  }
  const result = await invoke("create_chat", {
    form: {
      id: 0,
      model: message.model,
      plugin: message.plugin,
      shared: false,
      marked: false,
      messages: JSON.stringify(message.messages),
      title: title,
      create_time: timestamp,
      update_time: timestamp,
      status: 1,
    },
  });
  return result;
}

export async function updateChat(message) {
  const timestamp = Date.now();

  const result = await invoke("update_chat", {
    id: message.id,
    form: {
      ...message,
      messages: JSON.stringify(message.messages),
      update_time: timestamp,
    },
  });
  return result;
}

export async function deleteChat(message) {
  const result = await invoke("delete_chat", {
    id: message.id,
  });
  return result;
}

export async function getChat(message) {
  const result = await invoke("get_chat", {
    id: message.id,
  });
  return result;
}

/**
 *
 * @param { id: string; title: string } chat
 * @returns
 */
export async function updateChatTitle(chat) {
  const timestamp = Date.now();

  const result = await invoke("update_chat_title", {
    id: message.id,
    form: {
      title: chat.title,
      update_time: timestamp,
    },
  });
  return result;
}

export async function markChat(updateMarked) {
  const result = await invoke("mark_chat", {
    id: message.id,
    form: {
      marked: updateMarked.marked,
    },
  });
  return result;
}

export async function listChats(query) {
  const result = await invoke("list_chats", {
    params: { page: query.current, configs_per_page: query.pageSize },
  });
  return result;
}
