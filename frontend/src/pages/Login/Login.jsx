import styles from "./Login.module.css"
import { useState } from "react"
import axios from 'axios'
import API from "../../API"


function Login(props) {

    const [name, setName] = useState('')
    const [password, setPassword] = useState('')
    let data = { name: name, password: password }
    // const API = 'http://192.168.1.104:5000'

    const handleName = _ => { setName(document.getElementById('name').value) }
    const handlePassword = _ => { setPassword(document.getElementById('password').value) }
    const sendRequest = async () => {
        if (data.name && data.password) {
            const res = await axios.post(API + '/login', data)
            const response = res.data
            console.log(response)

            if (response.id) {
                window.sessionStorage.setItem('admin_data', JSON.stringify(res.data))
                props.changePage()
            } else if (response.err) alert(response.err)
        } else { alert("Formulaire incomplet") }
    }


    return(
        <div className={styles.main}>
            <div className={styles.container}>
                <p>Admin Login</p>
                <input type="text" id="name" onChange={handleName} placeholder="Identifiant" value={name} />
                <input type="password" id="password" onChange={handlePassword} placeholder="Mot de passe" value={password} />
                <input type="button" onClick={sendRequest} value="Connexion" id="conn" />
            </div>
        </div>
    )
}

export default Login; 