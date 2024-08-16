<template>
  <q-page
    class="row"
    :style="{
      backgroundColor: $q.dark.isActive ? '#1e1e1e' : 'rgba(0,0,0,0.02)',
    }"
  >
    <q-carousel
      v-if="messageList.length === 0"
      v-model="slide"
      transition-prev="jump-right"
      transition-next="jump-left"
      swipeable
      animated
      control-color="primary"
      navigation
      padding
      arrows
      height="74vh"
      class="rounded-borders col self-center"
    >
      <q-carousel-slide
        :name="index"
        class="column no-wrap flex-center"
        v-for="(prompt, index) in great_prompts"
        :key="index"
      >
        <q-icon name="style" color="primary" size="56px" />
        <div class="q-mt-md text-center">
          {{ prompt }}
        </div>
        <q-btn
          flat
          color="primary"
          label="Use this Promt"
          @click="usePrompt(prompt)"
        />
      </q-carousel-slide>
    </q-carousel>
    <q-scroll-area
      ref="scrollAreaRef"
      :style="style"
      class="q-pa-md col self-center"
      v-else
    >
      <div
        v-for="(message, index) in messageList"
        :key="index"
        style="margin-right: 10px"
      >
        <q-chat-message
          v-if="message.role === 'assistant'"
          avatar="~assets/ollama.png"
          :text-color="$q.dark.isActive ? 'white' : ''"
          :bg-color="$q.dark.isActive ? 'blue-grey-10' : 'blue-grey-1'"
        >
          <div
            class="q-gutter-xs"
            v-if="currentChat.model === 'Code Interpreter'"
          >
            <q-btn
              round
              color="black"
              size="sm"
              flat
              @click="showCodeRef = !showCodeRef"
              icon="code"
            >
              <q-tooltip
                anchor="bottom middle"
                self="top middle"
                :offset="[10, 10]"
                transition-show="scale"
                transition-hide="scale"
              >
                展开代码
              </q-tooltip>
            </q-btn>
            <q-btn
              v-if="messageList.length == index + 1"
              round
              :loading="codeRunning || typing"
              color="black"
              size="sm"
              flat
              @click="runCode"
              icon="play_circle_outline"
            >
              <template v-slot:loading>
                <q-spinner-pie color="primary" />
              </template>

              <q-tooltip
                v-if="!codeRunning"
                anchor="bottom middle"
                self="top middle"
                :offset="[10, 10]"
                transition-show="scale"
                transition-hide="scale"
              >
                运行代码
              </q-tooltip>
              <q-tooltip
                v-if="codeRunning"
                anchor="bottom middle"
                self="top middle"
                :offset="[10, 10]"
                transition-show="scale"
                transition-hide="scale"
              >
                正在执行代码
              </q-tooltip>
            </q-btn>
            <q-btn
              v-if="!typing && messageList.length == index + 1"
              round
              color="white"
              :text-color="$q.dark.isActive ? 'white' : 'black'"
              size="sm"
              icon="replay"
              flat
              @click="reChat"
            >
              <q-tooltip
                anchor="bottom middle"
                self="top middle"
                :offset="[10, 10]"
                transition-show="scale"
                transition-hide="scale"
              >
                {{ $t("chat.msg.tool.retry_chat") }}
              </q-tooltip>
            </q-btn>
            <q-btn
              v-if="typing && messageList.length == index + 1"
              round
              color="white"
              :text-color="$q.dark.isActive ? 'white' : 'black'"
              size="sm"
              icon="stop_circle"
              flat
              @click="stopChat"
            >
              <q-tooltip
                anchor="bottom middle"
                self="top middle"
                :offset="[10, 10]"
                transition-show="scale"
                transition-hide="scale"
              >
                {{ $t("chat.msg.tool.stop_chat") }}
              </q-tooltip>
            </q-btn>
          </div>
          <MdPreview
            editorId="preview-only"
            :modelValue="message.content"
            :theme="$q.dark.isActive ? 'dark' : 'light'"
            :style="editorStyle"
          />
          <pre
            v-if="stdout != '' && messageList.length == index + 1"
            style="padding: 0; margin: 0"
          >
            {{ stdout }}
          </pre>

          <template v-slot:name>
            <div class="q-gutter-xs">
              <q-btn
                dense
                flat
                color="primary"
                size="sm"
                icon="file_download"
                @click="downloadMarkdown(message)"
              >
                <q-tooltip
                  anchor="top middle"
                  self="bottom middle"
                  :offset="[10, 10]"
                  transition-show="scale"
                  transition-hide="scale"
                >
                  {{
                    mdDownloaded
                      ? $t("chat.msg.tool.downloaded")
                      : $t("chat.msg.tool.download")
                  }}
                </q-tooltip>
              </q-btn>
              <q-btn
                dense
                flat
                color="primary"
                size="sm"
                icon="content_copy"
                @click="copyMarkdown(message)"
              >
                <q-tooltip
                  anchor="top middle"
                  self="bottom middle"
                  :offset="[10, 10]"
                  transition-show="scale"
                  transition-hide="scale"
                >
                  {{
                    mdCopied
                      ? $t("chat.msg.tool.copied")
                      : $t("chat.msg.tool.copy")
                  }}
                </q-tooltip>
              </q-btn>
              <!-- <q-btn
                dense
                flat
                color="primary"
                size="sm"
                icon="star_border"
                @click="downloadMarkdown(message)"
              >
                <q-tooltip
                  anchor="top middle"
                  self="bottom middle"
                  :offset="[10, 10]"
                  transition-show="scale"
                  transition-hide="scale"
                >
                  {{ $t("chat.msg.tool.download") }}
                </q-tooltip>
              </q-btn> -->
            </div>
          </template>
          <template
            v-slot:stamp
            v-if="
              messageList.length == index + 1 &&
              currentChat.model !== 'Code Interpreter'
            "
          >
            <div class="q-gutter-xs">
              <q-btn
                v-if="!typing"
                round
                color="white"
                :text-color="$q.dark.isActive ? 'white' : 'black'"
                size="sm"
                icon="replay"
                flat
                @click="reChat"
              >
                <q-tooltip
                  anchor="bottom middle"
                  self="top middle"
                  :offset="[10, 10]"
                  transition-show="scale"
                  transition-hide="scale"
                >
                  {{ $t("chat.msg.tool.retry_chat") }}
                </q-tooltip>
              </q-btn>
              <q-btn
                v-if="typing"
                round
                color="white"
                :text-color="$q.dark.isActive ? 'white' : 'black'"
                size="sm"
                icon="stop_circle"
                flat
                @click="stopChat"
              >
                <q-tooltip
                  anchor="bottom middle"
                  self="top middle"
                  :offset="[10, 10]"
                  transition-show="scale"
                  transition-hide="scale"
                >
                  {{ $t("chat.msg.tool.stop_chat") }}
                </q-tooltip>
              </q-btn>
            </div>
          </template>
        </q-chat-message>
        <q-chat-message
          v-if="message.role === 'user'"
          sent
          text-color="white"
          bg-color="light-blue-9"
        >
          <div :style="sentStyle">
            <q-markdown
              :key="index"
              v-model:src="message.content"
              :no-html="true"
              :no-link="false"
              :no-linkify="false"
              :no-typographer="false"
              :no-breaks="false"
              :no-highlight="false"
              :no-image="false"
              :no-container="false"
              :no-line-numbers="true"
              :plugins="markdownPlugins"
              class="bordered"
            />
          </div>
          <template v-slot:name>
            <div class="q-gutter-xs">
              <q-btn
                v-if="messageList.length == index + 2 && !typing"
                dense
                flat
                color="primary"
                size="sm"
                icon="edit"
                @click="editMessage()"
              >
                <q-tooltip
                  anchor="top middle"
                  self="bottom middle"
                  :offset="[10, 10]"
                  transition-show="scale"
                  transition-hide="scale"
                >
                  {{ $t("chat.msg.tool.edit") }}
                </q-tooltip>
              </q-btn>
              <q-btn
                dense
                flat
                color="primary"
                size="sm"
                icon="file_download"
                @click="downloadMarkdown(message)"
              >
                <q-tooltip
                  anchor="top middle"
                  self="bottom middle"
                  :offset="[10, 10]"
                  transition-show="scale"
                  transition-hide="scale"
                >
                  {{
                    mdDownloaded
                      ? $t("chat.msg.tool.downloaded")
                      : $t("chat.msg.tool.download")
                  }}
                </q-tooltip>
              </q-btn>
              <q-btn
                dense
                flat
                color="primary"
                size="sm"
                icon="content_copy"
                @click="copyMarkdown(message)"
              >
                <q-tooltip
                  anchor="top middle"
                  self="bottom middle"
                  :offset="[10, 10]"
                  transition-show="scale"
                  transition-hide="scale"
                >
                  {{
                    mdCopied
                      ? $t("chat.msg.tool.copied")
                      : $t("chat.msg.tool.copy")
                  }}
                </q-tooltip>
              </q-btn>
            </div>
          </template>
        </q-chat-message>
      </div>
      <q-chat-message
        v-if="receiving"
        avatar="~assets/ollama.png"
        :bg-color="$q.dark.isActive ? 'blue-grey-10' : 'blue-grey-1'"
      >
        <q-spinner-dots size="2rem" color="primary" />
      </q-chat-message>
    </q-scroll-area>

    <q-page-sticky position="top-left" :offset="[10, 10]">
      <q-btn
        color="primary"
        :label="currentChat.model"
        rounded
        no-caps
        size="md"
        padding="none md"
      >
        <q-menu :offset="[0, 5]">
          <q-list dense style="min-width: 100px">
            <q-item clickable>
              <q-item-section>llama2</q-item-section>
              <q-item-section side>
                <q-icon name="keyboard_arrow_right" />
              </q-item-section>

              <q-menu anchor="top end" self="top start">
                <q-list>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('llama2:latest')"
                  >
                    <q-item-section>llama2:latest</q-item-section>
                  </q-item>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('llama2:7b')"
                  >
                    <q-item-section>llama2:7b</q-item-section>
                  </q-item>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('llama2:13b')"
                  >
                    <q-item-section>llama2:13b</q-item-section>
                  </q-item>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('llama2:70b')"
                  >
                    <q-item-section>llama2:70b</q-item-section>
                  </q-item>
                </q-list>
              </q-menu>
            </q-item>

            <q-item clickable>
              <q-item-section>qwen</q-item-section>
              <q-item-section side>
                <q-icon name="keyboard_arrow_right" />
              </q-item-section>

              <q-menu anchor="top end" self="top start">
                <q-list>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('qwen:latest')"
                  >
                    <q-item-section>qwen:latest</q-item-section>
                  </q-item>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('qwen:4b')"
                  >
                    <q-item-section>qwen:4b</q-item-section>
                  </q-item>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('qwen:7b')"
                  >
                    <q-item-section>qwen:7b</q-item-section>
                  </q-item>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('qwen:14b')"
                  >
                    <q-item-section>qwen:14b</q-item-section>
                  </q-item>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('qwen:72b')"
                  >
                    <q-item-section>qwen:72b</q-item-section>
                  </q-item>
                </q-list>
              </q-menu>
            </q-item>

            <q-item clickable>
              <q-item-section>mistral</q-item-section>
              <q-item-section side>
                <q-icon name="keyboard_arrow_right" />
              </q-item-section>

              <q-menu anchor="top end" self="top start">
                <q-list>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('mistral:latest')"
                  >
                    <q-item-section>mistral:latest</q-item-section>
                  </q-item>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('mistral:7b')"
                  >
                    <q-item-section>mistral:7b</q-item-section>
                  </q-item>
                </q-list>
              </q-menu>
            </q-item>

            <q-item clickable>
              <q-item-section>codellama</q-item-section>
              <q-item-section side>
                <q-icon name="keyboard_arrow_right" />
              </q-item-section>

              <q-menu anchor="top end" self="top start">
                <q-list>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('codellama:latest')"
                  >
                    <q-item-section>codellama:latest</q-item-section>
                  </q-item>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('codellama:7b')"
                  >
                    <q-item-section>codellama:7b</q-item-section>
                  </q-item>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('codellama:13b')"
                  >
                    <q-item-section>codellama:13b</q-item-section>
                  </q-item>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('codellama:34b')"
                  >
                    <q-item-section>codellama:34b</q-item-section>
                  </q-item>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('codellama:70b')"
                  >
                    <q-item-section>codellama:70b</q-item-section>
                  </q-item>
                </q-list>
              </q-menu>
            </q-item>

            <q-item clickable>
              <q-item-section>gemma</q-item-section>
              <q-item-section side>
                <q-icon name="keyboard_arrow_right" />
              </q-item-section>

              <q-menu anchor="top end" self="top start">
                <q-list>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('gemma:latest')"
                  >
                    <q-item-section>gemma:latest</q-item-section>
                  </q-item>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('gemma:2b')"
                  >
                    <q-item-section>gemma:2b</q-item-section>
                  </q-item>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('gemma:7b')"
                  >
                    <q-item-section>gemma:7b</q-item-section>
                  </q-item>
                </q-list>
              </q-menu>
            </q-item>

            <q-item clickable>
              <q-item-section>yi</q-item-section>
              <q-item-section side>
                <q-icon name="keyboard_arrow_right" />
              </q-item-section>

              <q-menu anchor="top end" self="top start">
                <q-list>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('yi:latest')"
                  >
                    <q-item-section>yi:latest</q-item-section>
                  </q-item>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('yi:6b')"
                  >
                    <q-item-section>yi:6b</q-item-section>
                  </q-item>
                  <q-item
                    dense
                    clickable
                    v-close-popup
                    @click="selectModel('yi:34b')"
                  >
                    <q-item-section>yi:34b</q-item-section>
                  </q-item>
                </q-list>
              </q-menu>
            </q-item>
            <!-- <q-separator />
            <q-item
              clickable
              v-close-popup
              @click="
                () => {
                  changeCurrentModel('Code Interpreter');
                  clearHistory();
                  permitFolder();
                }
              "
            >
              <q-item-section
                ><q-item-label
                  >Code Interpreter
                  <q-badge color="red" align="middle"
                    >Preview</q-badge
                  ></q-item-label
                ></q-item-section
              >
            </q-item> -->
          </q-list>
        </q-menu>
      </q-btn>
    </q-page-sticky>
  </q-page>
  <q-footer>
    <q-toolbar
      :class="
        $q.dark.isActive ? 'bg-dart-bar row' : 'bg-light-bar text-black row'
      "
    >
      <!-- <q-btn round flat icon="insert_emoticon" class="q-mr-sm" /> -->

      <q-btn round flat icon="history" class="q-mr-sm" @click="showHistory">
        <q-tooltip anchor="top middle" self="bottom middle" :offset="[10, 10]">
          <strong>{{ $t("chat.input.history") }}</strong>
        </q-tooltip>
      </q-btn>
      <q-btn round flat icon="refresh" class="q-mr-sm" @click="clearHistory">
        <q-tooltip anchor="top middle" self="bottom middle" :offset="[10, 10]">
          <strong>{{ $t("chat.input.clear") }}</strong>
        </q-tooltip>
      </q-btn>
      <q-btn round flat icon="share" class="q-mr-sm" @click="shareHistory">
        <q-tooltip anchor="top middle" self="bottom middle" :offset="[10, 10]">
          <strong>{{ $t("chat.input.share") }}</strong>
        </q-tooltip>
      </q-btn>

      <!-- <q-fab
        color="primary"
        rounded
        flat
        icon="auto_awesome"
        direction="up"
        padding="xs"
        class="q-mr-sm"
      >
        <q-fab-action
          external-label
          label-position="left"
          color="primary"
          icon="translate"
          label="翻译"
        />
        <q-fab-action
          external-label
          label-position="left"
          color="primary"
          icon="psychology"
          label="讲解"
        />
        <q-fab-action
          external-label
          label-position="left"
          color="primary"
          icon="travel_explore"
          label="搜索"
        />
        <q-fab-action
          external-label
          label-position="left"
          color="primary"
          icon="school"
          label="教学"
        />
        <q-fab-action
          external-label
          label-position="left"
          color="primary"
          icon="emoji_objects"
          label="灵感"
        />
      </q-fab> -->

      <q-input
        rounded
        outlined
        dense
        autofocus
        type="textarea"
        rows="1"
        class="WAL__field col-grow q-mr-sm"
        v-model="inputMessage"
        :placeholder="$t('chat.input.tip')"
        @keydown.enter.exact="sendMessage"
      />

      <q-btn
        v-if="currentChat.model === 'Code Interpreter'"
        round
        flat
        icon="file_open"
        color="primary"
        class="q-mr-sm"
        @click="addFile"
      >
        <q-tooltip anchor="top middle" self="bottom middle" :offset="[10, 10]">
          <strong>{{ $t("chat.input.add_file") }}</strong>
        </q-tooltip>
      </q-btn>
      <q-btn round flat icon="send" color="primary" @click="sendMessage">
        <q-tooltip anchor="top middle" self="bottom middle" :offset="[10, 10]">
          <strong>{{ $t("chat.input.send") }}</strong>
        </q-tooltip>
      </q-btn>
      <!-- <q-btn round flat icon="mic" /> -->
    </q-toolbar>
  </q-footer>
  <q-drawer
    side="right"
    v-model="drawerRight"
    bordered
    :width="210"
    :breakpoint="500"
    :class="$q.dark.isActive ? 'bg-grey-9' : 'bg-grey-3'"
  >
    <q-scroll-area class="fit">
      <!-- <q-toolbar class="bg-grey-2">
        <q-input
          rounded
          outlined
          dense
          class="WAL__field full-width"
          bg-color="white"
          v-model="search"
          placeholder="Search"
        >
          <template v-slot:prepend>
            <q-icon name="search" />
          </template>
        </q-input>
      </q-toolbar> -->
      <q-list padding>
        <q-item-label header>{{ $t("chat.input.history") }}</q-item-label>

        <q-item
          clickable
          v-ripple
          v-for="(history, index) in historyList"
          :key="index"
        >
          <q-item-section @click="historyClick(history)">
            <q-item-label>{{ history.title }}</q-item-label>
            <q-item-label caption>
              {{ history.model }}
            </q-item-label>
          </q-item-section>
          <q-item-section top side>
            <div class="text-grey-8 q-gutter-xs">
              <q-btn
                class="gt-xs"
                size="12px"
                flat
                dense
                round
                icon="delete"
                @click="handleHistoryDeleteClick(history)"
              />
            </div>
          </q-item-section>
        </q-item>
      </q-list>
    </q-scroll-area>
  </q-drawer>

  <!-- delete history -->
  <q-dialog
    v-model="deleteConfirm"
    persistent
    @hide="
      () => {
        deleteCache = null;
      }
    "
  >
    <q-card style="width: 300px">
      <q-card-section>
        <div class="text-h6">{{ $t("chat.input.history.delete.title") }}</div>
      </q-card-section>

      <q-card-section class="q-pt-none">
        {{ $t("chat.input.history.delete.description") }}
      </q-card-section>

      <q-card-actions align="right" class="bg-white text-teal">
        <q-btn
          flat
          :label="$t('chat.input.history.delete.cancel')"
          color="primary"
          v-close-popup
        />
        <q-btn
          flat
          :label="$t('chat.input.history.delete.confirm')"
          color="primary"
          @click="historyDelete"
        />
      </q-card-actions>
    </q-card>
  </q-dialog>

  <!-- Model Download Dialog -->
  <q-dialog v-model="modelDownloadDlg" persistent>
    <q-card style="width: 350px">
      <q-linear-progress
        v-if="downloadStatus.completed > 0"
        :value="downloadStatus.completed / downloadStatus.total"
        color="primary"
      />

      <q-card-section class="row items-center no-wrap">
        <div>
          <div class="text-weight-bold">Download {{ currentChat.model }}</div>
          <div class="text-grey">{{ downloadStatus.status }}</div>
          <div class="text-grey" v-if="downloadStatus.total">
            Total: {{ formatBytes(downloadStatus.total) }}
          </div>
          <div class="text-grey" v-if="downloadStatus.completed">
            Completed: {{ formatBytes(downloadStatus.completed) }}
          </div>
        </div>

        <q-space />

        <q-btn
          flat
          round
          icon="cloud_download"
          color="primary"
          :loading="modelDownloading"
          @click="doDownloadModel(currentChat.model)"
        />
        <q-btn flat round icon="close" v-close-popup />
      </q-card-section>
    </q-card>
  </q-dialog>

  <!-- File save Dialog -->
  <q-dialog v-model="fileSaveDlg" persistent>
    <q-card>
      <q-toolbar>
        <q-avatar icon="share" color="primary" text-color="white" />

        <q-toolbar-title>{{ $t("chat.input.save") }}</q-toolbar-title>

        <q-btn flat round dense icon="close" v-close-popup />
      </q-toolbar>

      <q-card-section class="q-pt-none">
        <div class="q-pa-md" style="width: 300px">
          <div class="q-gutter-md">
            <q-input
              outlined
              v-model="fileSaveName"
              :label="$t('chat.input.save.name')"
            />
            <q-select
              outlined
              v-model="fileSaveFormat"
              :options="['HTML', 'Markdown']"
              :label="$t('chat.input.save.format')"
            />
            <q-select
              outlined
              v-model="fileSaveLocation"
              :options="[
                {
                  label: $t('chat.input.save.location.desktop'),
                  value: 'Desktop',
                },
                {
                  label: $t('chat.input.save.location.document'),
                  value: 'Document',
                },
                {
                  label: $t('chat.input.save.location.download'),
                  value: 'Download',
                },
              ]"
              :label="$t('chat.input.save.location')"
            />
          </div>
        </div>
      </q-card-section>
      <q-separator />
      <q-card-actions align="right">
        <q-btn
          flat
          :label="$t('chat.input.save')"
          color="primary"
          v-close-popup
          @click="saveTextFile()"
        />
      </q-card-actions>
    </q-card>
  </q-dialog>

  <!-- ollama connect test -->
  <q-dialog v-model="ollamaConnectDlg" persistent>
    <q-card style="width: 80vw">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">{{ $t("ollama.title") }}</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-card-section>
        {{ $t("ollama.description") }}
      </q-card-section>
      <q-card-section>
        <q-input bottom-slots outlined v-model="ollamaURL" label="Ollama URL">
          <template v-slot:hint>
            {{ ollamaConnectMsg }}
          </template>
          <template v-slot:append>
            <q-icon
              name="warning"
              color="warning"
              v-if="ollamaConnectMsg.indexOf('Ollama is running') === -1"
            />
            <q-icon name="check_circle" color="positive" v-else />
          </template>
        </q-input>
      </q-card-section>

      <q-card-actions class="text-primary" style="padding: 16px">
        <q-btn
          no-caps
          color="primary"
          :label="$t('ollama.download')"
          icon="install_desktop"
          @click="
            () => {
              openURL('https://ollama.com/download');
            }
          "
        />
        <q-space />
        <q-btn
          no-caps
          flat
          :label="$t('ollama.skip')"
          @click="
            () => {
              ollamaConnectDlg = false;
            }
          "
        />
        <q-btn
          no-caps
          outline
          :label="$t('ollama.connect')"
          @click="connectOllama"
        />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script>
