<template>
    <form @submit.prevent="onSubmit" class="space-y-4 w-full">
        <div class="flex flex-row space-x-2 w-full">
            <Input v-model="teamName" placeholder="Team Name" class="basis-3/4" />
            <Button type="submit" class="basis-1/4">Create Team</Button>
        </div>
    </form>
</template>

<script setup>
import { ref } from 'vue'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { invoke } from '@tauri-apps/api/core'
import { message } from '@tauri-apps/plugin-dialog';

const teamName = ref('')

const onSubmit = async () => {
    const values = {
        team: teamName.value,
    }
    try {
        await invoke('new', values);
        // Assuming you have a store, you might want to update it here
        // store.team = teamName.value
    } catch (error) {
        console.error(error);
        await message('Unable to create new team', {title: 'City runners', kind: 'error'})
    }
}
</script>
