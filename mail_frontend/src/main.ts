import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import './style.css'
import { isTauri } from '@/services/api'
import { setupSeo } from '@/seo'

// 桌面端添加标识 class
if (isTauri()) {
  document.body.classList.add('is-tauri')
}

const app = createApp(App)

app.use(createPinia())
app.use(router)
setupSeo(router)

app.mount('#app')
