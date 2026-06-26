<script setup>
import { ref, onMounted, computed } from 'vue'
import init, { vignette } from 'pixel-math-wasm'

const originalCanvasRef = ref(null)
const resultCanvasRef = ref(null)
const isLoaded = ref(false)

// 模式：vignette / vintage
const kind = ref('vignette')
// 强度
const strength = ref(0.6)

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

  const result = vignette(
    cachedImageData.data,
    width,
    height,
    kind.value,
    strength.value
  )

  ctx.putImageData(
    new ImageData(new Uint8ClampedArray(result), width, height),
    0, 0
  )
}

const label = computed(() => kind.value === 'vignette' ? '暗角' : '复古胶片')
</script>

<template>
  <div class="page">
    <h2>暗角 & 复古胶片</h2>
    <p class="desc">四周衰减中心高亮，支持纯暗角和复古胶片两种模式</p>

    <div class="controls">
      <label class="upload-btn">
        <input type="file" accept="image/*" @change="handleFileChange" />
        <span v-if="!isLoaded">WASM 加载中...</span>
        <span v-else>选择图片</span>
      </label>

      <div class="mode-group">
        <label class="radio-label">
          <input type="radio" value="vignette" v-model="kind" @change="processImage" />
          暗角
        </label>
        <label class="radio-label">
          <input type="radio" value="vintage" v-model="kind" @change="processImage" />
          复古胶片
        </label>
      </div>

      <div class="slider-group">
        <label>强度：<span class="value">{{ strength.toFixed(2) }}</span></label>
        <input
          type="range"
          min="0"
          max="1"
          step="0.05"
          v-model.number="strength"
          @input="processImage"
        />
        <span class="hint">0=无效果，1=最强</span>
      </div>
    </div>

    <div class="canvas-wrapper">
      <div class="canvas-item">
        <div class="canvas-label">原图</div>
        <canvas ref="originalCanvasRef"></canvas>
      </div>
      <div class="canvas-item">
        <div class="canvas-label">{{ label }}（{{ strength.toFixed(2) }}）</div>
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
  white-space: nowrap;
}
.slider-group .value {
  color: #5c7cfa;
  font-weight: 600;
  display: inline-block;
  min-width: 36px;
  text-align: center;
}
.slider-group input[type="range"] {
  width: 160px;
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
