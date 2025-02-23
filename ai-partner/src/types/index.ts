export interface MessageItem {
    role: string;
    content?: string;
    reasoning_content?:string;
    text: string;
    timestamp?: number;
}
export interface AppSetting  extends APISetting {
    smoothing: boolean;
}
export interface APISetting {
    base_url: string;
    key: string;
    model: string;
}