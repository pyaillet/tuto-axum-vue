<template>
  <div class="prose max-w-none">
    <h1>Hello !</h1>
    <p class="lead">
      An opinionated Vue 3, TypeScript, Tailwind CSS and ESLint template.
    </p>
    <h2>Messages:</h2>
    <p class="lead">
      <Message
        v-for="(msg, index) in messages"
        :key="index"
        :msg="msg"
        :index="index"
      />
    </p>
    <input
      v-model="message"
      class="m-3 py-2 px-4 rounded shadow-md"
      type="text"
      maxlength="100"
      placeholder="Type your message"
    >
    <button
      class="m-3 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded shadow-md"
      @click="post"
    >
      Post
    </button>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

import Message from '../components/Message.vue'

import MessageService from '../services/MessageService'

import CreateMessage from '../types/CreateMessage'

export default defineComponent({
  components: {
    Message,
  },
  data () {
    return {
      messages: [],
    }
  },
  mounted () {
    this.refreshMessages()
  },
  methods: {
    refreshMessages: function() {
      MessageService.getAll()
        .then((response) => {
          console.log(response)
          console.log(response.data.content)
          this.messages = response.data.content
        })
        .catch((error) => {
          console.log(error)
        })
    },
    post: function() {
      console.log('Posting new message')

      MessageService.create(new CreateMessage(this.message))
        .then((response: ResponseData) => {
          console.log(response.data)
          this.submitted = true
          console.log('Created message')
          this.message = ''
          this.refreshMessages()
        })
        .catch((e: Error) => {
          console.log(e)
        })
    },
  },
})
</script>

<style>
.prose a {
  @apply text-gray-900 underline !important;
}
</style>
