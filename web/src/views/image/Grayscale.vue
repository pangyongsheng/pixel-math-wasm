<script setup>
import { ref, computed, onMounted } from 'vue'
import init, { grayscale_with } from 'pixel-math-wasm'

const originalCanvasRef = ref(null)
const grayscaleCanvasRef = ref(null)
const isLoaded = ref(false)

// 当前选中的算法
const selectedAlgo = ref('luminance')

// 算法选项：值对应 Rust 端 GrayAlgorithm 接受的字符串
const algorithms = [
  { value: 'luminance', label: '加权平均 BT.601', desc: '最常用，最符合人眼感知' },
  { value: 'luminance709', label: '加权平均 BT.709', desc: 'HD/现代显示器标准' },
  { value: 'average', label: '简单平均', desc: '(R+G+B)/3，效果偏平' },
  { value: 'max', label: '最大值法', desc: 'max(R,G,B)，结果偏亮' },
  { value: 'min', label: '最小值法', desc: 'min(R,G,B)，结果偏暗' },
  { value: 'desaturate', label: '去饱和法', desc: '(max+min)/2，PS 早期算法' },
]

// 缓存最近一次的原图数据，切换算法时无需重新选图
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
  // 画到原图画布上，保留原始像素
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
  const grayscaleCanvas = grayscaleCanvasRef.value
  const grayscaleCtx = grayscaleCanvas.getContext('2d')
  grayscaleCanvas.width = width
  grayscaleCanvas.height = height

  const result = grayscale_with(cachedImageData.data, width, height, selectedAlgo.value)
  grayscaleCtx.putImageData(
    new ImageData(new Uint8ClampedArray(result), width, height),
    0,
    0
  )
}

// 切换算法时实时重新处理
function onAlgoChange() {
  processImage()
}

// 当前算法的描述（随选择自动更新）
const currentDesc = computed(() => {
  return algorithms.find(a => a.value === selectedAlgo.value)?.desc ?? ''
})
const currentLabel = computed(() => {
  return algorithms.find(a => a.value === selectedAlgo.value)?.label ?? ''
})
</script>

<template>
  <div class="grayscale-page">
    <h2>灰度化处理</h2>
    <p class="desc">支持 6 种常见灰度算法，可实时切换对比效果</p>

    <div class="controls">
      <label class="upload-btn">
        <input type="file" accept="image/*" @change="handleFileChange" />
        <span v-if="!isLoaded">WASM 加载中...</span>
        <span v-else>选择图片</span>
      </label>

      <div class="algo-group">
        <label class="algo-label">算法：</label>
        <select v-model="selectedAlgo" @change="onAlgoChange" class="algo-select">
          <option
            v-for="algo in algorithms"
            :key="algo.value"
            :value="algo.value"
          >
            {{ algo.label }}
          </option>
        </select>
        <span class="algo-desc">{{ currentDesc }}</span>
      </div>
    </div>

    <div class="canvas-wrapper">
      <div class="canvas-item">
        <div class="canvas-label">原图</div>
        <canvas ref="originalCanvasRef"></canvas>
      </div>
      <div class="canvas-item">
        <div class="canvas-label">灰度图（{{ currentLabel }}）</div>
        <canvas ref="grayscaleCanvasRef"></canvas>
      </div>
    </div>
  </div>
</template>

<style scoped>
.grayscale-page {
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
.algo-group {
  display: flex;
  align-items: center;
  gap: 10px;
}
.algo-label {
  color: #888;
  font-size: 14px;
}
.algo-select {
  padding: 8px 12px;
  border: 1px solid #3b5bdb;
  background: #1a1a2e;
  color: #fff;
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
  outline: none;
}
.algo-select:hover {
  border-color: #5c7cfa;
}
.algo-desc {
  color: #5c7cfa;
  font-size: 13px;
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
