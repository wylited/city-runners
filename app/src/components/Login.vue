<script setup lang="ts">
 import { Button } from '@/components/ui/button'
 import { useTemplateRef, onMounted, ref } from 'vue'
 import { listen } from '@tauri-apps/api/event';

 import {
  Drawer,
  DrawerClose,
  DrawerContent,
  DrawerDescription,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger,
} from '@/components/ui/drawer'

 import LoginForm from '@/components/LoginForm.vue'

 const drawerOpen = ref(false);
 onMounted(() => {
   listen('closeDrawer', () => {
     console.log('Trying to close new DRAWER');
     drawerOpen.value = false;
   })
 })
</script>

<template>
  <div class="min-h-[95vh] flex items-center justify-center">
    <div class="w-full text-center pt-8">
      <h1 class="text-7xl text-left italic">
        CITY
      </h1>
      <h1 class="text-7xl text-right italic pb-8">
        RUNNERS
      </h1>

      <Drawer v-model:open="drawerOpen">
        <DrawerTrigger aschild class="bg-black">
          <Button size="lg" class="text-2xl font-medium active:bg-gray">
            CONNECT
          </Button>
        </DrawerTrigger>
        <DrawerContent>
          <DrawerHeader>
            <LoginForm />
          </DrawerHeader>
        </DrawerContent>
      </Drawer>
    </div>
  </div>
</template>
