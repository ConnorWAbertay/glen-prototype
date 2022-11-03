import { invoke } from "@tauri-apps/api/tauri";
import { useState, useEffect } from "react";
import "./App.css";

function Insert() {
    const [assetForm, setAssetForm] = useState();
    const [manuSelect, setManuSelect] = useState();
    useEffect(() => {
        createAssetForm();
        getManuList();
      }, [])

    async function createAssetForm() {
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

        setAssetForm(
            <form name="insert_form" id="insert_form" onSubmit={(e) => {
                e.preventDefault()
                let form = document.forms["insert_form"];
                invoke("insert_asset", {
                    info: {
                        id: 0,
                        name: form.elements["name"].value,
                        modelid: parseInt(form.elements["select_model"].value),
                        typeid: parseInt(form.elements["select_type"].value),
                        ip: form.elements["ip"].value
                    }
                });
            }}>
                <input type="text" name="name" placeholder="name"></input>
                <input type="text" name="ip" placeholder="ip"></input>
                <select id="select_type" name="select_type">
                    {
                        typeArr.map(
                            (e) =>
                            (
                                <option value={e[0]}>{e[1]}</option>
                            )
                        )
                    }
                </select>
                <select id="select_model" name="select_model">
                   { modelArr.map(
                            (e) =>
                            (
                                <option value={e[0]}>{e[1]}</option>
                            )
                        )}
                </select>
                <input type="submit" value="INSERT"></input>
            </form>);
    }

    async function getManuList() {
        let arr = await invoke("get_name", { query: "SELECT `glenmanu`.id, `glenmanu`.`name` FROM `glenmanu` WHERE 1" });
        setManuSelect((
            <select name="select_manu" id="select_manu">
                {
                    arr.map(
                        (e) =>
                        (
                            <option value={toString(e[0])}>{e[1]}</option>
                        )
                    )
                }
            </select>
        ))
    }

    return (
        <div>
            <h5>Insert Manu</h5>
            <input type="text" id="input_manu" placeholder="manufacturer"></input>
            <button id="insert_manu" onClick={() => {
                invoke("insert_manu",
                    {
                        manu: document.getElementById("input_manu").value
                    })
            }
            }>INSERT</button>
            <br></br>
            <h5>Insert Type</h5>
            <input type="text" id="input_type" placeholder="type"></input>
            <button id="insert_type" onClick={() => {
                invoke("insert_type",
                    {
                        glentype: document.getElementById("input_type").value
                    })
            }
            }>INSERT</button>
            <br></br>
            <h5>Insert Model</h5>
            <input type="text" id="input_model" placeholder="model"></input>
            <button>AAA</button>
            {manuSelect}
            <button id="insert_model" onClick={() => {
                invoke("insert_model",
                    {
                        model: document.getElementById("input_model").value,
                        manuid: parseInt(document.getElementById("select_manu").value)
                    })
            }
            }>INSERT</button>
            <h5>Insert Asset</h5>
            {assetForm}
        </div>
    )
}

export default Insert;