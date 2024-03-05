export interface Bookmark {
    id? :number
    name :string
    url:string,
    content:string
    created_at?:string
    category_id?:number
}

export interface Category{
    id? :number
    name:string
    tab:string
    created_at?:string
}