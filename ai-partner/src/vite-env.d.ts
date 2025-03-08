/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

declare module 'markdown-it' {
  const MarkdownIt: any;
  export default MarkdownIt;
}

declare module 'markdown-it-highlightjs' {
  const highlightjs: any;
  export default highlightjs;
}
import type { MessageApi } from 'naive-ui'
declare global {
  interface Window {
    $loadingBar: LoadingBarInst
    $dialog: DialogApiInjection
    $message: MessageApi
    $notification: NotificationApiInjection
  }
}

