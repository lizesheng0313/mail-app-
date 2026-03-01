<template>
  <div class="simple-chart bg-white">
    <div class="chart-header text-center mb-4">
      <h4 class="text-sm font-medium text-black">{{ title }}</h4>
    </div>
    
    <div class="chart-content">
      <div class="chart-bars flex items-end justify-around px-2 mb-4" style="height: 200px;">
        <div
          v-for="(item, index) in chartData"
          :key="index"
          class="chart-bar-group flex flex-col items-center flex-1 max-w-16"
        >
          <div class="chart-bars-container flex items-end justify-center gap-1 h-44 mb-2">
            <div
              class="chart-bar total w-3 min-h-0.5 rounded-t-sm bg-primary-500 transition-all duration-300 hover:opacity-80 cursor-pointer"
              :style="{ height: getBarHeight(item.total) + '%' }"
              :title="`总调用: ${item.total}`"
            ></div>
            <div
              class="chart-bar success w-3 min-h-0.5 rounded-t-sm bg-primary-500 transition-all duration-300 hover:opacity-80 cursor-pointer"
              :style="{ height: getBarHeight(item.success) + '%' }"
              :title="`成功调用: ${item.success}`"
            ></div>
            <div
              class="chart-bar error w-3 min-h-0.5 rounded-t-sm bg-red-500 transition-all duration-300 hover:opacity-80 cursor-pointer"
              :style="{ height: getBarHeight(item.error) + '%' }"
              :title="`失败调用: ${item.error}`"
            ></div>
          </div>
          <div class="chart-label text-xs text-black text-center transform -rotate-45 whitespace-nowrap">
            {{ item.label }}
          </div>
        </div>
      </div>
      
      <div class="chart-legend flex justify-center gap-5 pt-2 border-t border-gray-100">
        <div class="legend-item flex items-center gap-1.5">
          <div class="legend-color w-3 h-3 rounded-sm bg-primary-500"></div>
          <span class="text-xs text-black">总调用</span>
        </div>
        <div class="legend-item flex items-center gap-1.5">
          <div class="legend-color w-3 h-3 rounded-sm bg-primary-500"></div>
          <span class="text-xs text-black">成功调用</span>
        </div>
        <div class="legend-item flex items-center gap-1.5">
          <div class="legend-color w-3 h-3 rounded-sm bg-red-500"></div>
          <span class="text-xs text-black">失败调用</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  title: {
    type: String,
    default: '使用统计'
  },
  data: {
    type: Array,
    default: () => []
  }
})

// 处理图表数据
const chartData = computed(() => {
  return props.data.map(item => ({
    label: formatDate(item.date),
    total: item.calls || 0,
    success: item.success_calls || 0,
    error: item.error_calls || 0
  }))
})

// 获取最大值用于计算比例
const maxValue = computed(() => {
  const values = chartData.value.map(item => item.total)
  return Math.max(...values, 1)
})

// 计算柱状图高度百分比
const getBarHeight = (value) => {
  return (value / maxValue.value) * 100
}

// 格式化日期
const formatDate = (dateString) => {
  const date = new Date(dateString)
  return `${date.getMonth() + 1}/${date.getDate()}`
}
</script>
