import { useState,useEffect } from 'react'
import './index.css'
import type { APISetting } from '../../types'
import {getApiSetting,closeSettingWindow} from '../../utils/index'
import {invoke} from '@tauri-apps/api/core'
export default function Settings() {
  const [apiSetting, setApiSetting] = useState<APISetting>({
    base_url: '',
    key: '',
    model: ''
  })
  useEffect(()=>{
    getApiSetting().then(res=>{
      setApiSetting(res)
    })
  },[])
  // useEffect第二个参数留空就会不停调用？NB！
  const saveApiSetting=()=>{
    const api:APISetting = {
      base_url:apiSetting.base_url,
      key:apiSetting.key,
      model:apiSetting.model
    } 
    console.log(api);
    
    invoke('modify_api',{baseUrl:apiSetting.base_url,key:apiSetting.key,model:apiSetting.model})
  }
  return(
    <div className="settings">
      <form action="#" className="settings-form" >
        <div className="settings-form-api">
          <h2 className='item-title'>API设置</h2>
          <div className="form-item api-base_url">
            <span className="form-label">API</span>
            <input type="text" value={apiSetting?.base_url} onChange={e=>{
              setApiSetting(prev=>{
                return {...prev,base_url:e.target.value}
              })
            }}/>
          </div>
          <div className="form-item api-key">
            <span className="form-label">key</span>
            <input type="password" value={apiSetting?.key} onChange={e=>{
              setApiSetting(prev=>{
                return {...prev,key:e.target.value}
              })
            }}/>
          </div>
          <div className="form-item model-name">
            <span className="form-label">model</span>
            <input type="text" value={apiSetting?.model} onChange={e=>{
              setApiSetting(prev=>{
                return {...prev,model:e.target.value}
              })
            }}/>
          </div>
        </div>
        <div className="settings-form-button">
          <button type="submit" onClick={saveApiSetting}>保存</button>
        </div>
      </form>
    </div>
  )
}