<template>
  <div>
    <!-- 顶部导航 -->
    <PageHeader />
    
    <div class="min-h-screen bg-gray-50 py-8">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <!-- 面包屑导航 -->
      <nav class="flex mb-6" aria-label="Breadcrumb">
        <ol class="inline-flex items-center space-x-1 md:space-x-3">
          <li class="inline-flex items-center">
            <router-link to="/automation" class="inline-flex items-center text-sm font-medium text-black hover:text-primary-600">
              <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
              </svg>
              自动化中心
            </router-link>
          </li>
          <li>
            <div class="flex items-center">
              <svg class="w-6 h-6 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd"></path>
              </svg>
              <span class="ml-1 text-sm font-medium text-black md:ml-2">我的资源</span>
            </div>
          </li>
        </ol>
      </nav>

      <!-- 页面头部 -->
      <div class="mb-8">
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-2xl font-bold text-black">我的资源</h1>
            <p class="mt-2 text-black">管理我创建和购买的数字资源</p>
          </div>
          <div class="flex space-x-3">
            <button
              @click="showCreateDialog = true"
              class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
            >
              <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
              创建工作流
            </button>
          </div>
        </div>
      </div>

      <!-- Tab 切换 -->
      <div class="bg-white rounded-lg shadow-sm p-1 inline-flex mb-4">
        <button
          @click="activeTab = 'all'"
          :class="[
            'px-5 py-2 rounded-md font-medium text-sm transition-all',
            activeTab === 'all'
              ? 'bg-primary-600 text-white shadow-md'
              : 'text-gray-600 hover:text-gray-900 hover:bg-gray-50'
          ]"
        >
          全部
        </button>
        <button
          @click="activeTab = 'owner'"
          :class="[
            'px-5 py-2 rounded-md font-medium text-sm transition-all',
            activeTab === 'owner'
              ? 'bg-primary-600 text-white shadow-md'
              : 'text-gray-600 hover:text-gray-900 hover:bg-gray-50'
          ]"
        >
          我创建的
        </button>
        <button
          @click="activeTab = 'purchased'"
          :class="[
            'px-5 py-2 rounded-md font-medium text-sm transition-all',
            activeTab === 'purchased'
              ? 'bg-primary-600 text-white shadow-md'
              : 'text-gray-600 hover:text-gray-900 hover:bg-gray-50'
          ]"
        >
          我购买的
        </button>
      </div>

      <!-- 筛选栏 -->
      <WorkflowFilters
        v-model:search-query="searchQuery"
        v-model:status-filter="statusFilter"
        :loading="loading"
        @refresh="fetchWorkflows"
      />

      <!-- 工作流列表 -->
      <WorkflowTable
        :workflows="filteredWorkflows"
        :loading="loading"
        @view="viewWorkflow"
        @edit="editWorkflow"
        @delete="deleteWorkflow"
        @publish="handlePublish"
        @edit-publish="handleEditPublish"
        @manage-inventory="handleManageInventory"
        @execute="handleExecuteWorkflow"
        @unpublish="handleUnpublish"
        @republish="handleRepublish"
      />
    </div>

    <!-- 创建工作流对话框 -->
    <CreateWorkflowModal
      v-if="showCreateDialog"
      :workflow="selectedWorkflow"
      @close="handleCreateDialogClose"
      @created="handleWorkflowCreated"
    />

    <!-- 工作流详情对话框 -->
    <WorkflowDetailModal
      v-if="showDetailDialog"
      :workflow="selectedWorkflow"
      @close="showDetailDialog = false"
      @updated="handleWorkflowUpdated"
    />

    <!-- 执行历史对话框 -->
    <ExecutionHistoryModal
      v-if="showHistoryDialog"
      :workflow-id="selectedWorkflow.workflow_id"
      :market-status="selectedWorkflow.market_status"
      @close="showHistoryDialog = false"
    />

    <!-- 删除确认对话框 -->
    <ConfirmDialog
      v-model:visible="showDeleteConfirm"
      title="确认删除"
      :message="`确定要删除工作流 &quot;${workflowToDelete?.name}&quot; 吗？`"
      type="danger"
      confirm-text="删除"
      :loading="deleting"
      loading-text="删除中"
      @confirm="confirmDeleteWorkflow"
      @cancel="cancelDelete"
    />

    <!-- 下架确认对话框 -->
    <ConfirmDialog
      v-model:visible="showUnpublishConfirm"
      title="确认下架"
      :message="`确定要下架工作流 &quot;${workflowToUnpublish?.name}&quot; 吗？下架后市场中将不再展示此工作流。`"
      type="warning"
      confirm-text="下架"
      :loading="unpublishing"
      loading-text="下架中"
      @confirm="confirmUnpublish"
      @cancel="showUnpublishConfirm = false"
    />

    <!-- 库存管理弹窗 -->
    <InventoryManagementModal
      v-if="showInventoryModal"
      :workflow="selectedInventoryWorkflow"
      @close="showInventoryModal = false"
      @updated="handleInventoryUpdated"
    />

    <!-- 执行确认对话框 -->
    <ConfirmDialog
      :visible="showExecuteConfirm"
      title="确认执行"
      :message="executeConfirmMessage"
      type="warning"
      confirm-text="确认执行"
      :loading="executing"
      @confirm="confirmExecuteWorkflow"
      @cancel="showExecuteConfirm = false"
    />

    <!-- 执行结果弹窗 -->
    <ExecutionResultModal
      :visible="showExecutionResult"
      :execution-data="executionResultData"
      @close="showExecutionResult = false"
    />

    <!-- 执行中的全局loading -->
    <div v-if="executing && !showExecuteConfirm" class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
      <div class="bg-white rounded-lg p-6 flex flex-col items-center">
        <svg class="animate-spin h-12 w-12 text-primary-600 mb-4" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        <p class="text-lg font-medium text-gray-900">工作流执行中...</p>
        <p class="text-sm text-gray-500 mt-2">请稍候，正在处理您的请求</p>
      </div>
    </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { workflowApi } from '@/api/workflow'
