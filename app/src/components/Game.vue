<template>
  <div class="w-screen min-h-screen p-4 flex flex-col gap-4">
    <!-- Map Container -->
    <div class="w-[90%] md:w-[40%] aspect-square mx-auto">
      <div class="w-full h-full rounded-2xl bg-gray-800 p-2 shadow-lg">
        <Map
          :mapImageUrl="'/map.svg'"
          :minZoom="1"
          :maxZoom="4"
          class="w-full h-full rounded-xl overflow-hidden"
        >
          <template #overlays>
            <div class="absolute top-[50%] left-[50%] w-4 h-4 bg-red-500 rounded-full -translate-x-1/2 -translate-y-1/2"></div>
          </template>
        </Map>
      </div>
    </div>
   <Chat />
  </div>
</template>

<script>
import Map from './Map.vue'
import Chat from './Chat.vue'

export default {
  components: {
    Map,
    Chat
  }
}
</script>
