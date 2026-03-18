import type { RouteLocationNormalizedLoaded, Router } from 'vue-router'

export interface PageSeo {
  title: string
  description: string
  keywords?: string
  canonicalPath?: string
  robots?: string
  ogType?: string
  lang?: string
}

const SITE_URL = 'https://zjkdongao.cn'
const SITE_NAME = '肥猫猫'

const DEFAULT_SEO: PageSeo = {
  title: '肥猫猫 - 邮箱管理、第三方邮箱接入与邮件自动化平台',
  description:
    '肥猫猫提供邮箱管理、第三方邮箱接入、邮件自动化、批量收发与工作流市场能力，帮助团队统一处理多邮箱和邮件任务。',
  keywords:
    '邮箱管理,第三方邮箱,邮件自动化,多邮箱管理,邮件工具,工作流市场,mail management,workflow marketplace',
  canonicalPath: '/',
  robots: 'index, follow',
  ogType: 'website',
  lang: 'zh-CN'
}

const NOINDEX_PREFIXES = ['/auth/google', '/share/', '/payment', '/purchase']

const normalizePath = (path: string) => {
  if (!path || path === '/') return '/'
  return path.replace(/\/+$/, '')
}

const buildCanonicalUrl = (path: string) => {
  if (!path || path === '/') return `${SITE_URL}/`
  return `${SITE_URL}${path.startsWith('/') ? path : `/${path}`}`
}

const shouldNoIndex = (route: RouteLocationNormalizedLoaded) => {
  if (route.name === 'login') {
    return true
  }

  if (NOINDEX_PREFIXES.some(prefix => route.path === prefix || route.path.startsWith(prefix))) {
    return true
  }

  return route.matched.some(record => Boolean(record.meta.requiresAuth || record.meta.requiresAdmin))
}

const upsertMeta = (selector: string, attributes: Record<string, string>) => {
  let meta = document.head.querySelector(selector) as HTMLMetaElement | null

  if (!meta) {
    meta = document.createElement('meta')
    document.head.appendChild(meta)
  }

  Object.entries(attributes).forEach(([key, value]) => {
    meta?.setAttribute(key, value)
  })
}

const upsertLink = (rel: string, href: string) => {
  let link = document.head.querySelector(`link[rel="${rel}"]`) as HTMLLinkElement | null

  if (!link) {
    link = document.createElement('link')
    link.setAttribute('rel', rel)
    document.head.appendChild(link)
  }

  link.setAttribute('href', href)
}

export const applyPageSeo = (seo: PageSeo) => {
  const canonicalUrl = buildCanonicalUrl(seo.canonicalPath || '/')

  document.documentElement.setAttribute('lang', seo.lang || DEFAULT_SEO.lang || 'zh-CN')
  document.title = seo.title

  upsertMeta('meta[name="description"]', { name: 'description', content: seo.description })
  upsertMeta('meta[name="keywords"]', {
    name: 'keywords',
    content: seo.keywords || DEFAULT_SEO.keywords || ''
  })
  upsertMeta('meta[name="robots"]', {
    name: 'robots',
    content: seo.robots || DEFAULT_SEO.robots || 'index, follow'
  })
  upsertMeta('meta[property="og:title"]', { property: 'og:title', content: seo.title })
  upsertMeta('meta[property="og:description"]', {
    property: 'og:description',
    content: seo.description
  })
  upsertMeta('meta[property="og:type"]', {
    property: 'og:type',
    content: seo.ogType || DEFAULT_SEO.ogType || 'website'
  })
  upsertMeta('meta[property="og:url"]', { property: 'og:url', content: canonicalUrl })
  upsertMeta('meta[property="og:site_name"]', { property: 'og:site_name', content: SITE_NAME })
  upsertMeta('meta[property="og:locale"]', { property: 'og:locale', content: 'zh_CN' })
  upsertMeta('meta[name="twitter:card"]', { name: 'twitter:card', content: 'summary' })
  upsertMeta('meta[name="twitter:title"]', { name: 'twitter:title', content: seo.title })
  upsertMeta('meta[name="twitter:description"]', {
    name: 'twitter:description',
    content: seo.description
  })

  upsertLink('canonical', canonicalUrl)
}

export const resolveRouteSeo = (route: RouteLocationNormalizedLoaded): PageSeo => {
  const routeSeo = (route.meta.seo || {}) as Partial<PageSeo>

  return {
    ...DEFAULT_SEO,
    ...routeSeo,
    canonicalPath: routeSeo.canonicalPath || normalizePath(route.path),
    robots: routeSeo.robots || (shouldNoIndex(route) ? 'noindex, nofollow' : DEFAULT_SEO.robots)
  }
}

export const setPageSeo = (seo: Partial<PageSeo>) => {
  applyPageSeo({
    ...DEFAULT_SEO,
    canonicalPath: normalizePath(window.location.pathname),
    ...seo
  })
}

export const setupSeo = (router: Router) => {
  router.afterEach(to => {
    applyPageSeo(resolveRouteSeo(to))
  })

  router
    .isReady()
    .then(() => {
      applyPageSeo(resolveRouteSeo(router.currentRoute.value))
    })
    .catch(() => {})
}