import { unpublishWorkflow, republishWorkflow } from '@/api/workflowMarket'
import PageHeader from '@/components/PageHeader/index.vue'
import WorkflowFilters from '../../workflows/components/WorkflowFilters/index.vue'
import WorkflowTable from '../../workflows/components/WorkflowTable/index.vue'
import CreateWorkflowModal from '../../workflows/components/CreateWorkflowModal/index.vue'
import WorkflowDetailModal from '../../workflows/components/WorkflowDetailModal/index.vue'
import ExecutionHistoryModal from '../../workflows/components/ExecutionHistoryModal/index.vue'
import InventoryManagementModal from '../../workflows/components/InventoryManagementModal/index.vue'
import ExecutionResultModal from '../../workflows/components/ExecutionResultModal/index.vue'
import ConfirmDialog from '@/components/ConfirmDialog/index.vue'
import ActionButton from '@/components/ActionButton/index.vue'
import { showMessage } from '@/utils/message'

const route = useRoute()
const router = useRouter()

// 响应式数据
const loading = ref(false)
const showCreateDialog = ref(false)
const showDetailDialog = ref(false)
const showHistoryDialog = ref(false)
const showDeleteConfirm = ref(false)
const showUnpublishConfirm = ref(false)
const workflowToUnpublish = ref(null)
const unpublishing = ref(false)
const showInventoryModal = ref(false)
const showExecuteConfirm = ref(false)
const showExecutionResult = ref(false)
const executingWorkflow = ref(null)
const executing = ref(false)
const executionResultData = ref({
  execution_id: '',
  status: '',
  result: null
})
const selectedWorkflow = ref(null)
const selectedInventoryWorkflow = ref(null)
const workflowToDelete = ref(null)
const deleting = ref(false)

const searchQuery = ref('')
const statusFilter = ref('')
const activeTab = ref('all')

const workflows = ref([])

