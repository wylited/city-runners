<template>
  <div class="relative w-full h-full overflow-hidden bg-background">
    <div
      class="w-full h-full overflow-hidden touch-none"
      ref="mapWrapper"
      @touchstart="handleTouchStart"
      @touchmove="handleTouchMove"
      @touchend="handleTouchEnd"
    >
      <div
        class="relative will-change-transform"
        :style="{
          transform: `translate3d(${position.x}px, ${position.y}px, 0) scale3d(${scale}, ${scale}, 1)`,
          transformOrigin: transformOrigin,
          transition: isTransitioning ? 'transform 0.1s' : 'none',
          backfaceVisibility: 'hidden',
          perspective: '1000px'
        }"
      >
        <img
          :src="mapImageUrl"
          alt="Hong Kong MTR Map"
          class="w-full h-auto block high-res-image"
          @load="initializeBounds"
          ref="mapImage"
          draggable="false"
        />
        <!-- Overlay container -->
        <div class="absolute inset-0 w-full h-full pointer-events-none">
          <slot name="overlays"></slot>
        </div>
      </div>
    </div>
  </div>
</template>
n
<script>
export default {
  name: 'Map',
  props: {
    mapImageUrl: {
      type: String,
      required: true
    },
    minZoom: {
      type: Number,
      default: 1
    },
    maxZoom: {
      type: Number,
      default: 4
    }
  },
  data() {
    return {
      scale: 1.2,
      position: { x: 0, y: 0 },
      lastPosition: { x: 0, y: 0 },
      lastDistance: 0,
      isTransitioning: false,
      touchStarted: false,
      transformOrigin: 'center',
      bounds: {
        width: 0,
        height: 0
      }
    }
  },

  methods: {
    initializeBounds() {
      const container = this.$refs.mapWrapper
      const image = this.$refs.mapImage
      this.bounds = {
        width: container.clientWidth,
        height: container.clientHeight,
        imageWidth: image.clientWidth,
        imageHeight: image.clientHeight
      }
    },

    handleTouchStart(event) {
      if (event.touches.length === 2) {
        this.touchStarted = true
        this.lastDistance = this.getDistance(event.touches[0], event.touches[1])

        // Calculate center point between fingers for zoom origin
        const centerX = (event.touches[0].clientX + event.touches[1].clientX) / 2
        const centerY = (event.touches[0].clientY + event.touches[1].clientY) / 2
        const rect = this.$refs.mapWrapper.getBoundingClientRect()
        this.transformOrigin = `${centerX - rect.left}px ${centerY - rect.top}px`
      } else if (event.touches.length === 1) {
        this.touchStarted = true
        this.lastPosition = {
          x: event.touches[0].clientX - this.position.x,
          y: event.touches[0].clientY - this.position.y
        }
      }
    },

    handleTouchMove(event) {
      event.preventDefault()
      if (!this.touchStarted) return

      if (event.touches.length === 2) {
        // Handle pinch zoom
        const currentDistance = this.getDistance(event.touches[0], event.touches[1])
        const delta = currentDistance - this.lastDistance
        this.lastDistance = currentDistance

        const newScale = this.scale * (1 + delta * 0.01)
        this.scale = Math.max(this.minZoom, Math.min(this.maxZoom, newScale))
      } else if (event.touches.length === 1) {
        // Handle pan with bounds
        const newX = event.touches[0].clientX - this.lastPosition.x
        const newY = event.touches[0].clientY - this.lastPosition.y

        // Calculate bounds
        const maxX = (this.bounds.imageWidth * this.scale - this.bounds.width) / 2
        const maxY = (this.bounds.imageHeight * this.scale - this.bounds.height) / 2

        // Apply bounded position
        this.position = {
          x: Math.max(-maxX, Math.min(maxX, newX)),
          y: Math.max(-maxY, Math.min(maxY, newY))
        }
      }
    },

    handleTouchEnd() {
      this.touchStarted = false
      this.isTransitioning = true
      setTimeout(() => {
        this.isTransitioning = false
      }, 100)

      // Ensure position is within bounds after interaction
      this.enforceBounds()
    },

    enforceBounds() {
      const maxX = (this.bounds.imageWidth * this.scale - this.bounds.width) / 2
      const maxY = (this.bounds.imageHeight * this.scale - this.bounds.height) / 2

      this.position = {
        x: Math.max(-maxX, Math.min(maxX, this.position.x)),
        y: Math.max(-maxY, Math.min(maxY, this.position.y))
      }
    },

    getDistance(touch1, touch2) {
      const dx = touch1.clientX - touch2.clientX
      const dy = touch1.clientY - touch2.clientY
      return Math.sqrt(dx * dx + dy * dy)
    }
  },

  mounted() {
    window.addEventListener('resize', this.initializeBounds)
  },

  beforeDestroy() {
    window.removeEventListener('resize', this.initializeBounds)
  }
}
</script>
