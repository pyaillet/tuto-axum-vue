<template>
  <div class="bg-indigo-100 m-2 p-2 pl-10 rounded">
    <p>{{ msg }}</p>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'

export default defineComponent({
  props: {
    msg: {
      type: String,
      default: '',
    },
  },

  setup() {
    return {
      name: ref(''),
    }
  },

})
</script>

