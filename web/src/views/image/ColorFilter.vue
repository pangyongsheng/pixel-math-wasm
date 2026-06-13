<script setup>
import { ref, onMounted, computed } from 'vue'
import init, { color_filter } from 'pixel-math-wasm'

const originalCanvasRef = ref(null)
const resultCanvasRef = ref(null)
const isLoaded = ref(false)

const selectedFilter = ref('sepia')

const filters = [
  { value: 'sepia',          label: '🎞️ 复古泛黄',     desc: '老照片味道，棕黄色调' },
  { value: 'cool',           label: '❄️ 冷色调',       desc: '减红增蓝，蓝绿清新' },
  { value: 'warm',           label: '🔥 暖色调',       desc: '增红减蓝，橙红温暖' },
  { value: 'red_boost',      label: '🔴 红通道增强',   desc: '红色更突出' },
  { value: 'green_boost',    label: '🟢 绿通道增强',   desc: '绿色更突出' },
  { value: 'blue_boost',     label: '🔵 蓝通道增强',   desc: '蓝色更突出' },
  { value: 'grayscale_tint', label: '🟡 怀旧黄染色',   desc: '灰度 + 暖色叠加' },
]

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

  const result = color_filter(
    cachedImageData.data,
    width,
    height,
    selectedFilter.value
  )
  ctx.putImageData(
    new ImageData(new Uint8ClampedArray(result), width, height),
    0, 0
  )
}

const currentLabel = computed(() => {
  return filters.find(f => f.value === selectedFilter.value)?.label ?? ''
})
</script>

<template>
  <div class="page">
    <h2>基础单色滤镜</h2>
    <p class="desc">7 种艺术滤镜，单选切换实时预览</p>

    <div class="controls">
      <label class="upload-btn">
        <input type="file" accept="image/*" @change="handleFileChange" />
        <span v-if="!isLoaded">WASM 加载中...</span>
        <span v-else>选择图片</span>
      </label>
    </div>

    <div class="filter-grid">
      <button
        v-for="f in filters"
        :key="f.value"
        class="filter-btn"
        :class="{ active: selectedFilter === f.value }"
        @click="selectedFilter = f.value; processImage()"
      >
        <div class="filter-label">{{ f.label }}</div>
        <div class="filter-desc">{{ f.desc }}</div>
      </button>
    </div>

    <div class="canvas-wrapper">
      <div class="canvas-item">
        <div class="canvas-label">原图</div>
        <canvas ref="originalCanvasRef"></canvas>
      </div>
      <div class="canvas-item">
        <div class="canvas-label">{{ currentLabel }}</div>
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

.filter-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: 12px;
  margin-bottom: 30px;
}
.filter-btn {
  padding: 12px 16px;
  background: #1a1a2e;
  color: #fff;
  border: 1px solid #333;
  border-radius: 6px;
  cursor: pointer;
  text-align: left;
  transition: all 0.2s;
}
.filter-btn:hover {
  border-color: #3b5bdb;
}
.filter-btn.active {
  background: #3b5bdb;
  border-color: #3b5bdb;
}
.filter-label {
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 4px;
}
.filter-desc {
  font-size: 12px;
  color: #aaa;
}
.filter-btn.active .filter-desc {
  color: #ddd;
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
