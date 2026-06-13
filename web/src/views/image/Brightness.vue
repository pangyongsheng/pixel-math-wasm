<script setup>
import { ref, onMounted } from 'vue'
import init, { brightness_contrast } from 'pixel-math-wasm'

const originalCanvasRef = ref(null)
const resultCanvasRef = ref(null)
const isLoaded = ref(false)

// 滑块值
const brightness = ref(0)
const contrast = ref(0)

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

  // 调用 Rust 端的 brightness_contrast 函数
  const result = brightness_contrast(
    cachedImageData.data,
    width,
    height,
    brightness.value,
    contrast.value
  )
  ctx.putImageData(
    new ImageData(new Uint8ClampedArray(result), width, height),
    0, 0
  )
}

// 滑块拖动时实时更新
function onSliderChange() {
  processImage()
}

function resetSliders() {
  brightness.value = 0
  contrast.value = 0
  processImage()
}
</script>

<template>
  <div class="page">
    <h2>亮度 / 对比度调节</h2>
    <p class="desc">拖动滑块实时调整，左边原图，右边结果</p>

    <div class="controls">
      <label class="upload-btn">
        <input type="file" accept="image/*" @change="handleFileChange" />
        <span v-if="!isLoaded">WASM 加载中...</span>
        <span v-else>选择图片</span>
      </label>

      <div class="slider-group">
        <label>亮度：<span class="value">{{ brightness }}</span></label>
        <input
          type="range"
          min="-100"
          max="100"
          v-model.number="brightness"
          @input="onSliderChange"
        />
      </div>

      <div class="slider-group">
        <label>对比度：<span class="value">{{ contrast }}</span></label>
        <input
          type="range"
          min="-100"
          max="100"
          v-model.number="contrast"
          @input="onSliderChange"
        />
      </div>

      <button @click="resetSliders" class="reset-btn">重置</button>
    </div>

    <div class="canvas-wrapper">
      <div class="canvas-item">
        <div class="canvas-label">原图</div>
        <canvas ref="originalCanvasRef"></canvas>
      </div>
      <div class="canvas-item">
        <div class="canvas-label">调整后</div>
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
.slider-group {
  display: flex;
  align-items: center;
  gap: 12px;
}
.slider-group label {
  color: #888;
  font-size: 14px;
  min-width: 110px;
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
.reset-btn {
  padding: 10px 20px;
  background: transparent;
  color: #888;
  border: 1px solid #495057;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
}
.reset-btn:hover {
  border-color: #3b5bdb;
  color: #fff;
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
