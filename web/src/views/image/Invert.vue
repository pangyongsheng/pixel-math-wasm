<script setup>
import { ref, onMounted } from 'vue'
import init, { invert } from 'pixel-math-wasm'

const originalCanvasRef = ref(null)
const invertCanvasRef = ref(null)
const isLoaded = ref(false)

// 缓存原图数据
let cachedImage = null
let cachedImageData = null

onMounted(async () => {
  await init()
  isLoaded.value = true
})

function handleFileChange(e) {
  const file = e.target.files[0]
  if (!file) return

  const reader = new FileReader()
  reader.onload = (event) => {
    const img = new Image()
    img.onload = () => {
      cacheImage(img)
      processImage()
    }
    img.src = event.target.result
  }
  reader.readAsDataURL(file)
}

function cacheImage(img) {
  const originalCanvas = originalCanvasRef.value
  const originalCtx = originalCanvas.getContext('2d')
  originalCanvas.width = img.width
  originalCanvas.height = img.height
  originalCtx.drawImage(img, 0, 0)

  cachedImage = img
  cachedImageData = originalCtx.getImageData(0, 0, img.width, img.height)
}

function processImage() {
  if (!cachedImage || !cachedImageData) return

  const { width, height } = cachedImage
  const invertCanvas = invertCanvasRef.value
  const invertCtx = invertCanvas.getContext('2d')
  invertCanvas.width = width
  invertCanvas.height = height

  // 调用 Rust 端的 invert 函数
  const result = invert(cachedImageData.data, width, height)
  invertCtx.putImageData(
    new ImageData(new Uint8ClampedArray(result), width, height),
    0,
    0
  )
}
</script>

<template>
  <div class="invert-page">
    <h2>反色 / 负片效果</h2>
    <p class="desc">RGB 三通道各自取反（255 - 原值），生成底片风格，Alpha 通道保持不变。</p>

    <div class="controls">
      <label class="upload-btn">
        <input type="file" accept="image/*" @change="handleFileChange" />
        <span v-if="!isLoaded">WASM 加载中...</span>
        <span v-else>选择图片</span>
      </label>
    </div>

    <div class="canvas-wrapper">
      <div class="canvas-item">
        <div class="canvas-label">原图</div>
        <canvas ref="originalCanvasRef"></canvas>
      </div>
      <div class="canvas-item">
        <div class="canvas-label">反色后</div>
        <canvas ref="invertCanvasRef"></canvas>
      </div>
    </div>
  </div>
</template>

<style scoped>
.invert-page {
  padding: 40px;
}
h2 {
  margin-bottom: 10px;
}
.desc {
  color: #888;
  margin-bottom: 30px;
}
.controls {
  margin-bottom: 30px;
}
.upload-btn {
  display: inline-block;
  padding: 10px 24px;
  background: #3b5bdb;
  color: #fff;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  user-select: none;
}
.upload-btn:hover {
  background: #3450d1;
}
.upload-btn input {
  display: none;
}
.canvas-wrapper {
  display: flex;
  gap: 40px;
  flex-wrap: wrap;
}
.canvas-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}
.canvas-label {
  color: #888;
  font-size: 14px;
}
canvas {
  max-width: 300px;
  max-height: 300px;
  object-fit: contain;
  background: #1a1a2e;
  border-radius: 8px;
  padding: 8px;
}
</style>
