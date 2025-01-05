import {useState,useRef,useEffect}from "react";
import {invoke} from '@tauri-apps/api/core'
import BotIcon from "../../components/BotIcon";
import ChatForm from "../../components/ChatForm";
import type{MessageItem} from '../../types/index'
import './index.css'
import ChatMessage from "../../components/ChatMessage";
import {createSettingWindow} from '../../utils/index'
export default function Home() {
    const [chatHistory,setChatHistory] = useState<Array<MessageItem>>([])
    const [isChatOpen,setIsChatOpen] = useState(true)
    const bodyRef =  useRef<HTMLInputElement>(null);
    const updateHistory = (message:MessageItem) => {
        setChatHistory([...chatHistory.filter((item) => item.content !== 'Thinking...'),message])
    }

    const scrollToBottom = () => {
        if(bodyRef.current){
            bodyRef.current.scrollTo({top:bodyRef.current.scrollHeight,behavior:'smooth'})
        }
    }
    useEffect(() => {
        scrollToBottom()
    },[chatHistory])
    const generateBotResponse =async (history:Array<MessageItem>) => {
        const {base_url,key} = await getApiSetting()
        const requestOptions = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization':`Bearer ${key}`,
            },
            body: JSON.stringify({
                model: "deepseek-chat",
                messages: history
            })
        }
        try{
            const res = await fetch(base_url, requestOptions)
            const data = await res.json()
            if(!res.ok) throw new Error(res.statusText || "Something went wrong")
            const botAnswer:MessageItem = data.choices[0].message
            updateHistory(botAnswer)
        }
        catch(err:any){
            const errMessage:MessageItem  = {role:"system-error",content:err.message,text:err.message}
            updateHistory(errMessage) 
        }
    }


    const getApiSetting= async()=>{
        const r:Array<string> = await invoke('invoke_api') 
        if(r.length>0){
            return {base_url:r[0],key:r[1]}
        }
        return {base_url:'',key:''}
    }
    return (
        <div className={`home ${isChatOpen ? 'show-chatbot' : ''}`}>
            <button id="chatbot-toggler" onClick={async() => {
                    invoke('resize_window',{hide: isChatOpen, shortcut: false})
                    setIsChatOpen(!isChatOpen)
                }  
            }>
                <span className="material-symbols-rounded">mode_comment</span>
                <span className="material-symbols-rounded">close</span>
            </button>
            <div className="chatbot-popup">
                <div className="chatbot">
                    <div className="chatbot-header" data-tauri-drag-region>
                        <div className="header-info">
                        <BotIcon />
                        <h2 className="logo-text">Chatbot</h2>
                        </div>
                        <button className="material-symbols-rounded" onClick={() => setIsChatOpen(false)}>
                            keyboard_arrow_down
                        </button>
                    </div>
                    <div className="chatbot-body" ref={bodyRef}>
                        <div className="message bot-message">
                            <BotIcon />
                            <p className="message-text">How can I help you?</p>
                        </div>
                        {
                            chatHistory &&  //判断数组是否为空
                            chatHistory.map((item,index)=>{
                                return <ChatMessage key={index} message={item}/>
                            })
                        }

                    </div>
                    <div className="chatbot-footer">
                        <button onClick={createSettingWindow}>test</button>
                        <ChatForm 
                        chatHistory={chatHistory}
                        setChatHistory={setChatHistory} 
                        generateBotResponse={generateBotResponse}/>
                    </div>
                </div>
            </div>
        </div>
    );
}