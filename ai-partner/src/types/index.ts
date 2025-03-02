export interface MessageItem {
    role: string;
    content?: string;
    reasoning_content?:string;
    text: string;
    timestamp?: number;
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