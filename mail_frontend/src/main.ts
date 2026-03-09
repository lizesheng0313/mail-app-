import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import './style.css'
import { isTauri } from '@/services/api'

// 桌面端添加标识 class
if (isTauri()) {
  document.body.classList.add('is-tauri')
}

const app = createApp(App)

app.use(createPinia())
app.use(router)

app.mount('#app')

// 开发环境调试工具
if (import.meta.env.DEV) {
  (window as any).debugAuth = () => {
    console.log('🔍 认证状态调试信息:')
    console.log('Token:', localStorage.getItem('token'))
    console.log('IsAuthenticated:', localStorage.getItem('isAuthenticated'))
    console.log('User:', localStorage.getItem('user'))
  }
  console.log('💡 提示: 在控制台输入 debugAuth() 可查看认证状态')
}
