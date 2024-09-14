<script setup lang="ts">
import { ref, computed } from 'vue'
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

const joinTeam = (teamId: string) => {
  store.currentTeam = teamId
}

const leaveTeam = () => {
  store.currentTeam = null
}

const isCurrentTeamReady = computed(() => {
  const currentTeam = teams.value.find(team => team.id === store.currentTeam)
  return currentTeam ? currentTeam.ready : false
})

const teams = ref<Team[]>([
  { id: '1', name: 'Alpha Squad', members: ['Alice', 'Bob', 'Charlie'], type: 'Participant', ready: true },
  { id: '2', name: 'Beta Brigade', members: ['David', 'Eve', 'Frank'], type: 'Spectator', ready: false },
  { id: '3', name: 'Gamma Group', members: ['Grace', 'Heidi', 'Ivan'], type: 'Participant', ready: true },
  { id: '4', name: 'Delta Division', members: ['Judy', 'Kevin', 'Liam'], type: 'Participant', ready: false },
  { id: '5', name: 'Epsilon Ensemble', members: ['Mallory', 'Nina', 'Oscar'], type: 'Spectator', ready: true }
])
</script>

<template>
  <div class="flex flex-col py-2 px-2">
    <Card class="min-h-[95vh]">
      <CardHeader class="flex-row justify-between items-center px-4 py-2 border-b">
        <Button variant="ghost" class="text-lg font-semibold italic w-min">Lobby</Button>
        <div class="flex items-center space-x-2">
          <Badge variant="secondary" class="text-xs">{{ teams[0].type }}</Badge>
          <p class="text-sm italic underline">{{ store.username }}</p>
        </div>
      </CardHeader>
      <CardContent>
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
                <Badge v-for="member in team.members" :key="member" variant="secondary" class="text-sm">
                  {{ member }}
                </Badge>
              </div>
            </CardContent>
            <CardFooter>
              <Button
                @click="store.currentTeam === team.id ? leaveTeam() : joinTeam(team.id)"
                :variant="store.currentTeam === team.id ? 'destructive' : 'default'"
                class="w-full"
              >
                {{ store.currentTeam === team.id ? 'Leave' : 'Join' }}
              </Button>
            </CardFooter>
          </Card>
        </div>
      </CardContent>
      <CardFooter>
        <Separator/>
        <Button
          :style="{ backgroundColor: isCurrentTeamReady ? 'green' : 'lightcoral' }"
          class="w-full text-2xl"
        >
          {{isCurrentTeamReady ? 'Unready' : 'Ready'}}
        </Button>
        <Button v-if="store.admin" class="w-full text-2xl mt-2">Start</Button>
      </CardFooter>
    </Card>
  </div>
</template>
