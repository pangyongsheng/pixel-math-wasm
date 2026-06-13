<script setup>
import { ref, onMounted } from 'vue'
import init, { threshold_with, threshold_otsu } from 'pixel-math-wasm'

const originalCanvasRef = ref(null)
const resultCanvasRef = ref(null)
const isLoaded = ref(false)

// 模式：manual（手动） 或 otsu（自动）
const mode = ref('otsu')
// 手动阈值
const manualThreshold = ref(128)

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

  // 根据模式选不同函数
  const result = mode.value === 'otsu'
    ? threshold_otsu(cachedImageData.data, width, height)
    : threshold_with(cachedImageData.data, width, height, manualThreshold.value)

  ctx.putImageData(
    new ImageData(new Uint8ClampedArray(result), width, height),
    0, 0
  )
}

function onModeChange() {
  processImage()
}

function onThresholdChange() {
  if (mode.value === 'manual') processImage()
}
</script>

<template>
  <div class="page">
    <h2>二值化阈值分割</h2>
    <p class="desc">把图片变成纯黑 + 纯白，支持手动调阈值和 Otsu 自动选阈值</p>

    <div class="controls">
      <label class="upload-btn">
        <input type="file" accept="image/*" @change="handleFileChange" />
        <span v-if="!isLoaded">WASM 加载中...</span>
        <span v-else>选择图片</span>
      </label>

      <div class="mode-group">
        <label class="radio-label">
          <input
            type="radio"
            value="manual"
            v-model="mode"
            @change="onModeChange"
          />
          手动阈值
        </label>
        <label class="radio-label">
          <input
            type="radio"
            value="otsu"
            v-model="mode"
            @change="onModeChange"
          />
          Otsu 自动
        </label>
      </div>

      <div class="slider-group" v-if="mode === 'manual'">
        <label>阈值：<span class="value">{{ manualThreshold }}</span></label>
        <input
          type="range"
          min="0"
          max="255"
          v-model.number="manualThreshold"
          @input="onThresholdChange"
        />
      </div>
    </div>

    <div class="canvas-wrapper">
      <div class="canvas-item">
        <div class="canvas-label">原图</div>
        <canvas ref="originalCanvasRef"></canvas>
      </div>
      <div class="canvas-item">
        <div class="canvas-label">
          二值化（{{ mode === 'otsu' ? 'Otsu 自动' : `阈值 ${manualThreshold}` }}）
        </div>
        <canvas ref="resultCanvasRef"></canvas>
      </div>
    </div>
  </div>
</template>

<style scoped>
.page {
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
.upload-btn:hover {
  background: #3450d1;
}
.upload-btn input {
  display: none;
}
.mode-group {
  display: flex;
  gap: 16px;
}
.radio-label {
  display: flex;
  align-items: center;
  gap: 6px;
  color: #888;
  font-size: 14px;
  cursor: pointer;
}
.radio-label input {
  accent-color: #3b5bdb;
  cursor: pointer;
}
.slider-group {
  display: flex;
  align-items: center;
  gap: 12px;
}
.slider-group label {
  color: #888;
  font-size: 14px;
  min-width: 90px;
}
.slider-group .value {
  color: #5c7cfa;
  font-weight: 600;
  display: inline-block;
  min-width: 40px;
}
.slider-group input[type="range"] {
  width: 200px;
  cursor: pointer;
  accent-color: #3b5bdb;
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
