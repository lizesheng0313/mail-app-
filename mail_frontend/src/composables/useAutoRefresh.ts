import { ref, onUnmounted } from 'vue'

/**
 * 自动刷新
 */
export function useAutoRefresh(callback: () => void | Promise<void>, intervalSeconds: number) {
  const countdown = ref(intervalSeconds)
  let timer: number | null = null

  const start = () => {
    // 重置倒计时
    countdown.value = intervalSeconds

    // 使用单一定时器，每秒执行一次
    timer = window.setInterval(async () => {
      countdown.value--
      
      // 倒计时到0时执行回调并重置
      if (countdown.value <= 0) {
        await callback()
        countdown.value = intervalSeconds
      }
    }, 1000)
  }

  const stop = () => {
    if (timer) {
      clearInterval(timer)
      timer = null
    }
    countdown.value = 0
  }

  // 组件卸载时清理
  onUnmounted(() => {
    stop()
  })

  return {
    countdown,
    start,
    stop
  }
}
