import type {  Questionnaire, ResponseQuiz } from '@/composables/questionnaire';
import { defineStore } from 'pinia';

export const useQuiz = defineStore('useQuiz', {
    state: () => ({
        collection: {} as ResponseQuiz,
    }),
    actions: {
        async getResponseQuiz(quiz: Questionnaire[]): Promise<ResponseQuiz> {
            try {
                const token = localStorage.getItem('token');
                const id = localStorage.getItem('id');
                console.log('Quiz data:', quiz);
                const response = await fetch(`http://localhost:3000/api/survey/result/${id}`, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        'Authorization': `Bearer ${token}`,
                    },
                    body: JSON.stringify(quiz),
                });
                const data: ResponseQuiz = await response.json();
                this.collection = data;
                console.log('Jobs fetched:', this.collection);
                return data;
            } catch (error) {
                console.error('Error fetching jobs:', error);
                return {} as ResponseQuiz;
            }
        }
    }
})