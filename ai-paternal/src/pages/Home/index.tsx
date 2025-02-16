import {useState,useRef,useEffect}from "react";
import {invoke} from '@tauri-apps/api/core'
import { register,ShortcutEvent } from '@tauri-apps/plugin-global-shortcut';
import BotIcon from "../../components/BotIcon";
import ChatForm from "../../components/ChatForm";
import type{MessageItem} from '../../types/index'
import './index.css'
import ChatMessage from "../../components/ChatMessage";
import { querySetting } from "../../api/db";
export default function Home() {
    const [chatHistory,setChatHistory] = useState<Array<MessageItem>>([])
    const [isChatOpen,setIsChatOpen] = useState(true)
    const bodyRef =  useRef<HTMLInputElement>(null);
    const updateHistory = (message:MessageItem) => {
        setChatHistory((prev:Array<MessageItem>)=>[...prev.filter((item) => item.content !== 'Thinking...'),message])
    }
    let tempOpenState = isChatOpen;
    const scrollToBottom = () => {
        if(bodyRef.current){
            bodyRef.current.scrollTo({top:bodyRef.current.scrollHeight,behavior:'smooth'})
        }
    }
    useEffect(() => {
        register('CommandOrControl+Q', async(event: ShortcutEvent) => {
          if (event.state === 'Pressed') {
            tempOpenState = !tempOpenState
            invoke('resize_window',{hide: tempOpenState, shortcut: false})       
          }
        }); 
    },[])
    useEffect(() => {
        scrollToBottom()
    },[chatHistory])
    useEffect(() => {
        setIsChatOpen(tempOpenState)
    },[tempOpenState])
    const resetConversation = ()=>setChatHistory([])
    const generateBotResponse =async (history:Array<MessageItem>) => {
        const {base_url,key,model} = await querySetting()
        if(!base_url || !key || !model || base_url === "" || key === "" || model === "") {
            updateHistory({role:"system-error",content:"请先配置API密钥和模型",text:"请先配置API密钥和模型"})
            return
        }
        const requestOptions = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization':`Bearer ${key}`,
            },
            body: JSON.stringify({
                model,
                messages: history
            })
        }
        try{
            let api_target = ""
            if (base_url.endsWith('v1')){
                api_target = `${base_url}/chat/completions`
            }else{
                api_target = base_url
            }
            const res = await fetch(api_target, requestOptions)
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
                        <button className="logo-text" onClick={resetConversation}>Chatbot</button>
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
                        {/* <button onClick={createSettingWindow}>test</button> */}
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