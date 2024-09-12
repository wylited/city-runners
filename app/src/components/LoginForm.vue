<script setup lang="ts">
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

const formSchema = toTypedSchema(z.object({
  address: z.string().url(),
  username: z.string().min(2).max(50),
  password: z.string().min(8).max(100),
}))


const { handleSubmit } = useForm({
  validationSchema: formSchema,
})

const onSubmit = handleSubmit((values) => {
     invoke('login', values)
     console.log('Form submitted!', values)
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
          <Input type="text" class="text-gray text-xs" placeholder="hello123" v-bind="componentField" />
        </FormControl>
        <FormMessage />
      </FormItem>
    </FormField>

    <Button type="submit" class="text-xl font-medium">
      Submit
    </Button>
  </form>
</template>