import {
  defineComponent,
  computed,
  ref,
  nextTick,
  onMounted,
  onBeforeUnmount,
} from "vue";
import { useQuasar } from "quasar";
import { invoke } from "@tauri-apps/api/core";
import {
  desktopDir,
  documentDir,
  downloadDir,
  homeDir,
} from "@tauri-apps/api/path";
import { listen, emit } from "@tauri-apps/api/event";
import { writeText, readText } from "@tauri-apps/plugin-clipboard-manager";
import { writeTextFile, BaseDirectory } from "@tauri-apps/plugin-fs";
import { ionCartOutline } from "@quasar/extras/ionicons-v6";
import { fabMarkdown, fasFileCirclePlus } from "@quasar/extras/fontawesome-v6";
import { ionDocumentAttachOutline } from "@quasar/extras/ionicons-v6";
import showdown from "showdown";
import { convert } from "html-to-text";
import TurndownService from "turndown";
import { Window } from "@tauri-apps/api/window";

// loading embedded asset:
const appWindow = new Window("main");
import { Command, open } from "@tauri-apps/plugin-shell";
import { QMarkdown } from "@quasar/quasar-ui-qmarkdown";
import "@quasar/quasar-ui-qmarkdown/dist/index.css";
import abbreviation from "markdown-it-abbr";
import deflist from "markdown-it-deflist";
import emoji from "markdown-it-emoji";
import footnote from "markdown-it-footnote";
import insert from "markdown-it-ins";
import mark from "markdown-it-mark";
import subscript from "markdown-it-sub";
import superscript from "markdown-it-sup";
import taskLists from "markdown-it-task-lists";
import mermaid from "@datatraccorporation/markdown-it-mermaid";
import { asyncRun, onStdout, onStderr } from "./py-worker";
import { v4 as uuidv4 } from "uuid";
import { Message } from "@arco-design/web-vue";
import { showModel } from "../service/models";
import {
  listChats,
  deleteChat,
  getChat,
  updateChat,
  createChat,
} from "../service/chat";
import { crawlUrl } from "../service/crawler";
import {
  convertChatToMarkdown,
  formatBytes,
  convertChatToHTML,
  getCurrentDate,
  formatDate,
} from "../utils/format";
import { fetch } from "@tauri-apps/plugin-http";
import { MdPreview, MdCatalog } from "md-editor-v3";
import "md-editor-v3/lib/preview.css";
import { useI18n } from "vue-i18n";

