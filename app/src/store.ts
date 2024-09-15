// store.ts
import { reactive, watch } from 'vue';
import Login from './components/Login.vue';
import { Store } from '@tauri-apps/plugin-store';

export interface Team {
  name: string;
  players: string[];
  ttype: string;
  ready: bool;
}

const tauriStore = new Store('store.bin');

export const store = reactive({
  admin: true as boolean,
  address: "https://city-runners.shuttleapp.rs" as string,
  page: Login,
  username: null as string | null,
  token: null as string | null,
  team: null as string | null,
});

// Function to load data from tauri-store
async function loadStore() {
  store.username = (await tauriStore.get('username')) as string | null;
  store.token = (await tauriStore.get('token')) as string | null;
  store.admin = (await tauriStore.get('admin')) as boolean;
  store.address = (await tauriStore.get('address')) as string;
  store.team = (await tauriStore.get('team')) as string | null;
}

// Function to save data to tauri-store
async function saveStore() {
  console.log(store.address);
  await tauriStore.set('username', store.username);
  await tauriStore.set('token', store.token);
  await tauriStore.set('admin', store.admin);
  await tauriStore.set('address', store.address);
  await tauriStore.set('team', store.team);
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