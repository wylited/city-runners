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
 import { ask } from '@tauri-apps/plugin-dialog';

 import TeamForm from './TeamForm.vue'

 const isLoading = ref(false)
 const teams = ref<Team[]>([
   { id: '1', name: 'Alpha Example', players: ['Alice', 'Bob', 'Charlie'], ttype: 'Participant', ready: true },
   { id: '2', name: 'Beta Brigade', players: ['David', 'Eve', 'Frank'], ttype: 'Spectator', ready: false },
   { id: '3', name: 'Gamma Group', players: ['Grace', 'Heidi', 'Ivan'], ttype: 'Participant', ready: true },
   { id: '4', name: 'Delta Division', players: ['Judy', 'Kevin', 'Liam'], ttype: 'Participant', ready: false },
   { id: '5', name: 'Epsilon Ensemble', players: ['Mallory', 'Nina', 'Oscar'], ttype: 'Spectator', ready: true }
 ])

 const fetchTeams = async () => {
   try {
     const response = await invoke<Team[]>('get'); // Fetch teams from backend
     teams.value = response; // Update teams

     // Check if the current user is in any of the fetched teams
     const currentUserTeam = response.find(team =>
       team.players.includes(store.username || '')
     );

     // Update the store.team value based on whether the user is in a team
     if (currentUserTeam) {
       store.team = currentUserTeam.name;
     } else {
       store.team = null;
     }
   } catch (error) {
     console.error(error);
     await message('Failed to fetch teams', { title: 'City Runners', kind: 'error' });
   }
 };

 // Function to repeatedly fetch teams every 2 seconds
 const startFetchingTeams = () => {
   fetchTeams(); // Initial fetch
   setInterval(fetchTeams, 3000); // Fetch every 2 seconds
 };

 // Call the function to start fetching
 startFetchingTeams();

 const joinTeam = async (teamId: string) => {
   isLoading.value = true
   const values = {
     team: teamId,
   }
   try {
     await invoke('join', values);
     store.team = teamId
   } catch (error) {
     console.error(error);
     await message('Unable to join', {title: 'City Runners', kind: 'error'})
   } finally {
     isLoading.value = false
   }
 }

 const leaveTeam = async (teamId: string) => {
   const values = {
     team: teamId
   }
   console.log(values);
   isLoading.value = true
   try {
     await invoke('leave', values);
     store.team = null
   } catch (error) {
     console.error(error);
     await message('Unable to leave', {title: 'City Runners', kind: 'error'})
   } finally{
     isLoading.value = false
   }
 }

 const startGame = async () => {
     isLoading.value = true;
     try {
         await invoke('start');
     } catch (error) {
         console.error(error);
         await message('Unable to start the game', { title: 'City Runners', kind: 'error' });
     } finally {
         isLoading.value = false;
     }

     isLoading.value = true;
     try {
         await invoke('connect');
         await message('Connected to the game');
     } catch (error) {
         console.error(error);
         await message('Unable to connect to the game', { title: 'City Runners', kind: 'error' });
     } finally {
         isLoading.value = false;
     }

 };

 const toggleReadyState = async () => {
   // Create a confirmation dialog
   const answer = await ask('Are you sure?', {
     title: 'Confirmation',
     kind: 'warning',
   });

   // If the user confirms, proceed with toggling the ready state
   if (answer) {
     isLoading.value = true;
     const values = {
       team: store.team
     }
     try {
       await invoke('ready', values);
     } catch (error) {
       console.error(error);
       await message('Unable to change ready state', { title: 'City Runners', kind: 'error' });
     } finally {
       isLoading.value = false;
     }
   } else {
     console.log('User canceled the action');
   }
 };

 const isTeamReady = computed(() => {
   const team = teams.value.find(team => team.name === store.team)
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
        <Button variant="ghost" class="text-2xl font-semibold italic w-min">Lobby</Button>
        <div class="flex items-center space-x-2">
            <Badge variant="secondary" className="text-xs">
                {{store.admin ? 'admin' : 'player'}}
            </Badge>
          <p class="text-sm italic underline">{{ store.username }}</p>
        </div>
      </CardHeader>
      <CardContent class="min-h-[80vh]">
        <div class="flex-1 max-h-[78vh] overflow-y-auto overflow-scroll">
          <Card
            v-for="team in teams"
            :key="team.name"
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
                  :variant="store.team === team.name ? 'destructive' : 'default'"
                  class="w-full">
                  {{ store.team === team.name ? 'Leave' : 'Join' }}
                </Button>
              </template>
              <template v-else>
                <Button
                  @click="store.team === team.name ? leaveTeam(team.name) : joinTeam(team.name)"
                  :variant="store.team === team.name ? 'destructive' : 'default'"
                  class="w-full"
                >
                  {{ store.team === team.name ? 'Leave' : 'Join' }}
                </Button>
              </template>
            </CardFooter>
          </Card>
          <template v-if="store.admin">
              <Button
                  @click="startGame"
                  class="w-full text-2xl"
                  :style="{ backgroundColor: 'gray' }"
              >
                  Start
              </Button>
          </template>
        </div>
      </CardContent>
      <CardFooter class="border-t flex flex-col items-center justify-center p-4">
        <TeamForm v-if="store.team === null" />
        <template v-else>
          <template v-if="isLoading">
            <Button disabled
                    :style="{ backgroundColor: isTeamReady ? 'green' : 'lightcoral' }"
                    class="w-full text-2xl"
                    @click="toggleReadyState"
            >
              {{isTeamReady ? 'Unready' : 'Ready'}}
            </Button>

          </template>
          <template v-else>
            <Button
              :style="{ backgroundColor: isTeamReady ? 'green' : 'lightcoral' }"
              class="w-full text-2xl"
              @click="toggleReadyState"
            >
              {{isTeamReady ? 'Unready' : 'Ready'}}
            </Button>
          </template>
        </template>
      </CardFooter>
    </Card>
  </div>
</template>