const great_prompts = [
  "Compose a blog post of 1500 words from the perspective of a health professional on the importance of a balanced diet and regular exercise. Use engaging language and include practical tips for the readers to apply in their daily lives.",
  "Analyze the current state of <industry> and its trends, challenges, and opportunities, including relevant data and statistics. Provide a list of key players and a short and long-term industry forecast, and explain any potential impact of current events or future developments.",
  `I want you to act as a storyteller. You will come up with entertaining stories that are engaging, imaginative and captivating for the audience. It can be fairy tales, educational stories or any other type of stories which has the potential to capture people's attention and imagination. Depending on the target audience, you may choose specific themes or topics for your storytelling session e.g., if it’s children then you can talk about animals; If it’s adults then history-based tales might engage them better etc. My first request is "I need an interesting story on perseverance."`,
  `I want you to act as a math teacher. I will provide some mathematical equations or concepts, and it will be your job to explain them in easy-to-understand terms. This could include providing step-by-step instructions for solving a problem, demonstrating various techniques with visuals or suggesting online resources for further study. My first request is "I need help understanding how probability works."`,
];

export default defineComponent({
  name: "ChatPage",
  components: {
    MdPreview,
  },
  setup() {
    const $q = useQuasar();

    const { t } = useI18n();
    const receiving = ref(false);
    const codeRunning = ref(false);
    const showCodeRef = ref(false);
    const scrollElement = document.documentElement;
    const modelDownloadDlg = ref(false);
    const downloadStatus = ref({
      status: "model not found, download now?",
    });
    const modelDownloading = ref(false);
    const fileSaveDlg = ref(false);
    const ollamaConnectDlg = ref(false);
    const fileSaveName = ref("Untitled.html");
    const fileSaveFormat = ref("HTML");
    const fileSaveLocation = ref({
      value: "Desktop",
      label: t("chat.input.save.location.desktop"),
    });

    const mdCopied = ref(false);
    const mdDownloaded = ref(false);

    const markdownPlugins = ref([
      abbreviation,
      deflist,
      emoji,
      footnote,
      insert,
      mark,
      subscript,
      superscript,
      taskLists,
      mermaid,
    ]);

    const ollama_url = localStorage.getItem("ollama_url");
    const ollamaURL = ref(ollama_url ? ollama_url : "http://localhost:11434");

    const ollamaConnectMsg = ref("");

    const style = computed(() => ({
      height: $q.screen.height - 100 + "px",
      paddingRight: "15px",
    }));

    const mdBgColor = computed(() => ($q.dark.isActive ? "#444" : "#f0f0f0"));

    const editorStyle = computed(() => ({
      // minHeight: "0px",
      // minWidth: "0px",
      maxWidth: $q.screen.width * 0.7 + "px",
      //fontSize: "15px",
      //color: $q.dark.isActive ? "#fff" : "#000",
      //padding: "0px",
      //backgroundColor: $q.dark.isActive ? "#000" : "#fff",
    }));

    const sentStyle = computed(() => ({
      fontSize: "15px",
      lineHeight: 1.5,
      wordWrap: "break-word",
      maxWidth: $q.screen.width * 0.7 + "px",
    }));

    const inputMessage = ref(null);
    const messageList = ref([]);

    const historyList = ref([]);
    const scrollAreaRef = ref(null);
    const drawerRight = ref(false);
    const typing = ref(false);
    const deleteConfirm = ref(false);
    const deleteCache = ref(null);
    const stdout = ref("");

    const currentModel = localStorage.getItem("currentModel");
    const currentChat = ref({
      id: null,
      title: null,
      model: currentModel ? currentModel : "llama2:latest",
    });

    const changeCurrentModel = (model) => {
      currentChat.value.model = model;
      localStorage.setItem("currentModel", model);
    };

    async function doChat() {
      stdout.value = "";

      const messages = messageList.value
        .filter((item) => !item.error)
        .map((item) => {
          return {
            role: item.role,
            content: item.parsedContent ? item.parsedContent : item.content,
          };
        });

      // "后续对话遵循如下要求：1. 你是一个Code Interpreter，按照要求写出python代码，不要为自己不能做的事情道歉；2. 首先输出逻辑代码，输出一个完整的代码块，不要分步输出，不要有过多的文字描述，代码中涉及文件读写操作时，相对于/mount/目录操作；3. 如果逻辑代码包括除标准库之外的模块，输出所需的模块的安装代码，使用如下python代码安装所需模块：\nimport micropip\nawait micropip.install('模块名')\n，如果是标准库模块则不输出；"
      if (currentChat.value.model === "Code Interpreter") {
        messages.unshift({
          role: "system",
          content:
            "后续对话遵循如下要求：1. 按照要求写出python代码，python代码必须使用markdown格式，不要为自己不能做的事情道歉；2. 首先输出逻辑代码，输出一个完整的代码块，不要分步输出，不要有过多的文字描述，代码中涉及文件读写操作时，相对于/mount/目录操作；3. 如果逻辑代码包括除标准库之外的模块，输出所需的模块的安装代码，使用如下python代码安装所需模块：\nimport micropip\nawait micropip.install('模块名')\n，如果是标准库模块则不输出；",
        });
      }

      console.log(messages);

      const fetchId = uuidv4();
      let unlisten;

      listen("fetch-stream-chunk", (event) => {
        const payload = event.payload;
        if (payload.id !== fetchId) {
          return;
        }
        if (payload.done) {
          console.log(payload);
          typing.value = false;
          // runCode();
          updateMessages();
          return;
        }
        if (!typing.value) {
          unlisten?.();
          emit("abort-fetch-stream", { id: fetchId });
          updateMessages();
        }
        if (receiving.value) {
          receiving.value = false;
          messageList.value.push({
            role: "assistant",
            content: "",
          });
        }

        if (payload.status !== 200) {
          try {
            const data = JSON.parse(payload.data);
            console.log(data);
          } catch (e) {
            console.error(e);
          }
          return;
        }
        try {
          const data = JSON.parse(payload.data);
          messageList.value[messageList.value.length - 1].content +=
            data?.message?.content;
          nextTick(() => {
            scrollToBottom();
          });
        } catch (e) {
          console.error(e);
        }
      })
        .then((cb) => {
          unlisten = cb;
        })
        .catch((e) => {
          console.error(e);
        });

      invoke("fetch_stream", {
        id: fetchId,
        url: `${ollamaURL.value}/api/chat`,
        optionsStr: JSON.stringify({
          method: "POST",
          headers: {},
          body: JSON.stringify({
            model: currentChat.value.model,
            messages: messages,
            stream: true,
          }),
        }),
      })
        .catch((e) => {
          console.error(e);
        })
        .finally(() => {
          unlisten?.();
        });

      nextTick(() => {
        scrollToBottom();
      });
    }

    async function doDownloadModel(model) {
      modelDownloading.value = true;
      const fetchId = uuidv4();
      let unlisten;

      listen("fetch-stream-chunk", (event) => {
        const payload = event.payload;
        if (payload.id !== fetchId) {
          return;
        }
        if (payload.done) {
          console.log(payload);
          modelDownloading.value = false;
          return;
        }
        if (payload.status !== 200) {
          try {
            const data = JSON.parse(payload.data);
            console.log(data);
          } catch (e) {
            console.error(e);
          }
          return;
        }
        try {
          const data = JSON.parse(payload.data);
          downloadStatus.value = data;
        } catch (e) {
          console.error(e);
        }
      })
        .then((cb) => {
          unlisten = cb;
        })
        .catch((e) => {
          console.error(e);
        });

      invoke("fetch_stream", {
        id: fetchId,
        url: `${ollamaURL.value}/api/pull`,
        optionsStr: JSON.stringify({
          method: "POST",
          headers: {},
          body: JSON.stringify({
            name: model,
            stream: true,
          }),
        }),
      })
        .catch((e) => {
          console.error(e);
        })
        .finally(() => {
          unlisten?.();
        });
    }

    const clearHistory = async () => {
      stopChat();
      messageList.value = [];
      currentChat.value = {
        id: null,
        title: null,
        model: currentChat.value.model,
      };
    };

    const showHistory = async () => {
      drawerRight.value = !drawerRight.value;
      if (drawerRight.value) {
        await queryHistory();
      } else {
        historyList.value = [];
      }
    };

    const closeHistory = async () => {
      console.log("closeHistory");
      drawerRight.value = false;
      historyList.value = [];
    };

    const queryHistory = async () => {
      const chats = await listChats({ current: 1, pageSize: 999999 });
      console.log(chats);
      historyList.value = chats;
    };

    const historyClick = async (history) => {
      if (currentChat.value.id == history?.id) {
        return;
      }
      const chat = await getChat(history);
      console.log(chat);
      currentChat.value = chat;
      messageList.value = JSON.parse(chat.messages);
      nextTick(() => {
        scrollToBottom();
      });
    };

    const updateMessages = async () => {
      const result = await updateChat({
        ...currentChat.value,
        messages: messageList.value,
      });
      console.log(result);
    };

    const newChat = async () => {
      const result = await createChat({
        model: currentChat.value.model,
        messages: messageList.value,
      });
      currentChat.value = result.data;
      console.log(result);
    };

    const historyDelete = async () => {
      let history = deleteCache.value;
      if (history) {
        await deleteChat(history);
        await queryHistory();
        deleteCache.value = null;
      }
      deleteConfirm.value = false;
    };

    const handleHistoryDeleteClick = async (history) => {
      deleteConfirm.value = true;
      deleteCache.value = history;
    };

    const sendMessage = async (event) => {
      if (event) {
        event.preventDefault();
      }
      if (!inputMessage.value || inputMessage.value.trim() === "") {
        inputMessage.value = null;
        return;
      }
      if (typing.value) {
        return;
      }
      const result = await showModel(currentChat.value.model);
      if (result === undefined) {
        showModelDownloadDlg(currentChat.value.model);
        return;
      }

      messageList.value.push({
        role: "user",
        content: inputMessage.value,
        parsedContent: "",
      });

      inputMessage.value = null;

      typing.value = true;
      receiving.value = true;

      // parse url content
      const parsedContent = await crawlURLText(
        messageList.value[messageList.value.length - 1].content
      );
      console.log(parsedContent);
      messageList.value[messageList.value.length - 1].parsedContent =
        parsedContent;

      // try new chat
      if (!currentChat.value.id) {
        await newChat();
      }

      nextTick(() => {
        scrollToBottom();
      });
      await doChat();
    };

    const matchURL = (msg) => {
      // 从msg中提取链接
      let reg = /((http|https):\/\/)([\w.]+\/?)\S*/;
      let url = msg.match(reg);
      if (url) {
        return url[0];
      } else {
        return null;
      }
    };

    const crawlURLText = async (msg) => {
      // 从msg中提取链接
      const url = matchURL(msg);
      if (!url) {
        return undefined;
      }
      try {
        const res = await crawlUrl(url);
        if (res === "") {
          return undefined;
        }
        // var turndownService = new TurndownService();
        // var markdown = turndownService.turndown(res);
        const text = convert(res, {
          selectors: [
            { selector: "a", options: { ignoreHref: true } },
            { selector: "img", format: "skip" },
          ],
        });
        let result = msg.replace(url, text);
        return result;
      } catch (error) {
        return msg;
      }
    };

    const scrollToBottom = () => {
      let verticalSize = scrollAreaRef.value?.getScroll().verticalSize;
      scrollAreaRef.value?.setScrollPosition("vertical", verticalSize + 9999);
    };

    const copyMarkdown = async (message) => {
      await writeText(message.content);
      mdCopied.value = true;
      setTimeout(() => {
        mdCopied.value = false;
      }, 1000);
    };

    const downloadMarkdown = async (message) => {
      const filename = Date.now() + ".md";
      downloadTextFile(message.content, filename);
      mdDownloaded.value = true;
      setTimeout(() => {
        mdDownloaded.value = false;
      }, 1000);
      await openFileNotify(filename, BaseDirectory.Desktop);
    };

    /**
     * 将文本转化为文件并下载
     * @param {string} text 文件内容
     * @param {string} fileName 文件名
     */
    async function downloadTextFile(text, fileName) {
      await writeTextFile(fileName, text, {
        baseDir: BaseDirectory.Desktop,
      });
    }

    const closeWindow = () => {
      appWindow.close();
    };

    const openURL = async (url) => {
      await open(url);
    };

    nextTick(() => {
      scrollToBottom();
    });

    currentChat.value.title = getCurrentDate();

    const stopChat = () => {
      typing.value = false;
    };

    const reChat = () => {
      typing.value = true;
      receiving.value = true;
      messageList.value.pop();
      doChat();
    };

    const editMessage = () => {
      messageList.value.pop();
      let lastSent = messageList.value.pop();
      inputMessage.value = lastSent.content;
    };

    const unlistenRef = ref(null);
    onMounted(async () => {
      unlistenRef.value = await appWindow.onResized(({ payload: size }) => {
        console.log("Window resized", size);
        closeHistory();
      });

      // 检查Code Interpreter目录挂载情况
      if (currentChat.value.model === "Code Interpreter" && dirHandle == null) {
        permitFolder();
      }
    });

    onBeforeUnmount(() => {
      stopChat();
      closeHistory();
      if (unlistenRef.value) {
        // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted
        const unlisten = unlistenRef.value;
        unlisten();
      }
    });

    const scriptCode = ref(`
    import statistics
    from js import A_rank
    statistics.stdev(A_rank)
`);

    const codeRunResult = ref(null);

    const contextRef = ref({});

    let dirHandle = null;

    async function interpreter() {
      try {
        const { results, error } = await asyncRun(
          scriptCode.value,
          contextRef.value,
          dirHandle
        );
        if (results) {
          console.log("pyodideWorker return results: ", results);
          codeRunResult.value = results;
          stdout.value = stdout.value + "\n" + results;
        } else if (error) {
          console.log("pyodideWorker error: ", error);
          stdout.value = stdout.value + "\n" + error;
        }
      } catch (e) {
        console.log(
          `Error in pyodideWorker at ${e.filename}, Line: ${e.lineno}, ${e.message}`
        );
      }
    }

    onStdout((data) => {
      console.log("onStdout", data);
      stdout.value = stdout.value + "\n" + data.msg;
    });
    onStderr((data) => {
      console.log("onStderr", data);
      stdout.value = stdout.value + "\n" + data.msg;
    });

    const permitFolder = async () => {
      $q.dialog({
        title: "权限申请",
        message:
          "Code Interpreter本地化代码解释器功能需要申请使用本地目录，用于读取和存放代码运行所需的文件。<br><strong>说明：本地化代码解释器不会将您的文件上传到云端。</strong>",
        ok: {
          label: "选择目录",
          flat: true,
          icon: "folder_open",
        },
        cancel: {
          label: "取消",
          flat: true,
        },
        persistent: true,
        html: true,
      })
        .onOk(async () => {
          dirHandle = await showDirectoryPicker();

          if (
            (await dirHandle.queryPermission({ mode: "readwrite" })) !==
            "granted"
          ) {
            if (
              (await dirHandle.requestPermission({ mode: "readwrite" })) !==
              "granted"
            ) {
              throw Error("Unable to read and write directory");
            }
          }
        })
        .onCancel(() => {
          // console.log('>>>> Cancel')
        })
        .onDismiss(() => {
          // console.log('I am triggered on both OK and Cancel')
        });
    };

    const runCode = async () => {
      if (currentChat.value.model !== "Code Interpreter") {
        return;
      }
      let content = messageList.value[messageList.value.length - 1].content;
      //let matched = content.match(/\n```[pP]ython\n([\s\S]*?)\n```\n/g);
      let matched = Array.from(
        content.matchAll(/```[pP]ython\n([\s\S]*?)\n```/g)
      );
      codeRunning.value = true;

      setTimeout(async () => {
        let installCode = matched
          .map((m) => m[1])
          .filter((m) => m.indexOf("import micropip") !== -1);
        if (installCode.length !== 0) {
          scriptCode.value = installCode.join("\n");
          console.log(scriptCode.value);
          await interpreter();
        }

        let logicalCode = matched
          .map((m) => m[1])
          .filter((m) => m.indexOf("import micropip") === -1);
        if (logicalCode.length !== 0) {
          scriptCode.value = logicalCode.join("\n");
          console.log(scriptCode.value);
          await interpreter();
        }

        codeRunning.value = false;
      }, 3000);
    };

    const addFile = async () => {
      const opts = {
        types: [
          {
            description: "Files",
            accept: {
              "image/*": [],
              "text/*": [".md", ".py", ".txt", ".csv", ".json", ".js"],
              "application/*": [],
              "video/*": [],
              "audio/*": [],
              "font/*": [],
            },
          },
        ],
        excludeAcceptAllOption: true,
        multiple: true,
      };
      const fileHandles = await window.showOpenFilePicker(opts);
      $q.loading.show();

      // 在循环中获取每个文件
      for (let fileHandle of fileHandles) {
        // 获取 File对象
        const file = await fileHandle.getFile();

        // 创建 FileReader 对象
        const reader = new FileReader();

        // 使用 promise 完成读取操作
        await new Promise((resolve) => {
          reader.onloadend = () => {
            resolve();
          };
          reader.readAsArrayBuffer(file);
        });

        // 存储 ArrayBuffer
        contextRef.value.filename = file.name;
        contextRef.value.fileBuffer = reader.result;
        scriptCode.value = `
from js import filename
from js import fileBuffer

mv = fileBuffer.to_py()
with open(f'/mount/{filename}', 'wb') as f:
  f.write(mv)
      `;
        await interpreter();
        messageList.value.push({
          role: "user",
          content: '"' + file.name + '"',
          parsedContent: "",
        });
      }

      $q.loading.hide();

      console.log("File written successfully.");
    };

    function readFileAsync(file) {
      return new Promise((resolve, reject) => {
        let reader = new FileReader();

        reader.onload = () => {
          resolve(reader.result);
        };

        reader.onerror = reject;

        reader.readAsArrayBuffer(file);
      });
    }

    function showModelDownloadDlg(model) {
      modelDownloadDlg.value = true;
    }

    async function selectModel(model) {
      clearHistory();
      changeCurrentModel(model);
      const result = await showModel(model);
      if (result === undefined) {
        showModelDownloadDlg(model);
      }
    }

    function showFileSaveDlg() {
      fileSaveDlg.value = true;
    }

    function shareHistory() {
      if (fileSaveFormat.value === "HTML") {
        fileSaveName.value = Date.now() + ".html";
      } else if (fileSaveFormat.value === "Markdown") {
        fileSaveName.value = Date.now() + ".md";
      }
      showFileSaveDlg();
    }

    async function saveTextFile() {
      let baseDir = BaseDirectory.Desktop;
      let text = "";
      if (fileSaveLocation.value.value === "Document") {
        baseDir = BaseDirectory.Document;
      } else if (fileSaveLocation.value.value === "Download") {
        baseDir = BaseDirectory.Download;
      }
      if (fileSaveFormat.value === "HTML") {
        text = convertChatToHTML({
          messages: messageList.value,
          model: currentChat.value.model,
        });
      } else if (fileSaveFormat.value === "Markdown") {
        text = convertChatToMarkdown({ messages: messageList.value });
      }
      await writeTextFile(fileSaveName.value, text, {
        baseDir: baseDir,
      });
      await openFileNotify(fileSaveName.value, baseDir);
    }

    async function openFileNotify(filename, baseDir) {
      // Download toast
      let pathPrefix;
      if (baseDir === BaseDirectory.Document) {
        pathPrefix = await documentDir();
      } else if (baseDir === BaseDirectory.Desktop) {
        pathPrefix = await desktopDir();
      } else if (baseDir === BaseDirectory.Download) {
        pathPrefix = await downloadDir();
      } else {
        pathPrefix = await homeDir();
      }
      let path = pathPrefix + "/" + filename;
      $q.notify(
        {
          message: t("chat.msg.tool.downloaded.notify.message"),
          caption: t("chat.msg.tool.downloaded.notify.caption") + path,
          icon: "check_circle",
          position: "bottom-right",
          //color: "white",
          //textColor: "primary",
          progress: true,
          actions: [
            {
              label: t("chat.msg.tool.downloaded.notify.action"),
              //color: "primary",
              handler: () => {
                console.log(path);
                open(path);
              },
            },
          ],
        },
        5000
      );
    }

    const usePrompt = (prompt) => {
      inputMessage.value = prompt;
    };

    const connectOllama = async () => {
      localStorage.setItem("ollama_url", ollamaURL.value);
      const response = await fetch(ollamaURL.value);
      const jsonData = await response.text();
      if (response.status === 200) {
        ollamaConnectMsg.value = jsonData;
        ollamaConnectDlg.value = false;
      } else {
        ollamaConnectMsg.value = response.statusText;
        ollamaConnectDlg.value = true;
      }
    };

    onMounted(async () => {
      connectOllama();
    });

    return {
      style,
      editorStyle,
      sentStyle,
      sendMessage,
      inputMessage,
      messageList,
      scrollAreaRef,
      receiving,
      typing,
      clearHistory,
      drawerRight,
      showHistory,
      historyList,
      historyClick,
      historyDelete,
      mdBgColor,
      copyMarkdown,
      downloadMarkdown,
      deleteConfirm,
      handleHistoryDeleteClick,
      deleteCache,
      ionCartOutline,
      closeWindow,
      openURL,
      fabMarkdown,
      fasFileCirclePlus,
      ionDocumentAttachOutline,
      changeCurrentModel,
      currentChat,
      markdownPlugins,
      stopChat,
      reChat,
      editMessage,
      permitFolder,
      codeRunning,
      codeRunResult,
      runCode,
      addFile,
      showCodeRef,
      stdout,
      slide: ref(0),
      scrollElement,
      selectModel,
      modelDownloadDlg,
      downloadStatus,
      modelDownloading,
      doDownloadModel,
      formatBytes,
      fileSaveDlg,
      ollamaConnectDlg,
      showFileSaveDlg,
      shareHistory,
      fileSaveName,
      fileSaveFormat,
      fileSaveLocation,
      saveTextFile,
      formatDate,
      great_prompts,
      usePrompt,
      mdCopied,
      mdDownloaded,
      ollamaURL,
      ollamaConnectMsg,
      connectOllama,
    };
  },
});
</script>
<style lang="css"></style>
<style lang="sass"></style>
<style lang="css">
.text-brand {
  color: #fbfbfb !important;
}
.bg-brand {
  background: #fbfbfb !important;
}
.bg-md-dark {
  background: #1b1b1d !important;
}
.text-md-dark {
  background: #1b1b1d !important;
}
.q-message-avatar {
  width: 35px;
  height: 35px;
  min-width: 35px;
}
</style>
