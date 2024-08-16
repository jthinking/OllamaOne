<template>
  <q-layout view="hHh LpR lFf">
    <q-header :class="headerClass">
      <q-bar
        data-tauri-drag-region
        :class="$q.dark.isActive ? 'bg-dart-bar' : 'bg-light-bar'"
      >
        <q-icon v-if="$q.platform.is.win" :name="fasFeather" />
        <div v-if="$q.platform.is.win" class="text-weight-bold">
          <span :style="$q.dark.isActive ? 'color: white' : 'color: black'"
            >Ollama</span
          >One
        </div>
        <q-space />
        <div
          v-if="$q.platform.is.mac"
          class="col text-center text-weight-bold"
          data-tauri-drag-region
        >
          <q-icon size="1.2rem">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 310.44 294.66">
              <path
                d="m18.38,147.33c0-71.22,57.73-128.95,128.95-128.95,35.52,0,67.68,14.36,91,37.59,4.36-3.78,9.08-7.78,14.08-11.9C225.67,16.87,188.47,0,147.33,0,65.96,0,0,65.96,0,147.33c0,39.82,15.81,75.95,41.48,102.47,2.07-2.47,11.14-14.96,11.14-14.96-21.25-22.99-34.25-53.73-34.25-87.51Z"
                fill="#426cb4"
                stroke-width="0"
              />
              <path
                d="m276.12,140.79c.11,2.17.16,4.35.16,6.54,0,71.22-57.73,128.95-128.95,128.95-29.3,0-56.32-9.78-77.98-26.24,6.78-4.71,12.62-8.52,15.8-10.01,15.36-7.22,38.86-13.59,63.23-22.45l-18.25-4.83s63.79-5.26,96.02-31.21l-18.84-5.41C333.26,126.04,308.46,0,308.46,0c-4.19,16.5-39.81,53.75-39.81,53.75l-.24-11.35c-29.55,23.32-52.9,46.27-52.9,46.27l9.22-24.92c-40.97,27.36-78.24,51.89-78.24,51.89l15.98-23.74c-71.46,23.81-102.42,93.13-105.47,142.11,0,0,56.7-71.79,123.46-107.28,0,0-82.62,63.02-152.5,153.82,0,0,12.37-9.5,25.85-19.38,25.44,20.92,58,33.49,93.51,33.49,81.37,0,147.33-65.96,147.33-147.33,0-9.21-.85-18.21-2.47-26.95-4.55,6.9-9.86,13.76-16.07,20.41Z"
                fill="#426cb4"
                stroke-width="0"
              />
            </svg>
          </q-icon>
          <q-btn dense flat no-caps to="/">
            <span :style="$q.dark.isActive ? 'color: white' : 'color: black'"
              >Ollama</span
            >One
          </q-btn>
        </div>

        <div :class="windowBarClass">
          <q-btn dense flat icon="dashboard" to="/models">
            <q-tooltip> {{ $t("models.title") }} </q-tooltip>
          </q-btn>
          <q-btn dense flat icon="settings" to="/settings">
            <q-tooltip> {{ $t("menu.settings") }} </q-tooltip>
          </q-btn>
        </div>

        <div :class="windowBarClass" v-if="$q.platform.is.win">
          <q-btn dense flat icon="minimize" @click="minimizeWindow" />
          <q-btn dense flat icon="crop_square" @click="toggleMaximizeWindow" />
          <q-btn dense flat icon="close" @click="closeWindow" />
        </div>
      </q-bar>
    </q-header>

    <q-page-container>
      <keep-alive>
        <router-view />
      </keep-alive>
    </q-page-container>
  </q-layout>
</template>

<script>
import { defineComponent, ref, onMounted, computed } from "vue";
import EssentialLink from "components/EssentialLink.vue";
import { Window } from "@tauri-apps/api/window";
import { Webview } from "@tauri-apps/api/webview";

// loading embedded asset:
const appWindow = new Window("main");
// With the Tauri API npm package:
import { invoke } from "@tauri-apps/api/core";
import { fasFeather } from "@quasar/extras/fontawesome-v6";
import { useQuasar } from "quasar";
import { useI18n } from "vue-i18n";
import { getCurrent } from "@tauri-apps/api/window";

export default defineComponent({
  name: "MainLayout",

  components: {},

  setup() {
    const $q = useQuasar();
    const miniState = ref(true);

    const { t } = useI18n();

    const msg = computed(() => t("menu.chat"));

    const linksList = ref([
      {
        title: msg,
        caption: "OllamaOne Assistant",
        icon: "bubble_chart",
        link: "/",
      },
    ]);

    const dark_theme = localStorage.getItem("dark_theme");
    if (dark_theme === "auto") {
      $q.dark.set("auto");
    } else if (dark_theme === "dark") {
      $q.dark.set(true);
    } else {
      $q.dark.set(false);
    }

    onMounted(async () => {
      await getCurrent().show();
    });

    const headerClass = computed(() => {
      return $q.dark.isActive
        ? "bg-dart-bar text-primary"
        : "bg-light-bar text-primary";
    });

    const windowBarClass = computed(() => {
      return $q.dark.isActive ? "bg-dart-bar" : "bg-light-bar text-dark";
    });

    const closeWindow = () => {
      appWindow.close();
    };
    const minimizeWindow = () => {
      appWindow.minimize();
    };
    const toggleMaximizeWindow = () => {
      appWindow.toggleMaximize();
    };

    // const drawerClick = (e) => {
    //   // if in "mini" state and user
    //   // click on drawer, we switch it to "normal" mode
    //   if (miniState.value) {
    //     miniState.value = false;

    //     // notice we have registered an event with capture flag;
    //     // we need to stop further propagation as this click is
    //     // intended for switching drawer to "normal" mode only
    //     e.stopPropagation();
    //   }
    // };

    return {
      essentialLinks: linksList,
      closeWindow,
      minimizeWindow,
      toggleMaximizeWindow,
      miniState,
      drawer: ref(false),
      fasFeather,
      headerClass,
      windowBarClass,
    };
  },
});
</script>

<style lang="css">
.text-dart-bar {
  color: #202124 !important;
}
.bg-dart-bar {
  background: #202124 !important;
}
.text-light-bar {
  color: #dee1e6 !important;
}
.bg-light-bar {
  background: #dee1e6 !important;
}
</style>
