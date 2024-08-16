<template>
  <q-page class="flex flex-center">
    <q-table
      flat
      bordered
      grid
      :title="$t('models.title')"
      :rows="rows"
      :columns="columns"
      row-key="name"
      :filter="filter"
      hide-header
      :pagination="pagination"
      :rows-per-page-options="[3]"
    >
      <!-- <template v-slot:top-right>
        <q-input
          borderless
          dense
          debounce="300"
          v-model="filter"
          placeholder="Search"
        >
          <template v-slot:append>
            <q-icon name="search" />
          </template>
        </q-input>
      </template> -->

      <template v-slot:item="props">
        <div class="q-pa-xs">
          <q-card bordered flat style="min-width: 200px">
            <q-card-section>
              <div class="text-h6">{{ props.row.name }}</div>
            </q-card-section>
            <q-separator />
            <q-list dense>
              <q-item v-for="col in props.cols" :key="col.name">
                <q-item-section>
                  <q-item-label>{{ col.label }}</q-item-label>
                </q-item-section>
                <q-item-section side>
                  <q-item-label caption>{{ col.value }}</q-item-label>
                </q-item-section>
              </q-item>
            </q-list>
            <q-card-actions>
              <q-btn
                outline
                icon="info"
                size="sm"
                color="primary"
                @click="showInfoDlg(props.row)"
              ></q-btn>
              <q-btn
                outline
                icon="delete"
                size="sm"
                color="negative"
                :loading="loadingId === props.row.name"
              >
                <q-menu touch-position>
                  <q-list dense style="min-width: 100px">
                    <q-item clickable v-close-popup>
                      <q-item-section>{{ $t("models.cancel") }}</q-item-section>
                    </q-item>
                    <q-item
                      clickable
                      v-close-popup
                      @click="deleteModel(props.row.name)"
                    >
                      <q-item-section>{{ $t("models.delete") }}</q-item-section>
                    </q-item>
                  </q-list>
                </q-menu>
              </q-btn>
            </q-card-actions>
          </q-card>
        </div>
      </template>
    </q-table>

    <!-- info dialog -->
    <q-dialog v-model="infoDlg" persistent>
      <q-card>
        <q-toolbar>
          <q-avatar icon="info" color="primary" text-color="white" />

          <q-toolbar-title>Info</q-toolbar-title>

          <q-btn flat round dense icon="close" v-close-popup />
        </q-toolbar>

        <q-card-section class="q-pt-none">
          <div class="q-pa-md">
            <q-list dense>
              <q-item>
                <q-item-section>
                  <q-item-label>Name</q-item-label>
                </q-item-section>
                <q-item-section side>
                  <q-item-label caption>{{ currentInfo.name }}</q-item-label>
                </q-item-section>
              </q-item>
              <q-item>
                <q-item-section>
                  <q-item-label>Size</q-item-label>
                </q-item-section>
                <q-item-section side>
                  <q-item-label caption>{{
                    formatBytes(currentInfo.size)
                  }}</q-item-label>
                </q-item-section>
              </q-item>
              <q-item>
                <q-item-section>
                  <q-item-label>Family</q-item-label>
                </q-item-section>
                <q-item-section side>
                  <q-item-label caption>{{
                    currentInfo.details.family
                  }}</q-item-label>
                </q-item-section>
              </q-item>
              <q-item>
                <q-item-section>
                  <q-item-label>Format</q-item-label>
                </q-item-section>
                <q-item-section side>
                  <q-item-label caption>{{
                    currentInfo.details.format
                  }}</q-item-label>
                </q-item-section>
              </q-item>
              <q-item>
                <q-item-section>
                  <q-item-label>Parameter Size</q-item-label>
                </q-item-section>
                <q-item-section side>
                  <q-item-label caption>{{
                    currentInfo.details.parameter_size
                  }}</q-item-label>
                </q-item-section>
              </q-item>
              <q-item>
                <q-item-section>
                  <q-item-label>Quantization Level</q-item-label>
                </q-item-section>
                <q-item-section side>
                  <q-item-label caption>{{
                    currentInfo.details.quantization_level
                  }}</q-item-label>
                </q-item-section>
              </q-item>
              <q-item>
                <q-item-section>
                  <q-item-label>Parent Model</q-item-label>
                </q-item-section>
                <q-item-section side>
                  <q-item-label caption>{{
                    currentInfo.details.parent_model
                  }}</q-item-label>
                </q-item-section>
              </q-item>
              <q-item>
                <q-item-section>
                  <q-item-label>Modified At</q-item-label>
                </q-item-section>
                <q-item-section side>
                  <q-item-label caption>{{
                    currentInfo.modified_at
                  }}</q-item-label>
                </q-item-section>
              </q-item>
              <q-item>
                <q-item-section>
                  <q-item-label>Digest</q-item-label>
                </q-item-section>
                <q-item-section side>
                  <q-item-label caption
                    ><q-btn
                      icon="content_copy"
                      size="xs"
                      round
                      @click="copyDigest(currentInfo.digest)"
                  /></q-item-label>
                </q-item-section>
              </q-item>
            </q-list>
          </div>
        </q-card-section>
      </q-card>
    </q-dialog>

    <q-page-sticky position="top-right" :offset="[18, 18]">
      <q-btn fab icon="reply" color="primary" to="/" />
    </q-page-sticky>
  </q-page>
