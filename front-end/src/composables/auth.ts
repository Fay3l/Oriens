import { ref } from 'vue';
import { type UUID } from 'vue-uuid';
export const token = ref<string>('');
export const isAuthenticated = ref<boolean>(false);
export const user = ref<UUID>();