// 计算属性
const executeConfirmMessage = computed(() => {
  if (!executingWorkflow.value) return '确认执行工作流？'
  
  const workflow = executingWorkflow.value
  const price = workflow.milk_coin_price || 0
  
  // 按次付费需要提示扣费
  if ((workflow.pricing_model === 'pay_per_use' || workflow.pricing_model === 'per_use') && price > 0) {
    return `执行工作流将消耗 ${price} 奶片，是否继续？`
  } else if (workflow.pricing_model === 'free' || !workflow.pricing_model) {
    return `确认执行工作流"${workflow.name}"？`
  } else if (workflow.pricing_model === 'subscription') {
    return `确认执行工作流"${workflow.name}"？（订阅用户免费）`
  } else if (workflow.pricing_model === 'one_time') {
    return `确认执行工作流"${workflow.name}"？（已购买，免费执行）`
  }
  
  return `确认执行工作流"${workflow.name}"？`
})

const filteredWorkflows = computed(() => {
  let filtered = workflows.value

  // Tab 筛选（注意：后端返回is_owner是0/1数字，需要严格判断）
  if (activeTab.value === 'owner') {
    filtered = filtered.filter(w => w.is_owner === true || w.is_owner === 1)
  } else if (activeTab.value === 'purchased') {
    filtered = filtered.filter(w => w.is_owner === false || w.is_owner === 0)
  }

  // 搜索筛选
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(workflow => 
      workflow.name.toLowerCase().includes(query) ||
      workflow.description?.toLowerCase().includes(query)
    )
  }

  // 状态筛选
  if (statusFilter.value) {
    filtered = filtered.filter(workflow => workflow.status === statusFilter.value)
  }

  return filtered
})

// 生命周期
onMounted(() => {
  fetchWorkflows()
  
  // 检查是否有模板参数
  if (route.query.template) {
    handleTemplateCreation(route.query.template)
  }
})

// 方法
const fetchWorkflows = async () => {
  loading.value = true
  try {
    // /workflows/ 接口已经返回了所有工作流（创建的+购买的），直接使用
    const res = await workflowApi.getWorkflows()
    
    if (res.code === 0 && res.data) {
      // 后端已经返回了所有工作流（创建的+购买的），直接使用
      workflows.value = Array.isArray(res.data.workflows) ? res.data.workflows : []
    } else {
      workflows.value = []
    }
    
  } catch (error) {
    console.error('加载工作流失败:', error)
    workflows.value = []
  } finally {
    loading.value = false
  }
}

const handleTemplateCreation = (templateId) => {
  // 根据模板ID预设创建对话框的数据
  console.log('从模板创建工作流:', templateId)
  showCreateDialog.value = true
}

const viewWorkflow = async (workflow) => {
  // 调用详情接口获取完整数据（包括 steps）
  try {
    const res = await workflowApi.getWorkflow(workflow.workflow_id)
    if (res.code === 0 && res.data) {
      selectedWorkflow.value = res.data
      showDetailDialog.value = true
    } else {
      showMessage(res.message || '获取工作流详情失败', 'error')
    }
  } catch (error) {
    console.error('获取工作流详情失败:', error)
    showMessage('获取工作流详情失败', 'error')
  }
}

const editWorkflow = async (workflow) => {
  try {
    const res = await workflowApi.getWorkflow(workflow.workflow_id)
    if (res.code === 0 && res.data) {
      selectedWorkflow.value = res.data
      showCreateDialog.value = true
    } else {
      showMessage(res.message || '获取工作流详情失败', 'error')
    }
  } catch (error) {
    console.error('获取工作流详情失败:', error)
    showMessage('获取工作流详情失败', 'error')
  }
}

const executeWorkflow = async (workflow) => {
  try {
    const response = await workflowApi.executeWorkflow(workflow.workflow_id, {})
    if (response.code === 0) {
      showMessage(`工作流 "${workflow.name}" 已开始执行`, 'success')
      console.log('工作流执行成功:', response.data)
    }
  } catch (error) {
    console.error('执行工作流失败:', error)
  }
}

const deleteWorkflow = (workflow) => {
  workflowToDelete.value = workflow
  showDeleteConfirm.value = true
}

const confirmDeleteWorkflow = async () => {
  try {
    deleting.value = true
    const response = await workflowApi.deleteWorkflow(workflowToDelete.value.workflow_id)
    if (response.code === 0) {
      workflows.value = workflows.value.filter(w => w.id !== workflowToDelete.value.id)
      showMessage('工作流已删除', 'success')
      console.log(`工作流 ${workflowToDelete.value.name} 已删除`)
    }
  } catch (error) {
    console.error('删除工作流失败:', error)
  } finally {
    deleting.value = false
    showDeleteConfirm.value = false
    workflowToDelete.value = null
  }
}

