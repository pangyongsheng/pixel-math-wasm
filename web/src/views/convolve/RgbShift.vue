<script setup>
import { ref, onMounted } from 'vue'
import init, { rgb_shift } from 'pixel-math-wasm'

const originalCanvasRef = ref(null)
const resultCanvasRef = ref(null)
const isLoaded = ref(false)

// 模式：horizontal / random
const kind = ref('random')
// 偏移量
const offset = ref(8)

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

  const result = rgb_shift(
    cachedImageData.data,
    width,
    height,
    kind.value,
    offset.value
  )

  ctx.putImageData(
    new ImageData(new Uint8ClampedArray(result), width, height),
    0, 0
  )
}
</script>

<template>
  <div class="page">
    <h2>故障风 RGB 通道偏移</h2>
    <p class="desc">R、B 通道分别偏移，制造赛博朋克重影效果</p>

    <div class="controls">
      <label class="upload-btn">
        <input type="file" accept="image/*" @change="handleFileChange" />
        <span v-if="!isLoaded">WASM 加载中...</span>
        <span v-else>选择图片</span>
      </label>

      <div class="mode-group">
        <label class="radio-label">
          <input type="radio" value="horizontal" v-model="kind" @change="processImage" />
          整齐偏移
        </label>
        <label class="radio-label">
          <input type="radio" value="random" v-model="kind" @change="processImage" />
          随机偏移
        </label>
      </div>

      <div class="slider-group">
        <label>偏移量：<span class="value">{{ offset }}</span> 像素</label>
        <input
          type="range"
          min="0"
          max="30"
          v-model.number="offset"
          @input="processImage"
        />
        <span class="hint">0=无效果，5=轻微，15=明显</span>
      </div>
    </div>

    <div class="canvas-wrapper">
      <div class="canvas-item">
        <div class="canvas-label">原图</div>
        <canvas ref="originalCanvasRef"></canvas>
      </div>
      <div class="canvas-item">
        <div class="canvas-label">
          {{ kind === 'random' ? '随机偏移' : '整齐偏移' }}（{{ offset }}px）
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
  min-width: 28px;
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
