import * as echarts from 'echarts'
import { nextTick } from 'vue'
import chinaMapData from '@/assets/echarts/china.json'

// 中国省份名称映射 - 将后端简称映射到地图文件中的完整名称
const chinaProvinceMap: { [key: string]: string } = {
  // 直辖市
  '北京市': '北京市', '北京': '北京市',
  '天津市': '天津市', '天津': '天津市',
  '上海市': '上海市', '上海': '上海市',
  '重庆市': '重庆市', '重庆': '重庆市',
  // 省份
  '河北省': '河北省', '河北': '河北省',
  '山西省': '山西省', '山西': '山西省',
  '辽宁省': '辽宁省', '辽宁': '辽宁省',
  '吉林省': '吉林省', '吉林': '吉林省',
  '黑龙江省': '黑龙江省', '黑龙江': '黑龙江省',
  '江苏省': '江苏省', '江苏': '江苏省',
  '浙江省': '浙江省', '浙江': '浙江省',
  '安徽省': '安徽省', '安徽': '安徽省',
  '福建省': '福建省', '福建': '福建省',
  '江西省': '江西省', '江西': '江西省',
  '山东省': '山东省', '山东': '山东省',
  '河南省': '河南省', '河南': '河南省',
  '湖北省': '湖北省', '湖北': '湖北省',
  '湖南省': '湖南省', '湖南': '湖南省',
  '广东省': '广东省', '广东': '广东省',
  '海南省': '海南省', '海南': '海南省',
  '四川省': '四川省', '四川': '四川省',
  '贵州省': '贵州省', '贵州': '贵州省',
  '云南省': '云南省', '云南': '云南省',
  '陕西省': '陕西省', '陕西': '陕西省',
  '甘肃省': '甘肃省', '甘肃': '甘肃省',
  '青海省': '青海省', '青海': '青海省',
  // 自治区
  '广西壮族自治区': '广西壮族自治区', '广西': '广西壮族自治区',
  '西藏自治区': '西藏自治区', '西藏': '西藏自治区',
  '宁夏回族自治区': '宁夏回族自治区', '宁夏': '宁夏回族自治区',
  '新疆维吾尔自治区': '新疆维吾尔自治区', '新疆': '新疆维吾尔自治区',
  '内蒙古自治区': '内蒙古自治区', '内蒙古': '内蒙古自治区',
  // 特别行政区
  '香港特别行政区': '香港', '香港': '香港',
  '澳门特别行政区': '澳门', '澳门': '澳门',
  '台湾省': '台湾', '台湾': '台湾'
}

export interface GeoDistributionData {
  distribution: Array<{
    name: string
    visit_count?: number
    mailbox_count?: number
    total_users?: number
    reg_user_count?: number
    temp_user_count?: number
  }>
}

// 预加载世界地图数据
let worldMapLoaded = false
export const preloadWorldMap = async () => {
  if (worldMapLoaded) return
  try {
    ;(window as any).echarts = echarts

    // 在开发环境和生产环境使用不同的加载方式
    if ((import.meta as any).env?.DEV) {
      // 开发环境：使用动态导入
      await import('@/assets/echarts/world.js' as any)
      worldMapLoaded = true
      console.log('✅ 世界地图数据加载成功 (开发环境)')
    } else {
      // 生产环境：使用 public 目录中的文件
      const worldMapUrl = '/assets/echarts/world.js'
      console.log('🔄 尝试加载世界地图文件:', worldMapUrl)

      await new Promise((resolve, reject) => {
        const script = document.createElement('script')
        script.src = worldMapUrl
        script.onload = () => {
          worldMapLoaded = true
          console.log('✅ 世界地图数据加载成功 (生产环境)')
          console.log('📍 ECharts 世界地图已注册:', (window as any).echarts?.getMap?.('world'))
          resolve(true)
        }
        script.onerror = (error) => {
          console.error('❌ 加载世界地图脚本失败:', error)
          console.error('📍 请求的URL:', worldMapUrl)
          reject(error)
        }
        document.head.appendChild(script)
      })
    }
  } catch (error) {
    console.error('❌ 预加载世界地图失败:', error)
  }
}

// 加载中国地图
const loadChinaMap = async (mapChart: echarts.ECharts) => {
  if (!mapChart) return
  echarts.registerMap('china', chinaMapData as any)
}

