<template>
  <q-page class="flex flex-center">
    <div>
      <q-splitter v-model="splitterModel" :style="splitterStyle">
        <template v-slot:before>
          <q-tabs v-model="tab" vertical class="text-teal">
            <!-- <q-tab name="general" label="通用设置" /> -->
            <q-tab
              name="alarms"
              no-caps
              :label="$t('settings.menu.personalization')"
            />
            <q-tab name="language" no-caps :label="$t('settings.menu.lang')" />
            <q-tab name="ollama" no-caps label="Ollama" />
            <!-- <q-tab name="nodes" no-caps label="网络节点" /> -->
            <!-- <q-tab
              name="license"
              no-caps
              :label="$t('settings.menu.license')"
            /> -->
            <q-tab
              name="feedback"
              no-caps
              :label="$t('settings.menu.feedback')"
            />
            <q-tab name="about" no-caps :label="$t('settings.menu.about')" />
          </q-tabs>
        </template>

        <template v-slot:after>
          <q-tab-panels
            v-model="tab"
            animated
            swipeable
            vertical
            transition-prev="jump-up"
            transition-next="jump-up"
          >
            <q-tab-panel name="general">
              <div class="text-h4 q-mb-md">通用设置</div>

              <q-scroll-area style="height: 300px">
                <q-list padding>
                  <!-- <q-item-label header>主题</q-item-label>
                  <q-item>
                    <q-item-section top side>
                      <div class="text-grey-8 q-gutter-xs">
                        <q-radio
                          v-model="userTheme"
                          val="light"
                          label="浅色主题"
                          @update:model-value="changeTheme"
                        />
                        <q-radio
                          v-model="userTheme"
                          val="dark"
                          label="深色主题"
                          @update:model-value="changeTheme"
                        />
                        <q-radio
                          v-model="userTheme"
                          val="auto"
                          label="跟随系统"
                          @update:model-value="changeTheme"
                        />
                      </div>
                    </q-item-section>
                  </q-item> -->
                  <q-item-label header>功能特性</q-item-label>

                  <q-item tag="label" v-ripple>
                    <q-item-section>
                      <q-item-label>开启历史聊天记录滚动</q-item-label>
                      <q-item-label caption
                        >当历史对话Token数超过上限时，自动将最早的对话忽略，确保新对话可以被回答，但会导致被忽略的对话记忆消失。</q-item-label
                      >
                    </q-item-section>
                    <q-item-section side top>
                      <q-toggle color="green" v-model="notif2" val="friend" />
                    </q-item-section>
                  </q-item>

                  <q-item tag="label" v-ripple>
                    <q-item-section>
                      <q-item-label>开启网址内容实时获取</q-item-label>
                      <q-item-label caption
                        >当发送内容包含网站链接时，自动获取网站链接的网页内容进行解析发送。</q-item-label
                      >
                    </q-item-section>
                    <q-item-section side>
                      <q-toggle color="blue" v-model="notif1" val="battery" />
                    </q-item-section>
                  </q-item>

                  <q-item tag="label" v-ripple>
                    <q-item-section>
                      <q-item-label>开启更加精准回复模式</q-item-label>
                      <q-item-label caption
                        >增加回复内容的准确性，但无法完全避免回复内容带有编造成分。</q-item-label
                      >
                    </q-item-section>
                    <q-item-section side top>
                      <q-toggle color="red" v-model="notif3" val="picture" />
                    </q-item-section>
                  </q-item>
                </q-list>
              </q-scroll-area>
            </q-tab-panel>

            <q-tab-panel name="ollama">
              <div class="text-h4 q-mb-md">Ollama</div>
              <q-scroll-area style="height: 300px">
                <q-list padding>
                  <q-item-label header>Ollama URL</q-item-label>
                  <q-item>
                    <q-item-section>
                      <div class="text-grey-8 q-gutter-xs">
                        <q-input
                          outlined
                          bottom-slots
                          v-model="ollamaURL"
                          :readonly="ollamaURLReadonly"
                          label="Ollama URL"
                        >
                          <template v-slot:hint> {{ activateMsg }} </template>
                          <template v-slot:append>
                            <q-btn
                              round
                              dense
                              flat
                              icon="edit"
                              @click="ollamaURLReadonly = !ollamaURLReadonly"
                            />
                          </template>
                          <template v-slot:after>
                            <q-btn color="primary" @click="changeOllamaURL">{{
                              $t("settings.menu.license.update.btn")
                            }}</q-btn>
                          </template>
                        </q-input>
                      </div>
                    </q-item-section>
                  </q-item>
                </q-list>
              </q-scroll-area>
            </q-tab-panel>

            <q-tab-panel name="account">
              <div class="text-h4 q-mb-md">账号设置</div>

              <q-scroll-area style="height: 300px">
                <q-list padding>
                  <q-item-label header>General</q-item-label>

                  <q-item tag="label" v-ripple>
                    <q-item-section side top>
                      <q-checkbox v-model="check1" />
                    </q-item-section>

                    <q-item-section>
                      <q-item-label>Notifications</q-item-label>
                      <q-item-label caption>
                        Notify me about updates to apps or games that I
                        downloaded
                      </q-item-label>
                    </q-item-section>
                  </q-item>

                  <q-item tag="label" v-ripple>
                    <q-item-section side top>
                      <q-checkbox v-model="check2" />
                    </q-item-section>

                    <q-item-section>
                      <q-item-label>Sound</q-item-label>
                      <q-item-label caption>
                        Auto-update apps at anytime. Data charges may apply
                      </q-item-label>
                    </q-item-section>
                  </q-item>

                  <q-item tag="label" v-ripple>
                    <q-item-section side top>
                      <q-checkbox v-model="check3" />
                    </q-item-section>

                    <q-item-section>
                      <q-item-label>Auto-add widgets</q-item-label>
                      <q-item-label caption>
                        Automatically add home screen widgets
                      </q-item-label>
                    </q-item-section>
                  </q-item>

                  <q-separator spaced />
                  <q-item-label header>Notifications</q-item-label>

                  <q-item tag="label" v-ripple>
                    <q-item-section>
                      <q-item-label>Battery too low</q-item-label>
                    </q-item-section>
                    <q-item-section side>
                      <q-toggle color="blue" v-model="notif1" val="battery" />
                    </q-item-section>
                  </q-item>

                  <q-item tag="label" v-ripple>
                    <q-item-section>
                      <q-item-label>Friend request</q-item-label>
                      <q-item-label caption>Allow notification</q-item-label>
                    </q-item-section>
                    <q-item-section side top>
                      <q-toggle color="green" v-model="notif2" val="friend" />
                    </q-item-section>
                  </q-item>

                  <q-item tag="label" v-ripple>
                    <q-item-section>
                      <q-item-label>Picture uploaded</q-item-label>
                      <q-item-label caption
                        >Allow notification when uploading images</q-item-label
                      >
                    </q-item-section>
                    <q-item-section side top>
                      <q-toggle color="red" v-model="notif3" val="picture" />
                    </q-item-section>
                  </q-item>
                </q-list>
              </q-scroll-area>
            </q-tab-panel>

            <q-tab-panel name="alarms">
              <div class="text-h4 q-mb-md">
                {{ $t("settings.menu.personalization") }}
              </div>
              <q-scroll-area style="height: 300px">
                <q-list padding>
                  <q-item-label header>{{
                    $t("settings.menu.theme")
                  }}</q-item-label>
                  <q-item>
                    <q-item-section top side>
                      <div class="text-grey-8 q-gutter-xs q-pa-xs">
                        <div class="q-gutter-md row">
                          <q-radio
                            v-model="userTheme"
                            val="light"
                            :label="$t('settings.menu.theme.light')"
                            @update:model-value="changeTheme"
                          />
                          <q-radio
                            v-model="userTheme"
                            val="dark"
                            :label="$t('settings.menu.theme.dark')"
                            @update:model-value="changeTheme"
                          />
                          <q-radio
                            v-model="userTheme"
                            val="auto"
                            :label="$t('settings.menu.theme.system')"
                            @update:model-value="changeTheme"
                          />
                        </div>
                      </div>
                    </q-item-section>
                  </q-item>

                  <!-- <q-item-label header>配色</q-item-label>
                  <q-item>
                    <q-item-section>
                      <q-color
                        v-model="hex"
                        default-view="palette"
                        no-footer
                        :palette="[
                          '#019A9D',
                          '#D9B801',
                          '#E8045A',
                          '#B2028A',
                          '#2A0449',
                          '#019A9D',
                        ]"
                        class="my-picker"
                      />
                    </q-item-section>
                  </q-item> -->
                </q-list>
              </q-scroll-area>
            </q-tab-panel>

            <q-tab-panel name="language">
              <div class="text-h4 q-mb-md">{{ $t("settings.menu.lang") }}</div>
              <q-scroll-area style="height: 300px">
                <q-list padding>
                  <q-item-label header>{{
                    $t("settings.menu.lang.ui")
                  }}</q-item-label>
                  <q-item>
                    <q-item-section top side>
                      <div class="text-grey-8 q-gutter-xs q-pa-xs">
                        <div class="q-gutter-md row">
                          <q-select
                            v-model="lang"
                            :options="langOptions"
                            :label="$t('settings.menu.lang.tip')"
                            dense
                            emit-value
                            map-options
                            options-dense
                            style="min-width: 150px"
                          />
                        </div>
                      </div>
                    </q-item-section>
                  </q-item>
                  <!-- <q-item-label header>{{
                    $t("settings.menu.lang.msg")
                  }}</q-item-label>
                  <q-item>
                    <q-item-section top side>
                      <div class="text-grey-8 q-gutter-xs q-pa-xs">
                        <div class="q-gutter-md row">
                          <q-select
                            v-model="msgLang"
                            :options="langOptions"
                            :label="$t('settings.menu.lang.tip')"
                            dense
                            emit-value
                            map-options
                            options-dense
                            style="min-width: 150px"
                          />
                        </div>
                      </div>
                    </q-item-section>
                  </q-item> -->
                </q-list>
              </q-scroll-area>
            </q-tab-panel>

            <q-tab-panel name="nodes">
              <div class="text-h4 q-mb-md">网络节点</div>
              <q-scroll-area style="height: 300px">
                <q-list padding>
                  <q-item-label header>{{
                    $t("settings.menu.lang.ui")
                  }}</q-item-label>
                  <q-item>
                    <q-item-section top side>
                      <div class="text-grey-8 q-gutter-xs q-pa-xs">
                        <div class="q-gutter-md row">
                          <q-select
                            v-model="lang"
                            :options="langOptions"
                            :label="$t('settings.menu.lang.tip')"
                            dense
                            emit-value
                            map-options
                            options-dense
                            style="min-width: 150px"
                          />
                        </div>
                      </div>
                    </q-item-section>
                  </q-item>
                  <!-- <q-item-label header>{{
                    $t("settings.menu.lang.msg")
                  }}</q-item-label>
                  <q-item>
                    <q-item-section top side>
                      <div class="text-grey-8 q-gutter-xs q-pa-xs">
                        <div class="q-gutter-md row">
                          <q-select
                            v-model="msgLang"
                            :options="langOptions"
                            :label="$t('settings.menu.lang.tip')"
                            dense
                            emit-value
                            map-options
                            options-dense
                            style="min-width: 150px"
                          />
                        </div>
                      </div>
                    </q-item-section>
                  </q-item> -->
                </q-list>
              </q-scroll-area>
            </q-tab-panel>

            <q-tab-panel name="feedback">
              <div class="text-h4 q-mb-md">
                {{ $t("settings.menu.feedback") }}
              </div>
              <div class="q-pa-md q-gutter-sm">
                <q-scroll-area style="height: 230px">
                  <q-input
                    v-model="feedback"
                    outlined
                    autogrow
                    type="textarea"
                    :placeholder="$t('settings.menu.feedback.tip')"
                  />
                </q-scroll-area>
                <q-btn
                  color="primary"
                  :loading="feedbackBtnDisabled"
                  :label="$t('settings.menu.feedback.submit')"
                  @click="sendFeedback"
                />
              </div>
            </q-tab-panel>

            <q-tab-panel name="about">
              <div class="text-h4 q-mb-md">{{ $t("settings.menu.about") }}</div>
              <div class="q-pa-md q-gutter-sm">
                <q-scroll-area style="height: 260px">
                  <p>{{ $t("settings.menu.about.introduction") }}</p>
                  <p>
                    {{ $t("settings.menu.about.version") }}：{{ appVersion }}
                  </p>
                  <p>
                    {{ $t("settings.menu.about.website") }}：<a
                      href="javascript:void(0)"
                      style="text-decoration: none"
                      @click="openURL('https://github.com/jthinking/OllamaOne')"
                      >https://github.com/jthinking/OllamaOne</a
                    >
                  </p>
                </q-scroll-area>
              </div>
            </q-tab-panel>
          </q-tab-panels>
        </template>
      </q-splitter>

      <q-dialog v-model="dialog" position="bottom" auto-close>
        <q-card style="width: 350px">
          <q-card-section class="row items-center no-wrap">
            <div>
              <div class="text-weight-bold">
                {{ $t("settings.menu.feedback.ok") }}
              </div>
              <div class="text-grey">
                {{ $t("settings.menu.feedback.msg") }}
              </div>
            </div>
          </q-card-section>
        </q-card>
      </q-dialog>
    </div>
    <q-page-sticky position="top-right" :offset="[18, 18]">
      <q-btn fab icon="reply" color="primary" to="/" />
    </q-page-sticky>
  </q-page>
