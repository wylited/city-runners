<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Card, CardContent } from '@/components/ui/card'
import { store } from '../store.ts'

const scrollRef = ref<HTMLDivElement | null>(null)

const scrollToBottom = async () => {
  await nextTick()
  if (scrollRef.value) {
    scrollRef.value.scrollTop = scrollRef.value.scrollHeight
  }
}

onMounted(() => {
     scrollToBottom()
     console.log(store.msgs)
})

// Watch for new messages and scroll to bottom
 // watch(() => store.msgs, () => {
 //   scrollToBottom()
 // }, { deep: true })
</script>

<template>
  <Card class="h-[600px] w-full">
    <ScrollArea
      ref="scrollRef"
      class="h-full p-4 space-y-4"
    >
      <div v-for="(msg, index) in store.msgs" :key="index">
        <CardContent
          class="rounded-lg p-4 space-y-2"
        >
          <div class="flex items-start gap-2">
          </div>
          <div class="text-sm">
            {{ msg }}
          </div>
        </CardContent>
      </div>
    </ScrollArea>
  </Card>
</template>

<style scoped>
.scroll-area {
  scrollbar-width: thin;
  scrollbar-color: hsl(var(--primary)) transparent;
}

.scroll-area::-webkit-scrollbar {
  width: 6px;
}

.scroll-area::-webkit-scrollbar-track {
  background: transparent;
}

.scroll-area::-webkit-scrollbar-thumb {
  background-color: hsl(var(--primary));
  border-radius: 3px;
}
</style>
