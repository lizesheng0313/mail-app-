<template>
  <div class="flex flex-col h-full">
    <!-- 标题栏 -->
    <div class="border-b border-gray-200 pb-4 mb-4">
      <div class="flex justify-between items-center">
        <h2 class="text-base font-semibold text-black">{{ title }}</h2>
        <div class="flex items-center gap-2">
          <slot name="header-actions"></slot>
        </div>
      </div>
    </div>
    
    <!-- 邮件详情内容 -->
    <div class="flex-1 overflow-y-auto">
      <div v-if="!hasEmail" class="flex items-center justify-center h-full text-gray-400">
        <div class="text-center">
          <svg class="w-16 h-16 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"></path>
          </svg>
          <p class="text-sm">{{ emptyText }}</p>
        </div>
      </div>
      
      <div v-else>
        <slot name="content"></slot>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  title: string
  hasEmail?: boolean
  emptyText?: string
}

withDefaults(defineProps<Props>(), {
  hasEmail: false,
  emptyText: '请选择一封邮件查看详情'
})
</script>
