import { user } from '@/composables/auth'
import type { User, UserQuiz } from '@/composables/user'
import { defineStore } from 'pinia'


export const useUsers = defineStore('users', {
    state: () => ({
        users: {} as User,
        userQuiz: {} as UserQuiz,
        loading: false,
        error: null,

    }),
    actions: {
        async getUser() {
            const id = localStorage.getItem('id')
            const token = localStorage.getItem('token')
            this.loading = true
            this.error = null
            try {
                // Replace with your API call
                const response = await fetch(`/api/users/${id}`, {
                    method: 'GET',
                    headers: {
                        'Content-Type': 'application/json',
                        'Authorization': `Bearer ${token}`,
                    },
                }
                )
                if (!response.ok) throw new Error('Failed to fetch users')
                const data = await response.json()
                this.users = data
            } catch (err: any) {
                this.error = err.message || 'Unknown error'
            } finally {
                this.loading = false
            }
        },
        async GetLastQuiz() {
            const id = localStorage.getItem('id')
            const token = localStorage.getItem('token')
            this.loading = true
            this.error = null
            try {
                // Replace with your API call
                const response = await fetch(`/api/users/${id}/quiz`, {
                    method: 'GET',
                    headers: {
                        'Content-Type': 'application/json',
                        'Authorization': `Bearer ${token}`,
                    },
                })
                if (!response.ok) throw new Error('Failed to fetch last quiz')
                const data = await response.json()
                this.userQuiz = data
            } catch (err: any) {
                this.error = err.message || 'Unknown error'
            } finally {
                this.loading = false
            }
        }


    },
})