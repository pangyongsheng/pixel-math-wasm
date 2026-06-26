<script setup>
import { ref, onMounted } from 'vue'
import init, { mosaic } from 'pixel-math-wasm'

const originalCanvasRef = ref(null)
const resultCanvasRef = ref(null)
const isLoaded = ref(false)

// 马赛克块大小
const blockSize = ref(10)

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
  const canvas = originalCanvasRef.value
  const ctx = canvas.getContext('2d')
  canvas.width = img.width
  canvas.height = img.height
  ctx.drawImage(img, 0, 0)

  cachedImage = img
  cachedImageData = ctx.getImageData(0, 0, img.width, img.height)
}

function processImage() {
  if (!cachedImage || !cachedImageData) return

  const { width, height } = cachedImage
  const canvas = resultCanvasRef.value
  const ctx = canvas.getContext('2d')
  canvas.width = width
  canvas.height = height

  const result = mosaic(cachedImageData.data, width, height, blockSize.value)

  ctx.putImageData(
    new ImageData(new Uint8ClampedArray(result), width, height),
    0, 0
  )
}
</script>

<template>
  <div class="page">
    <h2>马赛克像素化</h2>
    <p class="desc">把图片分块，每块取平均色填充整块</p>

    <div class="controls">
      <label class="upload-btn">
        <input type="file" accept="image/*" @change="handleFileChange" />
        <span v-if="!isLoaded">WASM 加载中...</span>
        <span v-else>选择图片</span>
      </label>

      <div class="slider-group">
        <label>马赛克块大小：<span class="value">{{ blockSize }}</span></label>
        <input
          type="range"
          min="2"
          max="50"
          v-model.number="blockSize"
          @input="processImage"
        />
        <span class="hint">2=极细，10=标准，30=强马赛克</span>
      </div>
    </div>

    <div class="canvas-wrapper">
      <div class="canvas-item">
        <div class="canvas-label">原图</div>
        <canvas ref="originalCanvasRef"></canvas>
      </div>
      <div class="canvas-item">
        <div class="canvas-label">马赛克 {{ blockSize }}×{{ blockSize }}</div>
        <canvas ref="resultCanvasRef"></canvas>
      </div>
    </div>
  </div>
</template>

<style scoped>
.page {
  padding: 40px;
}
h2 { margin-bottom: 10px; }
.desc {
  color: #888;
  margin-bottom: 30px;
}
.controls {
  margin-bottom: 30px;
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 24px;
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
.upload-btn:hover { background: #3450d1; }
.upload-btn input { display: none; }

.slider-group {
  display: flex;
  align-items: center;
  gap: 12px;
}
.slider-group label {
  color: #888;
  font-size: 14px;
  white-space: nowrap;
}
.slider-group .value {
  color: #5c7cfa;
  font-weight: 600;
  display: inline-block;
  min-width: 32px;
  text-align: center;
}
.slider-group input[type="range"] {
  width: 200px;
  cursor: pointer;
  accent-color: #3b5bdb;
}
.slider-group .hint {
  color: #666;
  font-size: 12px;
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
