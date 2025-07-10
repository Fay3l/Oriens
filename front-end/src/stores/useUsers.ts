import type { User } from '@/composables/user'
import { defineStore } from 'pinia'


export const useUsers = defineStore('users', {
    state: () => ({
        users: [] as User[],
        loading: false,
        error: null,

    }),
    actions: {
        async getUser() {
            const id = localStorage.getItem('id')
            this.loading = true
            this.error = null
            try {
                // Replace with your API call
                const response = await fetch(`/api/users/${id}`)
                if (!response.ok) throw new Error('Failed to fetch users')
                const data = await response.json()
                this.users = data
            } catch (err: any) {
                this.error = err.message || 'Unknown error'
            } finally {
                this.loading = false
            }
        },
        
        
    },
})