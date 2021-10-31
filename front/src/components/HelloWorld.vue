<template>
  <div class="prose max-w-none">
    <h1>{{ msg }}</h1>
    <p class="lead">
      An opinionated Vue 3, TypeScript, Tailwind CSS and ESLint template.
    </p>
  </div>
</template>

<script lang="ts">
import { defineComponent, toRef, ref } from 'vue'
import { globalState } from '../store'

export default defineComponent({
  props: {
    msg: {
      type: String,
      default: '',
    },
  },

  setup() {
    return {
      count: toRef(globalState, 'count'),
      name: ref(''),
    }
  },
})
</script>

<style>
.prose a {
  @apply text-gray-900 underline !important;
}
</style>
