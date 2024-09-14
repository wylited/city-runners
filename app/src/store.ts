z// store.ts
import { reactive, watch } from 'vue';
import Login from './components/Login.vue';
import { Store } from '@tauri-apps/plugin-store';

export interface Team {
  id: string;
  name: string;
  members: string[];
  type: string;
}

const tauriStore = new Store('store.bin');

export const store = reactive({
  page: Login,
  username: null as string | null,
  token: null as string | null,
  admin: false,
  currentTeam: null as string | null,
  teams: [] as Team[],
});

// Function to load data from tauri-store
async function loadStore() {
  store.username = (await tauriStore.get('username')) as string | null;
  store.token = (await tauriStore.get('token')) as string | null;
  store.admin = (await tauriStore.get('admin')) as boolean;
  store.currentTeam = (await tauriStore.get('currentTeam')) as string | null;
  store.teams = (await tauriStore.get('teams')) as Team[] || [];
}

// Function to save data to tauri-store
async function saveStore() {
  await tauriStore.set('username', store.username);
  await tauriStore.set('token', store.token);
  await tauriStore.set('admin', store.admin);
  await tauriStore.set('currentTeam', store.currentTeam);
  await tauriStore.set('teams', store.teams);
  await tauriStore.save();
}

// Watch for changes in the reactive store and save to tauri-store
watch(
  () => ({ ...store }),
  async () => {
    await saveStore();
  },
  { deep: true }
);

// Load the store data initially
loadStore();
