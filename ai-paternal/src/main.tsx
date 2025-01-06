import ReactDOM from "react-dom/client";
import Router from './router'
import { BrowserRouter } from "react-router";


ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <BrowserRouter>
    <Router />
  </BrowserRouter>
);
