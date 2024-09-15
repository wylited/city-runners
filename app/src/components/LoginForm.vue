<script setup lang="ts">
 import { ref } from 'vue'
 import { invoke } from '@tauri-apps/api/core'
 import { message } from '@tauri-apps/plugin-dialog';
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
 import { Loader2 } from 'lucide-vue-next'

 import { store } from '../store'
 import Lobby from './Lobby.vue'
 import Spinner from './Spinner.vue'

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

 const isLoading = ref(false)

 const onSubmit = handleSubmit(async (values) => {
   isLoading.value = true
   try {
     const token = await invoke('login', values)
     console.log('Token received:', token)
     store.token = token
     store.username = values.username
     store.page = Lobby;
   } catch (error) {
     console.error(error)
     await message('Incorrect details', {title: 'City Runners', kind: 'error'})
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

    <Button type="submit" class="text-xl font-medium" :variant="isLoading ? 'outline' : 'default'">
      <template v-if="isLoading">
        <Loader2 class="mr-2 animate-spin" />
        Loading...
      </template>
      <template v-else>
        Submit
      </template>
    </Button>
  </form>
</template>
