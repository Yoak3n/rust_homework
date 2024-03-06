import {defineStore} from 'pinia';
import { create_category, read_categories } from '../../api/db';
import type{ Category } from '../../api/type';

export const useCategoryStore = defineStore(
    'category',
    {
        state: () => {
            return {
                categories: Array<Category>() as Array<Category>
            }
        },
        actions:{
            async read_categories(){
                this.categories = await read_categories()
                localStorage.setItem('categories',JSON.stringify(this.categories))
            },
            async create_category(record:Category){
                create_category(record)
            }
        }
    }
)