import { WebviewWindow } from "@tauri-apps/api/webviewWindow";


export const createSettingWindow= async() => {
    let oldWindow = await WebviewWindow.getByLabel("setting");
    if (oldWindow) {

        oldWindow.show();
        oldWindow.setAlwaysOnTop(true);
        return;
    }
    const webviewWindow = new WebviewWindow("setting", {
        url: "/setting",
        title: "设置",
        width: 400,
        height: 300,
        center: true,
        alwaysOnTop: true,
    });
    webviewWindow.once("tauri://created", () => {
        console.log("设置窗口已创建");
    });
    webviewWindow.once("tauri://error", (e) => {
        console.log(e);
                
    })
}