</template>

<script>
import { ref, computed } from "vue";
import { useQuasar } from "quasar";
import { Ollama } from "ollama/browser";
import { fetch } from "@tauri-apps/plugin-http";
import { formatBytes } from "../utils/format";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

const columns = [
  { name: "model", label: "Model", field: "model", sortable: true },
  { name: "format", label: "Format", field: (row) => row.details.format },
  { name: "family", label: "Family", field: (row) => row.details.family },
  {
    name: "parameter_size",
    label: "Parameter Size",
    field: (row) => row.details.parameter_size,
  },
  {
    name: "size",
    align: "center",
    label: "Size",
    field: "size",
    format: (val) => `${formatBytes(val)}`,
    sortable: true,
  },
  {
    name: "modified_at",
    align: "center",
    label: "Modified At",
    field: "modified_at",
    format: (val) => `${val.substring(0, val.lastIndexOf("."))}`,
    sortable: true,
  },
];

export default {
  setup() {
    const firstRef = ref(null);
    const secondRef = ref(null);
    const $q = useQuasar();

    const text = ref("");
    const searchResult = ref("");
    const loadingId = ref("");

    const ollama_url = localStorage.getItem("ollama_url");
    const ollamaURL = ref(ollama_url ? ollama_url : "http://localhost:11434");

    const rows = ref([]);
    const infoDlg = ref(false);
    const currentInfo = ref({});

    const scrollAreaStyle = computed(() => ({
      height: $q.screen.height - 140 + "px",
    }));

    let ignoreSource;

    function scroll(source, position) {
      // if we previously just updated
      // the scroll position, then ignore
      // this update as otherwise we'll flicker
      // the position from one scroll area to
      // the other in an infinite loop
      if (ignoreSource === source) {
        ignoreSource = null;
        return;
      }

      // we'll now update the other scroll area,
      // which will also trigger a @scroll event...
      // and we need to ignore that one
      ignoreSource = source === "first" ? "second" : "first";

      const areaRef = source === "first" ? secondRef : firstRef;

      areaRef.value.setScrollPosition("vertical", position);
    }

    async function doSearch() {
      text.value = text.value.trim();
      if (!text.value) {
        searchResult.value = "";
        return;
      }
    }

    async function listModels() {
      const ollama = new Ollama({
        host: ollamaURL.value,
        fetch: fetch,
      });
      const response = await ollama.list();
      rows.value = response.models;
      for (const model of response.models) {
        console.log(model);
      }
    }

    async function deleteModel(model) {
      loadingId.value = model;
      const ollama = new Ollama({
        host: ollamaURL.value,
        fetch: fetch,
      });

      const response = await ollama.delete({ model: model });
      console.log(response);
      loadingId.value = "";
      listModels();
    }

    function showInfoDlg(info) {
      infoDlg.value = true;
      currentInfo.value = info;
    }

    async function copyDigest(text) {
      await writeText(text);
    }

    listModels();

    return {
      text,
      searchResult,
      firstRef,
      secondRef,
      scrollAreaStyle,

      thumbStyle: {
        right: "4px",
        borderRadius: "7px",
        backgroundColor: "#027be3",
        width: "4px",
        opacity: 0.75,
      },

      barStyle: {
        right: "2px",
        borderRadius: "9px",
        backgroundColor: "#027be3",
        width: "8px",
        opacity: 0.2,
      },

      onScrollFirst({ verticalPosition }) {
        scroll("first", verticalPosition);
      },

      onScrollSecond({ verticalPosition }) {
        scroll("second", verticalPosition);
      },

      doSearch,
      filter: ref(""),
      columns,
      rows,
      pagination: {
        sortBy: "desc",
        descending: false,
        page: 1,
        rowsPerPage: 3,
        // rowsNumber: xx if getting data from a server
      },
      loadingId,
      deleteModel,
      infoDlg,
      showInfoDlg,
      currentInfo,
      formatBytes,
      copyDigest,
    };
  },
};
</script>
