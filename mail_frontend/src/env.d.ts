/// <reference types="vite/client" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}

declare module 'qrcode' {
  interface QRCodeToCanvasOptions {
    width?: number
    margin?: number
    color?: {
      dark?: string
      light?: string
    }
  }

  const QRCode: {
    toCanvas(
      canvasElement: HTMLCanvasElement,
      text: string,
      options?: QRCodeToCanvasOptions
    ): Promise<void>
  }

  export default QRCode
}
