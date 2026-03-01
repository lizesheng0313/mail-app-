/**
 * 规则组组件 - 递归构建嵌套规则条件
 */
<template>
  <div class="rule-group" :class="{ 'nested': level > 0 }">
    <!-- 逻辑操作符选择 -->
    <div class="flex items-center mb-3">
      <label class="text-sm font-medium text-black mr-2">逻辑关系:</label>
      <CustomSelect
        :model-value="localRules.logic"
        :options="logicOptions"
        @update:modelValue="(value) => { localRules.logic = value; updateParent() }"
        class="w-32"
      />
    </div>

    <!-- 条件列表 -->
    <div class="space-y-3">
      <div 
        v-for="(condition, index) in localRules.conditions" 
        :key="index"
        class="condition-item p-3 border border-gray-200 rounded-lg"
      >
        <!-- 嵌套规则组 -->
        <div v-if="condition.logic" class="nested-group">
          <div class="flex items-center justify-between mb-2">
            <span class="text-sm font-medium text-black">嵌套条件组</span>
            <button 
              type="button"
              @click="removeCondition(index)"
              class="text-red-500 hover:text-red-700 text-xs"
            >
              删除组
            </button>
          </div>
          <RuleGroup 
            :model-value="condition"
            :level="level + 1"
            @update:modelValue="(value) => updateCondition(index, value)"
          />
        </div>

        <!-- 单个条件 -->
        <div v-else class="single-condition">
          <div class="flex items-center justify-between mb-2">
            <span class="text-sm font-medium text-black">条件 {{ index + 1 }}</span>
            <button 
              type="button"
              @click="removeCondition(index)"
              class="text-red-500 hover:text-red-700 text-xs"
            >
              删除
            </button>
          </div>
          
          <div class="grid grid-cols-1 md:grid-cols-4 gap-3 items-center">
            <!-- 字段选择 -->
            <div>
              <label class="block text-xs text-black mb-1">字段</label>
              <CustomSelect
                :model-value="condition.field"
                :options="fieldOptions"
                placeholder="选择字段"
                @update:modelValue="(value) => updateConditionField(index, 'field', value)"
                class="w-full"
              />
            </div>

            <!-- 操作符选择 -->
            <div>
              <label class="block text-xs text-black mb-1">操作</label>
              <CustomSelect
                :model-value="condition.operator"
                :options="operatorOptions"
                placeholder="选择操作"
                @update:modelValue="(value) => updateConditionField(index, 'operator', value)"
                class="w-full"
              />
            </div>

            <!-- 值输入 -->
            <div>
              <label class="block text-xs text-black mb-1">值</label>
              <BaseInput
                :model-value="condition.value"
                type="text"
                size="sm"
                :placeholder="getValuePlaceholder(condition.field, condition.operator)"
                @update:modelValue="(value) => updateConditionField(index, 'value', value)"
                class="w-full"
              />
            </div>

            <!-- 大小写敏感 -->
            <div>
              <label class="block text-xs text-black mb-1">&nbsp;</label>
              <div class="flex items-center">
                <label class="flex items-center text-xs text-black">
                  <input 
                    type="checkbox"
                    :checked="condition.case_sensitive"
                    @change="(e) => updateConditionField(index, 'case_sensitive', e.target.checked)"
                    class="mr-1"
                  />
                  大小写敏感
                </label>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 添加按钮 -->
    <div class="mt-4 flex space-x-2">
      <button 
        type="button"
        @click="addCondition"
        class="px-3 py-1 bg-primary-100 text-primary-700 rounded text-sm hover:bg-primary-200"
      >
        + 添加条件
      </button>
      <button 
        type="button"
        @click="addGroup"
        class="px-3 py-1 bg-primary-100 text-primary-700 rounded text-sm hover:bg-blue-200"
      >
        + 添加条件组
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'
import CustomSelect from '../CustomSelect/index.vue'
import BaseInput from '../BaseInput/index.vue'

const props = defineProps({
  modelValue: {
    type: Object,
    default: () => ({
      logic: 'and',
      conditions: []
    })
  },
  level: {
    type: Number,
    default: 0
  }
})

const emit = defineEmits(['update:modelValue', 'update'])

const localRules = ref({ ...props.modelValue })

// 选项数据
const logicOptions = [
  { label: 'AND (且)', value: 'and' },
  { label: 'OR (或)', value: 'or' }
]

const fieldOptions = [
  { label: '发件人', value: 'email.sender' },
  { label: '收件人', value: 'email.recipient' },
  { label: '主题', value: 'email.subject' },
  { label: '正文内容', value: 'email.body' }
]

const operatorOptions = [
  { label: '包含', value: 'contains' },
  { label: '不包含', value: 'not_contains' },
  { label: '等于', value: 'equals' },
  { label: '不等于', value: 'not_equals' },
  { label: '开始于', value: 'starts_with' },
  { label: '结束于', value: 'ends_with' },
  { label: '正则匹配', value: 'regex' },
  { label: '为空', value: 'is_empty' },
  { label: '不为空', value: 'is_not_empty' }
]

const updateParent = () => {
  emit('update:modelValue', localRules.value)
  emit('update')
}

const addCondition = () => {
  localRules.value.conditions.push({
    field: '',
    operator: '',
    value: '',
    case_sensitive: false
  })
  updateParent()
}

const addGroup = () => {
  localRules.value.conditions.push({
    logic: 'and',
    conditions: []
  })
  updateParent()
}

const updateConditionField = (index, field, value) => {
  localRules.value.conditions[index][field] = value
  updateParent()
}

const updateCondition = (index, value) => {
  localRules.value.conditions[index] = value
  updateParent()
}

const removeCondition = (index) => {
  localRules.value.conditions.splice(index, 1)
  updateParent()
}

const getValuePlaceholder = (field, operator) => {
  if (!field || !operator) return '请先选择字段和操作'
  
  if (operator === 'is_empty' || operator === 'is_not_empty') {
    return '无需输入值'
  }
  
  return '输入匹配值'
}

watch(() => props.modelValue, (newValue) => {
  localRules.value = { ...newValue }
}, { deep: true })
</script>

<style scoped>
.rule-group.nested {
  @apply ml-4 pl-4 border-l-2 border-gray-200;
}

.condition-item {
  @apply bg-gray-50;
}

.nested-group {
  @apply bg-white p-3 rounded border border-gray-300;
}
</style>