// 加载世界地图
const loadWorldMap = async () => {
  try {
    if (!worldMapLoaded) {
      ;(window as any).echarts = echarts

      // 在开发环境和生产环境使用不同的加载方式
      if ((import.meta as any).env?.DEV) {
        // 开发环境：使用动态导入
        await import('@/assets/echarts/world.js' as any)
        worldMapLoaded = true
        console.log('✅ 世界地图数据加载成功 (开发环境)')
      } else {
        // 生产环境：使用 public 目录中的文件
        const worldMapUrl = '/assets/echarts/world.js'

        await new Promise((resolve, reject) => {
          const script = document.createElement('script')
          script.src = worldMapUrl
          script.onload = () => {
            worldMapLoaded = true
            console.log('✅ 世界地图数据加载成功 (生产环境)')
            resolve(true)
          }
          script.onerror = (error) => {
            console.error('❌ 加载世界地图脚本失败:', error)
            reject(error)
          }
          document.head.appendChild(script)
        })
      }
    }
  } catch (error) {
    console.error('❌ 加载世界地图失败:', error)
    return
  }
}

// 更新中国地图数据
const updateChinaMap = (mapChart: echarts.ECharts, geoDistribution: GeoDistributionData) => {
  if (!mapChart) return

  // 准备中国省份数据
  const chinaData: any[] = []
  const hasData = geoDistribution?.distribution?.length > 0

  // 所有省份列表 - 使用地图文件中的完整名称
  const allProvinces = [
    '北京市', '天津市', '上海市', '重庆市', '河北省', '山西省', '辽宁省', '吉林省',
    '黑龙江省', '江苏省', '浙江省', '安徽省', '福建省', '江西省', '山东省', '河南省',
    '湖北省', '湖南省', '广东省', '广西壮族自治区', '海南省', '四川省', '贵州省', '云南省',
    '西藏自治区', '陕西省', '甘肃省', '青海省', '宁夏回族自治区', '新疆维吾尔自治区', '内蒙古自治区',
    '香港特别行政区', '澳门特别行政区', '台湾省'
  ]

  // 初始化所有省份为0
  allProvinces.forEach(province => {
    chinaData.push({
      name: province,
      value: 0
    })
  })

  // 如果有数据，更新对应省份的值
  if (hasData) {
    geoDistribution.distribution.forEach((item: any) => {
      const provinceName = chinaProvinceMap[item.name] || item.name
      const userCount = item.total_users || 0

      const existingProvince = chinaData.find(p => p.name === provinceName)
      if (existingProvince) {
        existingProvince.value = userCount
      }
    })
  }

  const maxValue = Math.max(...chinaData.map(item => item.value))

  console.log('🗺️ 地图数据数量:', chinaData.length, '最大值:', maxValue)
  console.log('� 有数据的省份:', chinaData.filter(item => item.value > 0))

  const option = {
    title: {
      text: '中国省份用户分布',
      left: 'center',
      top: 20,
      textStyle: {
        fontSize: 16,
        color: '#374151'
      }
    },
    tooltip: {
      trigger: 'item',
      formatter: function(params: any) {
        if (params.value && params.value > 0) {
          return `<strong>${params.name}</strong><br/>用户数: ${params.value}`
        }
        return `<strong>${params.name}</strong><br/>用户数: 0`
      }
    },
    visualMap: {
      min: 0,
      max: 8000,
      left: 'left',
      top: 'bottom',
      text: ['高', '低'],
      pieces: [
        {min: 0, max: 0, color: '#f0f0f0', label: '0'},
        {min: 1, max: 500, color: '#e0f2fe', label: '1-500'},
        {min: 501, max: 2000, color: '#87CEEB', label: '501-2000'},
        {min: 2001, max: 8000, color: '#4169E1', label: '2001-8000'},
        {min: 8001, color: '#1e3a8a', label: '8000+'}
      ],
      show: true
    },
    series: [{
      name: '中国地图',
      type: 'map',
      map: 'china',
      roam: false,
      label: {
        show: false
      },
      itemStyle: {
        borderColor: '#999',
        borderWidth: 0.5,
        areaColor: '#e0e0e0'
      },
      emphasis: {
        label: {
          show: true,
          color: '#fff',
          fontSize: 14,
          fontWeight: 'bold'
        },
        itemStyle: {
          areaColor: '#3b82f6'
        }
      },
      data: chinaData
    }]
  }



  // 强制清空并重新设置
  mapChart.clear()
  mapChart.setOption(option, true)
}

// 国家名称映射（中文到英文）
const countryNameMap: { [key: string]: string } = {
  '中国': 'China',
  '美国': 'United States of America',
  '日本': 'Japan',
  '韩国': 'South Korea',
  '英国': 'United Kingdom',
  '法国': 'France',
  '德国': 'Germany',
  '俄罗斯': 'Russia',
  '印度': 'India',
  '巴西': 'Brazil',
  '加拿大': 'Canada',
  '澳大利亚': 'Australia',
  '意大利': 'Italy',
  '西班牙': 'Spain',
  '荷兰': 'Netherlands',
  '瑞士': 'Switzerland',
  '瑞典': 'Sweden',
  '挪威': 'Norway',
  '丹麦': 'Denmark',
  '芬兰': 'Finland'
}

