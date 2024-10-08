import { boot } from "quasar/wrappers";
import { createI18n } from "vue-i18n";
import messages from "src/i18n";
import { Quasar } from "quasar";

export default boot(({ app }) => {
  const cacheLang = localStorage.getItem("lang");
  const locale = cacheLang ? cacheLang : Quasar.lang.getLocale();
  const i18n = createI18n({
    legacy: false,
    locale: locale,
    globalInjection: true,
    messages,
  });

  // Set i18n instance on app
  app.use(i18n);
});
