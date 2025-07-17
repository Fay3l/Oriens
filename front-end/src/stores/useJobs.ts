import { Jobs } from '@/composables/jobs';
import { defineStore } from 'pinia';

export const useJobs = defineStore('useJobs', {

     
    state: () => ({
        collection: [] as Jobs[],
    }),
    
    actions: {
        async getJobs(page: number, perPage: number): Promise<Jobs[]> {
            try {
                const response = await fetch(`http://localhost:3000/api/jobs?page=${page}&per_page=${perPage}`);
                const data: Jobs[] = await response.json();
                this.collection = data;
                console.log('Jobs fetched:', this.collection);
                return data;
            } catch (error) {
                console.error('Error fetching jobs:', error);
                return [];
            }
        },
        
    }
})