<script setup>
import { ref, onMounted } from 'vue'
import init, { sobel } from 'pixel-math-wasm'

const originalCanvasRef = ref(null)
const resultCanvasRef = ref(null)
const isLoaded = ref(false)

// 阈值滑块
const threshold = ref(100)

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

  const result = sobel(
    cachedImageData.data,
    width,
    height,
    threshold.value
  )

  ctx.putImageData(
    new ImageData(new Uint8ClampedArray(result), width, height),
    0, 0
  )
}
</script>

<template>
  <div class="page">
    <h2>Sobel 边缘检测</h2>
    <p class="desc">用 3×3 卷积核检测图片中的明暗突变（边缘）</p>

    <div class="controls">
      <label class="upload-btn">
        <input type="file" accept="image/*" @change="handleFileChange" />
        <span v-if="!isLoaded">WASM 加载中...</span>
        <span v-else>选择图片</span>
      </label>

      <div class="slider-group">
        <label>阈值：<span class="value">{{ threshold }}</span></label>
        <input
          type="range"
          min="0"
          max="255"
          v-model.number="threshold"
          @input="processImage"
        />
        <span class="hint">0=全边，100=推荐，255=最干净</span>
      </div>
    </div>

    <div class="canvas-wrapper">
      <div class="canvas-item">
        <div class="canvas-label">原图</div>
        <canvas ref="originalCanvasRef"></canvas>
      </div>
      <div class="canvas-item">
        <div class="canvas-label">Sobel 边缘（{{ threshold }}）</div>
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
