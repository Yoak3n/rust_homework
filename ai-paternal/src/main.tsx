import ReactDOM from "react-dom/client";
import Router from './router'
import { BrowserRouter } from "react-router";

import { register,ShortcutEvent,unregister } from '@tauri-apps/plugin-global-shortcut';
import { invoke } from "@tauri-apps/api/core";
unregister('CommandOrControl+Q');
register('CommandOrControl+Q', (event: ShortcutEvent) => {
  if (event.state === 'Pressed') {
    invoke('resize_window',{hide: true, shortcut: true});
  }
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <BrowserRouter>
    <Router />
  </BrowserRouter>
);
