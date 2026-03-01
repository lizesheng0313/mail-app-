/**
 * 规则构建器组件 - 可视化配置触发器规则
 */
<template>
  <div class="rule-builder">
    <div class="mb-4">
      <h4 class="text-lg font-medium text-black mb-2">触发规则配置</h4>
      <p class="text-sm text-black">通过可视化界面配置复杂的邮件匹配规则</p>
    </div>

    <!-- 规则组 -->
    <RuleGroup 
      v-model="rules"
      :level="0"
      @update="$emit('update:modelValue', rules)"
    />

    <!-- JSON预览 -->
    <div class="mt-6">
      <div class="flex items-center justify-between mb-2">
        <h5 class="text-sm font-medium text-black">规则JSON预览</h5>
        <button 
          type="button"
          @click="showJson = !showJson"
          class="text-xs text-primary-600 hover:text-primary-700"
        >
          {{ showJson ? '隐藏' : '显示' }} JSON
        </button>
      </div>
      <div v-if="showJson" class="bg-gray-50 rounded-lg p-3">
        <pre class="text-xs text-black overflow-x-auto">{{ formattedJson }}</pre>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import RuleGroup from '../RuleGroup/index.vue'

const props = defineProps({
  modelValue: {
    type: Object,
    default: () => ({
      logic: 'and',
      conditions: []
    })
  }
})

const emit = defineEmits(['update:modelValue'])

const rules = ref(props.modelValue)
const showJson = ref(false)

// 格式化JSON显示
const formattedJson = computed(() => {
  return JSON.stringify(rules.value, null, 2)
})

// 监听props变化
watch(() => props.modelValue, (newValue) => {
  rules.value = newValue
}, { deep: true })
</script>

<style scoped>
.rule-builder {
  @apply space-y-4;
}

pre {
  white-space: pre-wrap;
  word-wrap: break-word;
}
</style>