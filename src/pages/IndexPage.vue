<template>
  <q-page class="flex flex-center">
    <div>
      <div class="flex flex-center">
        <img
          alt="Quasar logo"
          src="~assets/quasar-logo-vertical.svg"
          style="width: 200px; height: 200px"
        />
      </div>
      <div class="flex flex-center" style="margin-top: 30px">
        <q-btn color="primary" icon="chat" label="开始聊天" to="/chat" />
      </div>
    </div>
  </q-page>
</template>

<script>
import { defineComponent } from "vue";

export default defineComponent({
  name: "IndexPage",
  setup() {
    // now we can call our Command!
    // Right-click the application background and open the developer tools.
    // You will see "Hello, World!" printed in the console!
    return {};
  },
});
</script>