// 英文到中文的反向映射
const englishToChineseMap: { [key: string]: string } = {}
Object.keys(countryNameMap).forEach(chinese => {
  englishToChineseMap[countryNameMap[chinese]] = chinese
})

// 更新世界地图数据
const updateWorldMapData = (mapChart: echarts.ECharts, geoDistribution: GeoDistributionData) => {
  if (!mapChart) return

  console.log('🌍 世界地图原始数据:', geoDistribution?.distribution)

  // 准备世界地图数据
  const worldData: any[] = []
  const hasData = geoDistribution?.distribution?.length > 0

  if (hasData) {
    geoDistribution.distribution.forEach((item: any) => {
      const chineseName = item.name
      const englishName = countryNameMap[chineseName] || chineseName
      const userCount = item.total_users || 0

      console.log(`🔍 处理国家: ${chineseName} -> ${englishName}, 用户数: ${userCount}`)

      if (userCount > 0) {
        worldData.push({
          name: englishName,
          value: userCount
        })
        console.log(`✅ 添加国家数据: ${englishName} = ${userCount}`)
      }
    })
  }

  console.log('🗺️ 最终世界地图数据:', worldData)



  const option = {
    title: {
      text: '全球国家用户分布',
      left: 'center',
      top: 20,
      textStyle: {
        fontSize: 16,
        color: '#374151'
      }
    },
    tooltip: {
      trigger: 'item',
      formatter: function(params: any) {
        const chineseName = englishToChineseMap[params.name] || params.name
        if (params.value && params.value > 0) {
          return `<strong>${chineseName}</strong><br/>用户数: ${params.value}`
        }
        return `<strong>${chineseName}</strong><br/>用户数: 0`
      }
    },
    visualMap: {
      min: 0,
      max: 8000,
      left: 'left',
      top: 'bottom',
      text: ['高', '低'],
      pieces: [
        {min: 0, max: 0, color: '#f0f0f0', label: '0'},
        {min: 1, max: 500, color: '#e0f2fe', label: '1-500'},
        {min: 501, max: 2000, color: '#87CEEB', label: '501-2000'},
        {min: 2001, max: 8000, color: '#4169E1', label: '2001-8000'},
        {min: 8001, color: '#1e3a8a', label: '8000+'}
      ],
      show: true
    },
    series: [{
      name: '世界地图',
      type: 'map',
      map: 'world',
      roam: false,
      label: {
        show: false
      },
      itemStyle: {
        borderColor: '#999',
        borderWidth: 0.5,
        areaColor: '#e0e0e0'
      },
      emphasis: {
        label: {
          show: true,
          color: '#fff',
          fontSize: 14,
          fontWeight: 'bold'
        },
        itemStyle: {
          areaColor: '#3b82f6'
        }
      },
      data: worldData
    }]
  }

  console.log('📋 世界地图ECharts配置:', JSON.stringify({
    visualMap: {min: option.visualMap.min, max: option.visualMap.max, show: option.visualMap.show},
    seriesData: option.series[0].data,
    mapName: option.series[0].map
  }, null, 2))

  // 强制清空并重新设置
  mapChart.clear()
  mapChart.setOption(option, true)
  console.log('🎨 世界地图配置已设置')
}

// 更新世界地图
export const updateWorldMap = async (
  container: HTMLDivElement,
  geoMode: string,
  geoDistribution: GeoDistributionData,
  mapChart: echarts.ECharts | null
) => {
  // 等待 DOM 更新
  await nextTick()

  // 等待容器有正确的尺寸，多次尝试
  let attempts = 0
  while (attempts < 10 && (!container || container.clientWidth === 0)) {
    await new Promise(resolve => setTimeout(resolve, 50))
    attempts++
  }

  if (!container || container.clientWidth === 0 || container.clientHeight === 0) {
    console.error('❌ 容器尺寸为0:', container?.clientWidth, container?.clientHeight)
    return null
  }

  try {
    // 销毁旧图表
    if (mapChart) {
      mapChart.dispose()
      mapChart = null
    }

    mapChart = echarts.init(container)
  } catch (error) {
    console.error('❌ 初始化图表失败:', error)
    return null
  }

  if (geoMode === 'china') {
    await loadChinaMap(mapChart)
    updateChinaMap(mapChart, geoDistribution)
  } else {
    await loadWorldMap()
    updateWorldMapData(mapChart, geoDistribution)
  }

  // 确保图表自适应容器大小
  setTimeout(() => {
    if (mapChart) {
      mapChart.resize()
    }
  }, 100)

  return mapChart
}
