import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { appWindow, WebviewWindow } from "@tauri-apps/api/window";

function App() {
  try {
  let [text, setText] = useState("HEY");
  let [assetTable, setTable] = useState([]);

  async function refreshAssetList()
  {
    setText("LOADING");
    let arr = await invoke("assetList");
    setText(arr[0].name);
    setTable(arr);
    await invoke("error", {message : table.state[0].name});
  }

  async function createEditWindow(t)
  {
    const webview = new WebviewWindow('popup', {
      url: "/src/Popup.html"
    });

    webview.once("window_loaded", (e) =>
    {
      webview.emit("update_values", {
        id: t.id,
        name: t.name,
        model: t.model,
        modelid: t.modelid,
        manu: t.manu,
        typeid: t.typeid,
        glentype: t.glentype,
        ip: t.ip,
      })
    })

  }

  return (
    <div>
    <button onClick={refreshAssetList}></button>
    <p>{text}</p>
    <table>
      {assetTable.map((t) =>
      (
        <tr>
          <td>{t.name}</td>
          <td>{t.model}</td>
          <td>{t.manu}</td>
          <td>{t.glentype}</td>
          <td>{t.ip}</td>
          <td><button onClick={() => createEditWindow(t)}>EDIT</button></td>
        </tr>
      ))}
    </table>
    </div>
  );
  } catch (err)
  {
    invoke("error", { message : err.stack});
  }
}

export default App;
