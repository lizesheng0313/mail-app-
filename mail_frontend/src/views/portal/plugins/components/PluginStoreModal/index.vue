<template>
  <!-- 模态框背景 -->
  <div class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50" @click="$emit('close')">
    <!-- 模态框内容 -->
    <div class="relative top-20 mx-auto p-5 border w-11/12 max-w-6xl shadow-lg rounded-md bg-white" @click.stop>
      <!-- 头部 -->
      <div class="flex items-center justify-between pb-4 border-b border-gray-200">
        <div>
          <h3 class="text-lg font-semibold text-black">插件商店</h3>
          <p class="text-sm text-black mt-1">发现和安装新插件</p>
        </div>
        <button
          @click="$emit('close')"
          class="text-gray-400 hover:text-black transition-colors"
        >
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- 搜索和筛选 -->
      <div class="py-4 border-b border-gray-200">
        <div class="flex flex-col sm:flex-row gap-4">
          <!-- 搜索框 -->
          <div class="flex-1">
            <BaseInput
              v-model="searchQuery"
              type="text"
              placeholder="搜索插件..."
              size="sm"
              @input="handleSearch"
            >
              <template #left-icon>
                <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
              </template>
            </BaseInput>
          </div>

          <!-- 分类筛选 -->
          <div class="sm:w-48">
            <select
              v-model="selectedCategory"
              @change="handleCategoryChange"
              class="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
            >
              <option value="">所有分类</option>
              <option value="crawler">网页助手</option>
              <option value="proxy">代理工具</option>
              <option value="email">邮件工具</option>
              <option value="security">安全工具</option>
              <option value="utility">实用工具</option>
            </select>
          </div>

          <!-- 排序 -->
          <div class="sm:w-48">
            <select
              v-model="sortBy"
              @change="handleSortChange"
              class="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
            >
              <option value="featured">推荐排序</option>
              <option value="rating">评分排序</option>
              <option value="downloads">下载量排序</option>
              <option value="updated">更新时间</option>
            </select>
          </div>
        </div>
      </div>

      <!-- 内容区域 -->
      <div class="py-6 max-h-96 overflow-y-auto">
        <!-- 加载状态 -->
        <div v-if="loading" class="flex justify-center py-12">
          <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
        </div>

        <!-- 空状态 -->
        <div v-else-if="plugins.length === 0" class="text-center py-12">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2 2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
          </svg>
          <h3 class="mt-2 text-sm font-medium text-black">暂无插件</h3>
          <p class="mt-1 text-sm text-black">没有找到符合条件的插件</p>
        </div>

        <!-- 插件网格 -->
        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <PluginCard
            v-for="plugin in plugins"
            :key="plugin.plugin_id"
            :plugin="plugin"
            :show-price="true"
            @click="showPluginDetail(plugin)"
          >
            <template #actions="{ plugin }">
              <div class="flex space-x-2">
                <button
                  v-if="!isPluginInstalled(plugin.plugin_id)"
                  @click.stop="installPlugin(plugin)"
                  :disabled="installing === plugin.plugin_id"
                  class="flex-1 bg-primary-600 hover:bg-primary-700 disabled:bg-blue-400 text-white text-sm font-medium py-2 px-4 rounded-md transition-colors"
                >
                  {{ installing === plugin.plugin_id ? '安装中...' : (plugin.is_free ? '免费安装' : `${Math.floor(plugin.price)}奶片 购买`) }}
                </button>
                
                <button
                  v-else
                  class="flex-1 bg-gray-100 text-black text-sm font-medium py-2 px-4 rounded-md cursor-not-allowed"
                  disabled
                >
                  已安装
                </button>
                
                <button
                  @click.stop="showPluginDetail(plugin)"
                  class="px-3 py-2 border border-gray-300 text-black text-sm font-medium rounded-md hover:bg-gray-50 transition-colors"
                >
                  详情
                </button>
              </div>
            </template>
          </PluginCard>
        </div>

        <!-- 分页 -->
        <div v-if="totalPages > 1" class="mt-6 flex justify-center">
          <nav class="relative z-0 inline-flex rounded-md shadow-sm -space-x-px">
            <button
              @click="changePage(currentPage - 1)"
              :disabled="currentPage === 1"
              class="relative inline-flex items-center px-2 py-2 rounded-l-md border border-gray-300 bg-white text-sm font-medium text-black hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              上一页
            </button>
            
            <button
              v-for="page in visiblePages"
              :key="page"
              @click="changePage(page)"
              :class="[
                'relative inline-flex items-center px-4 py-2 border text-sm font-medium',
                page === currentPage
                  ? 'z-10 bg-primary-50 border-primary-500 text-primary-600'
                  : 'bg-white border-gray-300 text-black hover:bg-gray-50'
              ]"
            >
              {{ page }}
            </button>
            
            <button
              @click="changePage(currentPage + 1)"
              :disabled="currentPage === totalPages"
              class="relative inline-flex items-center px-2 py-2 rounded-r-md border border-gray-300 bg-white text-sm font-medium text-black hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              下一页
            </button>
          </nav>
        </div>
      </div>
    </div>

    <!-- 插件详情对话框 -->
    <PluginStoreDetailModal
      v-if="showDetailDialog && selectedPlugin"
      :plugin="selectedPlugin"
      @close="showDetailDialog = false"
      @install="handleInstallFromDetail"
    />
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'
import BaseInput from '@/components/BaseInput/index.vue'
import PluginCard from '../PluginCard/index.vue'
import PluginStoreDetailModal from '../PluginStoreDetailModal/index.vue'
import { pluginApi } from '@/api/plugin'
import { showMessage } from '@/utils/message'