</template>

<script>
import { defineComponent, ref, onBeforeMount, computed, watch } from "vue";
import { useQuasar } from "quasar";
import languages from "quasar/lang/index.json";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { getVersion } from "@tauri-apps/api/app";
import { getName } from "@tauri-apps/api/app";
import { open } from "@tauri-apps/plugin-shell";
import { fetch } from "@tauri-apps/plugin-http";

const appLanguages = languages.filter((lang) =>
  ["en-US", "zh-CN", "zh-TW", "ja"].includes(lang.isoName)
);

const langOptions = appLanguages.map((lang) => ({
  label: lang.nativeName,
  value: lang.isoName,
}));

export default defineComponent({
  name: "SettingsPage",
  setup() {
    const appVersion = ref("-");
    const $q = useQuasar();
    const lang = ref($q.lang.isoName);
    const msgLang = ref($q.lang.isoName);
    const { locale } = useI18n({ useScope: "global" });
    const langList = import.meta.glob("../../node_modules/quasar/lang/*.mjs");
    const userTheme = ref("auto");
    const productKey = ref("");
    const ollama_url = localStorage.getItem("ollama_url");
    const ollamaURL = ref(ollama_url ? ollama_url : "http://localhost:11434");
    const deviceId = ref(null);
    const dialog = ref(false);
    const feedback = ref(null);
    const feedbackBtnDisabled = ref(false);
    const activateMsg = ref(null);

    invoke("device_id").then((id) => {
      deviceId.value = id;
    });

    const dark_theme = localStorage.getItem("dark_theme");
    if (dark_theme && dark_theme !== "[]") {
      userTheme.value = dark_theme;
    }

    const splitterStyle = computed(() => ({
      width: $q.screen.width * 0.8 + "px",
      height: "400px",
    }));

    const getAppVersion = async () => {
      const appName = await getName();
      const version = await getVersion();
      appVersion.value = appName + " " + version;
    };

    const changeTheme = (value, evt) => {
      localStorage.setItem("dark_theme", value);
      // set status
      if (value === "auto") {
        $q.dark.set("auto");
      } else if (value === "dark") {
        $q.dark.set(true);
      } else {
        $q.dark.set(false);
      }
    };

    const changeOllamaURL = async () => {
      localStorage.setItem("ollama_url", ollamaURL.value);
      // test url connection
      const response = await fetch(ollamaURL.value);
      console.log(response.status); // e.g. 200
      console.log(response.statusText); // e.g. "OK"
      const jsonData = await response.text();
      console.log(jsonData);
      if (response.status === 200) {
        activateMsg.value = jsonData;
      } else {
        activateMsg.value = response.statusText;
      }
    };

    const sendFeedback = async () => {
      await open(
        "mailto:bochao.jia@icloud.com?subject=OllamaOne issues&body=" +
          feedback.value
      );
    };

    function formatTimestamp(timestamp) {
      const date = new Date(timestamp);
      const year = date.getFullYear();
      const month = ("0" + (date.getMonth() + 1)).slice(-2);
      const day = ("0" + date.getDate()).slice(-2);
      const hours = ("0" + date.getHours()).slice(-2);
      const minutes = ("0" + date.getMinutes()).slice(-2);
      const seconds = ("0" + date.getSeconds()).slice(-2);

      const formattedDate =
        year +
        "-" +
        month +
        "-" +
        day +
        "- " +
        hours +
        ":" +
        minutes +
        ":" +
        seconds;

      return formattedDate;
    }

    const openURL = async (url) => {
      await open(url);
    };

    watch(lang, (val) => {
      localStorage.setItem("lang", val);
      locale.value = val;
      langList[`../../node_modules/quasar/lang/${val}.mjs`]().then((lang) => {
        $q.lang.set(lang.default);
      });
    });

    onBeforeMount(() => {
      getAppVersion();
    });

    return {
      appVersion,
      tab: ref("alarms"),
      splitterModel: ref(20),
      splitterStyle,
      userTheme,
      changeTheme,
      productKey,
      ollamaURL,
      changeOllamaURL,
      hex: ref("#019a9d"),
      dialog,
      feedback,
      sendFeedback,
      feedbackBtnDisabled,
      formatTimestamp,
      activateMsg,
      productKeyReadonly: ref(true),
      ollamaURLReadonly: ref(true),
      openURL,
      lang,
      langOptions,
      msgLang,
    };
  },
});
</script>
<style lang="sass" scoped>
.my-picker
  max-width: 300px
</style>
