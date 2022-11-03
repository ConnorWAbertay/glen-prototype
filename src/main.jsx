import React from "react";
import { useState } from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import Insert from "./Insert";
import "./style.css";

ReactDOM.createRoot(document.getElementById("root")).render(
    <Tab />
);

function Tab()
{
  let [currentTab, setTab] = useState((<App />));

  return (<div>
        <div className = "sidebar">
      <button onClick={() => {setTab(<App />)}}>MAIN</button>
      <button onClick={() => {setTab(<Insert />)}}>INSERT</button>
    </div>
    <div className = "main">
      {currentTab}
    </div>
  </div>)
}