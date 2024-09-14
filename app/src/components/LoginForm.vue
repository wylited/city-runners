<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { h } from 'vue'
import { useForm } from 'vee-validate'
import { toTypedSchema } from '@vee-validate/zod'
import * as z from 'zod'

import { Button } from '@/components/ui/button'
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form'

import { Input } from '@/components/ui/input'
import { Dialog, DialogTrigger, DialogContent, DialogTitle, DialogDescription } from '@/components/ui/dialog'

import { store } from '../store'
import Lobby from './Lobby.vue'

const formSchema = toTypedSchema(z.object({
  address: z.string().url(),
  username: z.string().min(2).max(50),
  password: z.string().min(2).max(20),
}))

const { handleSubmit } = useForm({
  validationSchema: formSchema,
  initialValues: {
    address: 'https://city-runners.shuttleapp.rs',
  }
})

const showDialog = ref(false)
const dialogTitle = ref('Loading')
const dialogMessage = ref('')
const isLoading = ref(false)

const onSubmit = handleSubmit(async (values) => {
  showDialog.value = true
  isLoading.value = true
  dialogTitle.value = 'CONNECTING'

  try {
    const token = await invoke('login', values)
    console.log('Token received:', token)
    store.token = token
    store.username = values.username
    store.page = Lobby;
    dialogTitle.value = 'Success'
    dialogMessage.value = 'Login successful!'
  } catch (error) {
    console.error(error)
    dialogTitle.value = 'Error'
    dialogMessage.value = 'Incorrect login details'
  } finally {
    isLoading.value = false
  }
})
</script>

<template>
  <form class="space-y-2 text-left" @submit="onSubmit">
    <FormField v-slot="{ componentField }" name="address">
      <FormItem>
        <FormLabel class="text-lg font-bold">Server Address</FormLabel>
        <FormControl>
          <Input type="text" class="text-gray text-xs" placeholder="city-runners.shuttleapp.rs" v-bind="componentField" />
        </FormControl>
        <FormMessage />
      </FormItem>
    </FormField>

    <FormField v-slot="{ componentField }" name="username">
      <FormItem>
        <FormLabel class="text-lg font-bold">Username</FormLabel>
        <FormControl>
          <Input type="text" class="text-gray text-xs" placeholder="nano" v-bind="componentField" />
        </FormControl>
        <FormMessage />
      </FormItem>
    </FormField>

    <FormField v-slot="{ componentField }" name="password">
      <FormItem>
        <FormLabel class="text-lg font-bold">Password</FormLabel>
        <FormControl>
          <Input type="password" class="text-gray text-xs" placeholder="hello123" v-bind="componentField" />
        </FormControl>
        <FormMessage />
      </FormItem>
    </FormField>

    <Button type="submit" class="text-xl font-medium">
      Submit
    </Button>
  </form>

  <Dialog v-model:open="showDialog">
    <DialogContent class="rounded-lg p-6 items-center justify-center w-11/12 sm:max-w-md italic">
      <DialogTitle class="text-3xl font-bold mb-4">{{ dialogTitle }}</DialogTitle>
      <DialogDescription class="text-lg mb-6">
        <div v-if="isLoading" class="flex items-center justify-center">
          <div class="animate-spin rounded-full h-12 w-12 border-b-4 border-gray-900"></div>
        </div>
      </DialogDescription>
    </DialogContent>
  </Dialog>
</template>

<style scoped>
.animate-spin {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>
