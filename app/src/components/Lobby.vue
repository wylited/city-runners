<script setup lang="ts">
 import { ref, computed, onMounted } from 'vue'
 import { store, Team } from '../store'
 import { Button } from '@/components/ui/button'
 import { Card, CardHeader, CardContent, CardFooter } from '@/components/ui/card'
 import { Badge } from '@/components/ui/badge'
 import {
   Tooltip,
   TooltipContent,
   TooltipProvider,
   TooltipTrigger,
 } from "@/components/ui/tooltip"

 import { message } from '@tauri-apps/plugin-dialog';
 import { invoke } from '@tauri-apps/api/core';

 const isLoading = ref(false)
 const teams = ref<Team[]>([
   { id: '1', name: 'Alpha Example', members: ['Alice', 'Bob', 'Charlie'], ttype: 'Participant', ready: true },
   { id: '2', name: 'Beta Brigade', members: ['David', 'Eve', 'Frank'], ttype: 'Spectator', ready: false },
   { id: '3', name: 'Gamma Group', members: ['Grace', 'Heidi', 'Ivan'], ttype: 'Participant', ready: true },
   { id: '4', name: 'Delta Division', members: ['Judy', 'Kevin', 'Liam'], ttype: 'Participant', ready: false },
   { id: '5', name: 'Epsilon Ensemble', members: ['Mallory', 'Nina', 'Oscar'], ttype: 'Spectator', ready: true }
 ])

 const fetchTeams = async () => {
   isLoading.value = true;
   try {
     const response = await invoke<Team[]>('get');
     teams.value = response;
   } catch (error) {
     console.error(error);
     await message('Failed to fetch teams', { title: 'City Runners', kind: 'error' });
   } finally {
     isLoading.value = false;
   }
 };

 const joinTeam = async (teamId: string) => {
   isLoading.value = true
   try {
     const response = await invoke('join', teamId);
     store.team = teamId
   } catch (error) {
     console.error(error);
     await message('Unable to join', {title: 'City Runners', kind: 'error'})
   } finally {
     isLoading.value = false
   }
 }

 const leaveTeam = () => {
   store.team = null
 }

 const isTeamReady = computed(() => {
   const team = teams.value.find(team => team.id === store.team)
   return team ? team.ready : false
 })

 onMounted(async () => {
   isLoading.value = true;
   try {
     const response = await invoke<Team[]>('get');
     teams.value = response;
   } catch (error) {
     console.error(error);
     await message('Failed to fetch teams', { title: 'City Runners', kind: 'error' });
   } finally {
     isLoading.value = false;
   }
 });
</script>

<template>
  <div class="flex flex-col py-2 px-2">
    <Card class="min-h-[95vh]">
      <CardHeader class="flex-row justify-between items-center px-4 py-2 border-b">
        <Button variant="ghost" class="text-lg font-semibold italic w-min">Lobby</Button>
        <div class="flex items-center space-x-2">
          <Badge variant="secondary" class="text-xs">{{ teams[0].ttype }}</Badge>
          <p class="text-sm italic underline">{{ store.username }}</p>
        </div>
      </CardHeader>
      <CardContent class="min-h-[80vh]">
        <div class="flex-1 max-h-[78vh] overflow-y-auto overflow-scroll">
          <Card
            v-for="team in teams"
            :key="team.id"
            class="my-6"
          >
            <CardHeader class="flex flex-row items-center justify-between pb-2">
              <h3 class="text-lg font-semibold">{{ team.name }}</h3>
              <Badge
                :style="{ backgroundColor: team.ready ? 'green' : 'gray' }"
                class="text-xs"
              >
                {{ team.ready ? 'Ready' : 'Unready' }}
              </Badge>
            </CardHeader>
            <CardContent>
              <div class="flex flex-wrap gap-2">
                <Badge v-for="player in team.players" :key="player" variant="secondary" class="text-sm">
                  {{ player }}
                </Badge>
              </div>
            </CardContent>
            <CardFooter>
              <template v-if="isLoading">
                <Button
                  disabled
                  :variant="store.team === team.id ? 'destructive' : 'default'"
                  class="w-full">
                  {{ store.team === team.id ? 'Leave' : 'Join' }}
                </Button>
              </template>
              <template v-else>
                <Button
                  @click="store.team === team.id ? leaveTeam() : joinTeam(team.id)"
                  :variant="store.team === team.id ? 'destructive' : 'default'"
                  class="w-full"
                >
                  {{ store.team === team.id ? 'Leave' : 'Join' }}
                </Button>
              </template>
            </CardFooter>
          </Card>
        </div>
      </CardContent>
      <CardFooter class="border-t flex flex-col items-center justify-center p-4">
        <template v-if="store.team === null">
          <Button class="w-full text-2xl">
            New Team
          </Button>
        </template>
        <template v-else>
        <Button
          :style="{ backgroundColor: isTeamReady ? 'green' : 'lightcoral' }"
          class="w-full text-2xl"
        >
          {{isTeamReady ? 'Unready' : 'Ready'}}
        </Button>
        </template>
        <Button v-if="store.admin" class="w-full text-2xl mt-2">Start</Button>
      </CardFooter>
    </Card>
  </div>
</template>
