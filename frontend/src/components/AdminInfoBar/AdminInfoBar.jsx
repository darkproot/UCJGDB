import { useEffect, useState } from 'react'
import styles from '../MemberInfoBar/MemberInfoBar.module.css'
import axios from 'axios'

const API = 'http://192.168.1.104:5000'

export default function AdminInfoBar({ back, delId }) {

    const [id, setId] = useState(delId)
    const[admin_id, setAdminId] = useState(JSON.parse(window.sessionStorage.getItem('admin_data')).id)

    useEffect(_ => {
        setAdminId(JSON.parse(window.sessionStorage.getItem('admin_data')).id)
        setId(delId)
    }, [])

    const retour = () => {
        back() 
    }  

    const deleteMember = _ => {
        if (id) {
            const data ={
                admin_id: admin_id, 
                id: id
            }
            axios.post(API + '/del_admin', data)
                .then(res => {
                    if (res.data.err) alert(res.data.err)
                    else {
                        back()
                    }
                })
                .catch(err => console.log(err))
        } else alert("Il manque l'id " + id)
    }
    
    return(
        <div className={styles.container}>
            <button onClick={retour} className={styles.back}>{'<' + ' Retour'}</button>
            <div className={styles.gap}></div>

            <div className={styles.action}>
                <button className={styles.del} onClick={deleteMember}>Supprimer</button>
            </div>
        </div>
    )
}
