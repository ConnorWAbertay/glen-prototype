import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function AssetView() {
  try {
  let [text, setText] = useState("HEY");
  const [assetTable, setTable] = useState([])

  async function refreshAssetList()
  {
    setText("LOADING");
    let arr = await invoke("assetList");
    setText(arr[0].name);
    setTable(arr);
    await invoke("error", {message : table.state[0].name});
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
