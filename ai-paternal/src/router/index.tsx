import {Routes, Route} from 'react-router';

import Home from '../pages/Home'
import Settings from '../pages/Setting';
import App from '../App';
export default function()
{
    return (
      <Routes>
      <Route path="/" element={<App/>}>
        <Route index element={<Home />} />
        <Route path="setting" element={<Settings />} />
      </Route>
    </Routes>
    )
}