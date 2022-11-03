import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { createElement } from "react";
import { createRoot } from "react-dom/client";

var id;

const updateValues = async (payload) => {
    let modelArr = await invoke("get_name", {
        query:
            "SELECT `id`, `name` FROM `glenmodel` WHERE 1"
    });
    console.log(modelArr);
    let typeArr = await invoke("get_name", {
        query:
            "SELECT `id`, `name` FROM `glentype` WHERE 1"
    });
    console.log(typeArr);

    let modelElement = createElement("select", {name:"select_model", id:"select_model"}, (<>
        {
            modelArr.map(
                (e) =>
                (
                    <option value={e[0]} selected={e[0] == payload.modelid}>{e[1]}</option>
                )
            )
        }
    </>));

    let typeElement = createElement("select", {name:"select_type", id:"select_type"}, (<>
        {
            typeArr.map(
                (e) =>
                (
                    <option value={e[0]} selected={e[0] === payload.typeid}>{e[1]}</option>
                )
            )
        }
    </>));

    let root = createRoot(document.getElementById("edit"));
    
    root.render((
        <form name="edit_form" id="edit_form" onSubmit={(e) => {
            e.preventDefault()
            let form = document.forms["edit_form"];
            invoke("update", {
                info: {
                    id: payload.id,
                    name: form.elements["name"].value,
                    modelid: parseInt(form.elements["select_model"].value),
                    typeid: parseInt(form.elements["select_type"].value),
                    ip: form.elements["ip"].value
                }
            });
        }}>
            <input type="text" name="name" defaultValue={payload.name}></input>
            <input type="text" name="ip" defaultValue={payload.ip}></input>
            {typeElement}
            {modelElement}
            <input type="submit" value="Update"></input> 
        </form>
    ));
}

listen("update_values", (e) => {
    updateValues(JSON.parse(e.payload));
});
invoke("window_loaded");