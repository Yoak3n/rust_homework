import BotIcon from "./BotIcon"
import type { MessageItem } from "../types"

const ChatMessage = ({ message }: { message: MessageItem }) => {
    
    return (
        <><div className={`message ${message.role === 'assistant' ? 'bot' : message.role === 'system-error' ? 'error' : 'user'}-message`}>
            {message.role !=='user' && <BotIcon/>}
            <p className="message-text">{message.content}</p>
        </div>
            
        </>

    )
}

export default ChatMessage