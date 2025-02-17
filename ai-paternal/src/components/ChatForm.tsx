import { useRef,Dispatch } from "react";
import type { MessageItem } from "../types";
const ChatForm = (
    {
        chatHistory,
        setChatHistory,
        generateBotResponse
    }:
    {
        chatHistory:MessageItem[],
        setChatHistory:Dispatch<React.SetStateAction<MessageItem[]>>,
        generateBotResponse: (message:MessageItem[]) => void
    }) => {
    const inputRef =  useRef<HTMLInputElement>(null);
    const handleFormSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const userMessage = inputRef.current?.value.trim();
        if(!userMessage) return;
        inputRef.current!.value = "";
        const newMessage:MessageItem = {
            role: "user",
            text:userMessage,
            content: userMessage,
        } 
        setChatHistory((prev:Array<MessageItem>) => [...prev, newMessage]);
        setTimeout(()=>{
            generateBotResponse([...chatHistory, newMessage]);
            // 组件内部没有更新history？
        },600)

    }
    return (
        <form action="#" className="chat-form" onSubmit={handleFormSubmit}>
            <input ref={inputRef} className="message-input" type="text" placeholder="Type a message" required />
            <button className="material-symbols-rounded">send</button>
        </form>
    )
}



export default ChatForm