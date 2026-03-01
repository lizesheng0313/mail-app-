<template>
  <Teleport to="body">
    <div
      v-if="visible"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-80"
      @click="close"
    >
      <button
        class="absolute top-4 right-4 p-2 text-white hover:text-gray-300 transition-colors z-10"
        @click="close"
      >
        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
      <img
        :src="src"
        class="max-w-[90vw] max-h-[90vh] object-contain rounded-lg"
        @click.stop
      />
    </div>
  </Teleport>
</template>

<script setup>
defineProps({
  visible: {
    type: Boolean,
    default: false
  },
  src: {
    type: String,
    default: ''
  }
})

const emit = defineEmits(['close', 'update:visible'])

const close = () => {
  emit('update:visible', false)
  emit('close')
}
</script>
