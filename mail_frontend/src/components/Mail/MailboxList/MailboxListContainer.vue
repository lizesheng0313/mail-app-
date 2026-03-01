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
    
    <!-- 列表内容 -->
    <div class="flex-1 overflow-y-auto scrollbar-stable">
      <slot name="content"></slot>
      
      <!-- 空状态 -->
      <div v-if="isEmpty" class="p-8 text-center text-gray-400">
        <slot name="empty">
          <svg class="w-16 h-16 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 4.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"></path>
          </svg>
          <p class="text-sm">{{ emptyText }}</p>
        </slot>
      </div>
    </div>
    
    <!-- 底部分页 -->
    <div v-if="showPagination" class="mt-4">
      <slot name="pagination"></slot>
    </div>
    
    <!-- 批量操作工具栏 -->
    <slot name="batch-toolbar"></slot>
  </div>
</template>

<script setup lang="ts">
interface Props {
  title: string
  isEmpty?: boolean
  emptyText?: string
  showPagination?: boolean
}

withDefaults(defineProps<Props>(), {
  isEmpty: false,
  emptyText: '暂无数据',
  showPagination: false
})
</script>
