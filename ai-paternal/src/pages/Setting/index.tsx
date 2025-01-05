import { useState,useEffect } from 'react'
import './index.css'
import type { APISetting } from '../../types'
import {getApiSetting} from '../../utils/index'
export default function Settings() {
  const [apiSetting, setApiSetting] = useState<APISetting>()
  useEffect(()=>{
    getApiSetting().then(res=>{
      setApiSetting(res)
    })
  })
  return(
    <div className="settings">
      <form action="#" className="settings-form">
        <div className="settings-form-api">
          <h2 className='item-title'>API设置</h2>
          <div className="form-item api-base_url">
            <span className="form-label">API</span>
            <input type="text" value={apiSetting?.base_url} />
          </div>
          <div className="form-item api-key">
            <span className="form-label">key</span>
            <input type="password" value={apiSetting?.key} />
          </div>
          <div className="form-item model-name">
            <span className="form-label">model</span>
            <input type="text" value={apiSetting?.model}/>
          </div>
        </div>
        <div className="settings-form-button">
          <button type="submit">保存</button>
        </div>
      </form>
    </div>
  )
}