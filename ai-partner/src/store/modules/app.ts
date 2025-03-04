import {defineStore} from 'pinia'


export const useAppStore = defineStore('app', {
    state: () => ({
        generating: false,
    }),
    actions: {
        setGenerating(generating: boolean) {
            this.generating = generating
        }
    }
})