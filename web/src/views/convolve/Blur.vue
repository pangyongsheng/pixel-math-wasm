<script setup>
import { ref, onMounted } from 'vue'
import init, { mean_blur, median_blur } from 'pixel-math-wasm'

const originalCanvasRef = ref(null)
const resultCanvasRef = ref(null)
const isLoaded = ref(false)

// 当前选中的算法
const algo = ref('mean')
// 模糊强度（迭代次数）
const iterations = ref(3)

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

  const fn = algo.value === 'mean' ? mean_blur : median_blur
  const result = fn(cachedImageData.data, width, height, iterations.value)

  ctx.putImageData(
    new ImageData(new Uint8ClampedArray(result), width, height),
    0, 0
  )
}
</script>

<template>
  <div class="page">
    <h2>均值模糊 & 中值模糊</h2>
    <p class="desc">对每个像素用 3×3 邻域做平均或取中位数</p>

    <div class="controls">
      <label class="upload-btn">
        <input type="file" accept="image/*" @change="handleFileChange" />
        <span v-if="!isLoaded">WASM 加载中...</span>
        <span v-else>选择图片</span>
      </label>

      <div class="algo-group">
        <label class="radio-label">
          <input type="radio" value="mean" v-model="algo" @change="processImage" />
          均值模糊
        </label>
        <label class="radio-label">
          <input type="radio" value="median" v-model="algo" @change="processImage" />
          中值模糊
        </label>
      </div>

      <div class="slider-group">
        <label>模糊强度（迭代次数）：<span class="value">{{ iterations }}</span></label>
        <input
          type="range"
          min="1"
          max="5"
          v-model.number="iterations"
          @input="processImage"
        />
        <span class="hint">1=3×3 微弱，3=≈7×7 明显，5=≈11×11 强模糊</span>
      </div>
    </div>

    <div class="canvas-wrapper">
      <div class="canvas-item">
        <div class="canvas-label">原图</div>
        <canvas ref="originalCanvasRef"></canvas>
      </div>
      <div class="canvas-item">
        <div class="canvas-label">
          {{ algo === 'mean' ? '均值模糊' : '中值模糊' }} × {{ iterations }}
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

.algo-group {
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
  min-width: 24px;
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