const cancelDelete = () => {
  showDeleteConfirm.value = false
  workflowToDelete.value = null
}

const handleWorkflowCreated = (workflow) => {
  workflows.value.push(workflow)
  showCreateDialog.value = false
  selectedWorkflow.value = null // 清空选中的工作流
  fetchWorkflows() // 重新加载列表
}

const handleCreateDialogClose = () => {
  showCreateDialog.value = false
  selectedWorkflow.value = null // 清空选中的工作流
}

const handleWorkflowUpdated = (updatedWorkflow) => {
  const index = workflows.value.findIndex(w => w.id === updatedWorkflow.id)
  if (index !== -1) {
    workflows.value[index] = updatedWorkflow
  }
  showDetailDialog.value = false
  fetchWorkflows() // 重新加载列表
}

const handlePublish = (workflow) => {
  router.push({
    path: '/workflows/publish',
    state: {
      workflow_id: workflow.workflow_id,
      workflow_name: workflow.name
    }
  })
}

// 编辑已发布的工作流
const handleEditPublish = (workflow) => {
  router.push({
    path: '/workflows/publish',
    state: {
      workflow_id: workflow.workflow_id,
      workflow_name: workflow.name,
      edit_mode: true
    }
  })
}

// 下架工作流
const handleUnpublish = (workflow) => {
  workflowToUnpublish.value = workflow
  showUnpublishConfirm.value = true
}

const confirmUnpublish = async () => {
  if (!workflowToUnpublish.value) return
  unpublishing.value = true
  try {
    const res = await unpublishWorkflow(workflowToUnpublish.value.workflow_id)
    if (res.code === 0) {
      showMessage('工作流已下架', 'success')
      showUnpublishConfirm.value = false
      fetchWorkflows()
    } else {
      showMessage(res.message || '下架失败', 'error')
    }
  } catch (error) {
    console.error('下架失败:', error)
    showMessage('下架失败', 'error')
  } finally {
    unpublishing.value = false
    workflowToUnpublish.value = null
  }
}

// 重新上架工作流
const handleRepublish = async (workflow) => {
  try {
    const res = await republishWorkflow(workflow.workflow_id)
    if (res.code === 0) {
      showMessage('工作流已重新上架', 'success')
      fetchWorkflows()
    } else {
      showMessage(res.message || '重新上架失败', 'error')
    }
  } catch (error) {
    console.error('重新上架失败:', error)
    showMessage('重新上架失败', 'error')
  }
}

const handleManageInventory = (workflow) => {
  selectedInventoryWorkflow.value = workflow
  showInventoryModal.value = true
}

const handleInventoryUpdated = () => {
  // 刷新工作流列表以更新库存数量
  fetchWorkflows()
}

// 执行购买的工作流
const handleExecuteWorkflow = (workflow) => {
  console.log('handleExecuteWorkflow 被调用:', workflow)
  executingWorkflow.value = workflow
  // 所有工作流都要弹确认框
  showExecuteConfirm.value = true
  console.log('showExecuteConfirm 设置为 true')
}

const confirmExecuteWorkflow = async () => {
  console.log('confirmExecuteWorkflow 被调用')
  // 关闭确认框，显示loading
  showExecuteConfirm.value = false
  executing.value = true
  console.log('executing 设置为 true, 开始执行工作流')
  
  try {
    const response = await workflowApi.executeWorkflow(executingWorkflow.value.workflow_id, {})
    console.log('工作流执行响应:', response)
    if (response.code === 0) {
      const status = response.data.status
      
      // 执行完成，直接显示结果弹窗
      if (status === 'completed') {
        executionResultData.value = response.data
        showExecutionResult.value = true
        showMessage('工作流执行成功！', 'success')
      }
      // 执行失败
      else if (status === 'failed') {
        showMessage(response.data.error || '工作流执行失败', 'error')
      }
      // 其他状态（executing/pending）
      else {
        showMessage(response.data.message || '工作流已提交执行', 'info')
      }
    }
  } catch (error) {
    console.error('执行失败:', error)
  } finally {
    executing.value = false
  }
}

</script>