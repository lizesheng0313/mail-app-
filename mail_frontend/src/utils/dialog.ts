/**
 * 简单的确认对话框工具
 */

/**
 * 显示确认对话框
 */
export const showConfirm = (message: string, title: string = '提示'): Promise<boolean> => {
  return new Promise((resolve) => {
    // 创建遮罩层
    const overlay = document.createElement('div')
    overlay.className = 'fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-[10000]'

    // 创建对话框容器
    const dialog = document.createElement('div')
    dialog.className = 'bg-white rounded-lg shadow-xl max-w-md w-full mx-4'

    // 创建对话框内容
    dialog.innerHTML = `
      <div class="p-6">
        <h3 class="text-lg font-semibold text-gray-900 mb-3">${title}</h3>
        <p class="text-sm text-gray-600 mb-6">${message}</p>
        <div class="flex justify-end space-x-3">
          <button id="dialog-cancel" class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 transition-colors">
            取消
          </button>
          <button id="dialog-confirm" class="px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-md hover:bg-primary-700 transition-colors">
            确定
          </button>
        </div>
      </div>
    `

    overlay.appendChild(dialog)
    document.body.appendChild(overlay)

    // 清理函数
    const cleanup = () => {
      document.body.removeChild(overlay)
    }

    // 绑定事件
    const confirmBtn = dialog.querySelector('#dialog-confirm')
    const cancelBtn = dialog.querySelector('#dialog-cancel')

    confirmBtn?.addEventListener('click', () => {
      cleanup()
      resolve(true)
    })

    cancelBtn?.addEventListener('click', () => {
      cleanup()
      resolve(false)
    })

    // 点击遮罩层关闭
    overlay.addEventListener('click', (e) => {
      if (e.target === overlay) {
        cleanup()
        resolve(false)
      }
    })
  })
}

/**
 * 显示输入对话框
 */
export const showPrompt = (
  message: string,
  title: string = '提示',
  defaultValue: string = ''
): Promise<string | null> => {
  return new Promise((resolve) => {
    // 创建遮罩层
    const overlay = document.createElement('div')
    overlay.className = 'fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-[10000]'

    // 创建对话框容器
    const dialog = document.createElement('div')
    dialog.className = 'bg-white rounded-lg shadow-xl max-w-md w-full mx-4'

    // 创建对话框内容
    dialog.innerHTML = `
      <div class="p-6">
        <h3 class="text-lg font-semibold text-gray-900 mb-3">${title}</h3>
        <p class="text-sm text-gray-600 mb-3">${message}</p>
        <input
          id="dialog-input"
          type="text"
          value="${defaultValue}"
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-primary-500 focus:border-primary-500 mb-6"
        />
        <div class="flex justify-end space-x-3">
          <button id="dialog-cancel" class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 transition-colors">
            取消
          </button>
          <button id="dialog-confirm" class="px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-md hover:bg-primary-700 transition-colors">
            确定
          </button>
        </div>
      </div>
    `

    overlay.appendChild(dialog)
    document.body.appendChild(overlay)

    const input = dialog.querySelector('#dialog-input') as HTMLInputElement
    input.focus()
    input.select()

    // 清理函数
    const cleanup = () => {
      document.body.removeChild(overlay)
    }

    // 绑定事件
    const confirmBtn = dialog.querySelector('#dialog-confirm')
    const cancelBtn = dialog.querySelector('#dialog-cancel')

    const handleConfirm = () => {
      const value = input.value.trim()
      cleanup()
      resolve(value || null)
    }

    confirmBtn?.addEventListener('click', handleConfirm)

    cancelBtn?.addEventListener('click', () => {
      cleanup()
      resolve(null)
    })

    // 回车确认
    input.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') {
        handleConfirm()
      } else if (e.key === 'Escape') {
        cleanup()
        resolve(null)
      }
    })

    // 点击遮罩层关闭
    overlay.addEventListener('click', (e) => {
      if (e.target === overlay) {
        cleanup()
        resolve(null)
      }
    })
  })
}

/**
 * 显示警告对话框
 */
export const showAlert = (message: string, title: string = '提示'): Promise<void> => {
  return new Promise((resolve) => {
    // 创建遮罩层
    const overlay = document.createElement('div')
    overlay.className = 'fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-[10000]'

    // 创建对话框容器
    const dialog = document.createElement('div')
    dialog.className = 'bg-white rounded-lg shadow-xl max-w-md w-full mx-4'

    // 创建对话框内容
    dialog.innerHTML = `
      <div class="p-6">
        <h3 class="text-lg font-semibold text-gray-900 mb-3">${title}</h3>
        <p class="text-sm text-gray-600 mb-6">${message}</p>
        <div class="flex justify-end">
          <button id="dialog-ok" class="px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-md hover:bg-primary-700 transition-colors">
            确定
          </button>
        </div>
      </div>
    `

    overlay.appendChild(dialog)
    document.body.appendChild(overlay)

    // 清理函数
    const cleanup = () => {
      document.body.removeChild(overlay)
      resolve()
    }

    // 绑定事件
    const okBtn = dialog.querySelector('#dialog-ok')
    okBtn?.addEventListener('click', cleanup)

    // 点击遮罩层关闭
    overlay.addEventListener('click', (e) => {
      if (e.target === overlay) {
        cleanup()
      }
    })
  })
}
