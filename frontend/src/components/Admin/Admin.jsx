import { useEffect, useState } from "react"
import styles from '../Member/Member.module.css'
import axios from "axios"
import API from "../../API"

// const API = 'http://192.168.1.104:5000'

function Admin({ back, readOnly, AdminId }) {

    const [name, setName] = useState('')
    const handleName = (event) => setName(event.target.value)
    const [password, setPassword] = useState('')
    const handlePassword = (event) => setPassword(event.target.value) 
    const [id, setId] = useState(AdminId)

    const data_verification = () => {
        if (!name) {
            alert('⚠️ Il manque un PSEUDO')
            return false
        } else if (!password) {
            alert('⚠️ Il manque un Mot de passe')
            return false
        }
        
        return true
    }

    useEffect(() => {
        setId(AdminId)
        if (readOnly && id) {
            const admin_id = JSON.parse(window.sessionStorage.getItem('admin_data')).id
            axios.get(API + `/get_admin/${admin_id}/${AdminId}`)
                .then(res => {
                    if (res.data.err) alert(res.data.err)
                    else {
                        setName(res.data.name)
                        setPassword(res.data.password)
                    }
                })
                .catch(err => console.log(err))
        }
    }, [])


    const add_admin = () => {
        if (data_verification()) {
            const admin_id = JSON.parse(window.sessionStorage.getItem('admin_data')).id
            const data = {
                admin_id: admin_id,
                name: name,
                password: password
            }
            axios.post(API + '/sign', data)
                .then(res => {
                    if (res.data.err) alert(res.data.err)
                    else { back() }
                })
                .catch(err => console.log(err))
        }
    }   

    return(
        <>
            <div className={styles.container}>
                <div className={styles.block}>
                    <p>Pseudo:</p>
                    <input type="text" value={name} onChange={handleName} readOnly={readOnly} />
                </div>
                <div className={styles.block}>
                    <p>Mot de passe:</p>
                    <input type="text" value={password} onChange={handlePassword} readOnly={readOnly} />
                </div>

            </div>

            {readOnly || 
                <div className={styles.validation}>
                    <button onClick={add_admin}>Enregistrer</button>
                </div>
            }
        </>
    )
}

export default Admin