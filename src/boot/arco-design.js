import { boot } from "quasar/wrappers";
import ArcoVue from "@arco-design/web-vue";
import "@arco-design/web-vue/dist/arco.css";
import { Message } from "@arco-design/web-vue";
import { Notification } from "@arco-design/web-vue";

export default boot(({ app }) => {
  app.use(ArcoVue);
  Message._context = app._context;
  Notification._context = app._context;
});