const emit = defineEmits(['close', 'install'])

const loading = ref(false)
const plugins = ref([])
const installedPlugins = ref([])
const installing = ref(null)
const searchQuery = ref('')
const selectedCategory = ref('')
const sortBy = ref('featured')
const currentPage = ref(1)
const totalPages = ref(1)
const pageSize = 12

const showDetailDialog = ref(false)
const selectedPlugin = ref(null)

// 计算可见页码
const visiblePages = computed(() => {
  const pages = []
  const start = Math.max(1, currentPage.value - 2)
  const end = Math.min(totalPages.value, start + 4)
  
  for (let i = start; i <= end; i++) {
    pages.push(i)
  }
  return pages
})

// 获取插件列表
const fetchPlugins = async () => {
  try {
    loading.value = true
    const response = await pluginApi.getStorePlugins({
      page: currentPage.value,
      page_size: pageSize,
      search: searchQuery.value,
      category: selectedCategory.value,
      sort_by: sortBy.value
    })
    
    if (response.code === 0) {
      plugins.value = response.data.plugins
      totalPages.value = Math.ceil(response.data.total / pageSize)
    } else {
      showMessage(response.message, 'error')
    }
  } catch (error) {
    showMessage('获取插件列表失败', 'error')
  } finally {
    loading.value = false
  }
}

// 获取已安装插件列表
const fetchInstalledPlugins = async () => {
  try {
    const response = await pluginApi.getMyPlugins()
    if (response.code === 0) {
      installedPlugins.value = response.data.map(p => p.plugin_id)
    }
  } catch (error) {
    console.error('获取已安装插件失败:', error)
  }
}

// 检查插件是否已安装
const isPluginInstalled = (pluginId) => {
  return installedPlugins.value.includes(pluginId)
}

// 搜索处理
const handleSearch = () => {
  currentPage.value = 1
  fetchPlugins()
}

// 分类变更处理
const handleCategoryChange = () => {
  currentPage.value = 1
  fetchPlugins()
}

// 排序变更处理
const handleSortChange = () => {
  currentPage.value = 1
  fetchPlugins()
}

// 切换页面
const changePage = (page) => {
  if (page >= 1 && page <= totalPages.value) {
    currentPage.value = page
    fetchPlugins()
  }
}

// 显示插件详情
const showPluginDetail = (plugin) => {
  selectedPlugin.value = plugin
  showDetailDialog.value = true
}

// 安装插件
const installPlugin = async (plugin) => {
  try {
    installing.value = plugin.plugin_id
    
    const response = await pluginApi.installPlugin(plugin.plugin_id)
    
    if (response.code === 0) {
      showMessage('插件安装成功')
      installedPlugins.value.push(plugin.plugin_id)
      emit('install')
    } else {
      showMessage(response.message, 'error')
    }
  } catch (error) {
    showMessage('插件安装失败', 'error')
  } finally {
    installing.value = null
  }
}

// 从详情页安装
const handleInstallFromDetail = () => {
  showDetailDialog.value = false
  fetchInstalledPlugins()
  emit('install')
}

onMounted(() => {
  fetchPlugins()
  fetchInstalledPlugins()
})
</script>
