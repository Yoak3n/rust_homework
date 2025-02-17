import BotIcon from "./BotIcon"
import type { MessageItem } from "../types"
import { useEffect, useState,useRef, MutableRefObject } from "react";

const ChatMessage = ({ message }: { message: MessageItem }) => {
    const [displayText,setDisplayText] = useState('')
    let index = 0
    const [enableRunning, setEnableRunning] = useState(false);
    const timerRef:MutableRefObject<number|null> = useRef(null); 
    const savedCallback = useRef(() => {}); 
    useEffect(() => {
       savedCallback.current = () => {
            // 检测定时器是否在运行，防止内存泄漏
            // console.log('running')
           if (index < message.content!.length){
                index ++
                setDisplayText(message.content!.slice(0,index))
            }else{
                setEnableRunning(false)
            }
       }
    },[])
    useEffect(() => {
        if(message.role !== 'assistant'){
            setDisplayText(message.content!)
        }else{
            if (enableRunning){
                timerRef.current = setInterval(() => {
                    savedCallback.current();
                },100)
            }else{
                clearInterval(timerRef.current!)
            }
        }
        return () => clearInterval(timerRef.current!);
    },[enableRunning])
    useEffect(() => {
        if(message.role === 'assistant'){
            if(!enableRunning){setEnableRunning(true)}

            // TypeWriter(message.content!)
        }else{
            setDisplayText(message.content!)
        }
    },[message.content])


    return (
        <><div className={`message ${message.role === 'assistant' ? 'bot' : message.role === 'system-error' ? 'error' : 'user'}-message`}>
            {message.role !=='user' && <BotIcon/>}
            <p className="message-text">{displayText}</p>
        </div>
            
        </>

    )
}

export default ChatMessage