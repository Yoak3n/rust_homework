export interface MessageItem {
    role: string;
    content?: string;
    reasoning_content?:string;
    timestamp?: number;
}

export interface ErrorMessage {
    id:number;
    message: string;
}

export interface AppSetting  {
    smooth: boolean;
    api: API;
}
export interface API {
    url: string;
    key: string;
    model: string;
}

export interface Segment {
    raw:string;
    index?: number;
    hash: number;
    html?:string
}
export * from './listener'