export interface MessageItem {
    role: string;
    content?: string;
    reasoning_content?:string;
    text: string;
}

export interface APISetting {
    base_url: string;
    key: string;
    model: string;
}