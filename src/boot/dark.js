import { boot } from "quasar/wrappers";
import { Dark } from "quasar";
import { invoke } from "@tauri-apps/api/core";

export default boot(({ app }) => {
  // set status
  // or false or "auto"
  const dark_theme = localStorage.getItem("dark_theme");
  if (dark_theme === "auto") {
    Dark.set("auto");
  } else if (dark_theme === "dark") {
    Dark.set(true);
  } else {
    Dark.set(false);
  }
});
