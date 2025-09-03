import { useEffect, useState } from 'react'
import styles from './MemberInfoBar.module.css'
import axios from 'axios'

const API = 'http://192.168.1.104:5000'

export default function MemberInfoBar({ back, memberId }) {

    const [member_id, setId] = useState(memberId)
    const[admin_id, setAdminId] = useState(JSON.parse(window.sessionStorage.getItem('admin_data')).id)

    useEffect(_ => {
        setAdminId(JSON.parse(window.sessionStorage.getItem('admin_data')).id)
        setId(memberId)
    }, [])

    const deleteMember = _ => {
        if (member_id) {
            const data ={
                admin_id: admin_id, 
                id: member_id
            }
            axios.post(API + '/del_member', data)
                .then(res => {
                    if (res.data.err) alert(res.data.err)
                    else back()
                })
                .catch(err => console.log(err))
        } else if (memberId) {
            const data ={
                admin_id: admin_id, 
                id: memberId
            }
            axios.post(API + '/del_member', data)
                .then(res => {
                    if (res.data.err) alert(res.data.err)
                    else back()
                })
                .catch(err => console.log(err))
        } else alert("Il manque l'id " + memberId)
    }
    
    return(
        <div className={styles.container}>
            <button onClick={back} className={styles.back}>{'<' + ' Retour'}</button>
            <div className={styles.gap}></div>

            <div className={styles.action}>
                <button className={styles.del} onClick={deleteMember}>Supprimer</button>
                <button className={styles.download}>Telecharger</button>
            </div>
        </div>
    )
}
