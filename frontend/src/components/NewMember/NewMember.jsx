import { useEffect, useState } from 'react'
import styles from './NewMember.module.css'
import axios from 'axios'
import API from '../../API'

// const API = 'http://192.168.1.104:5000'

function data_verification() {
    const name = document.getElementById('name').value.trim()
    const surname = document.getElementById('surname').value.trim()
    const birth_date = document.getElementById('birth_date').value.trim()
    const birth_place = document.getElementById('birth_place').value.trim()
    const classe = document.getElementById('classe').value.trim()
    const numero = document.getElementById('numero').value.trim()
    const email = document.getElementById('email').value.trim()
    const parent_name = document.getElementById('parent_name').value.trim()
    const parent_surname = document.getElementById('parent_surname').value.trim()
    const parent_numero = document.getElementById('parent_numero').value.trim()
    
    const data = [
        [name, "Il vous manque au moins un NOM"],
        [surname, "Il vous manque au moins un PRENOM"],
        [birth_date, "Il manque la DATE DE NAISSANCE"],
        [birth_place, "Il manque le LIEU DE NAISSANCE"],
        [classe, "Il manque une OCCUPATION"],
        [numero, "Il manque un NUMERO"],
        [email, "Il manque un EMAIL"],
        [parent_name, "Il manque un NOM DE PARENT"],
        [parent_surname, "Il manque un PRENOM DE PARENT"],
        [parent_numero, "Il manque un NUMERO DE PARENT"],
    ]
    
    for (const element of data) {
        if (!element[0]) {
            alert('⚠️ ' + element[1])
            return false
        }
    }
    return true
}

export default function NewMember({ handleClosing, go_to_member }) {

    const [open, setOpen] = useState(false)
    const [radio, setRadio] = useState('M')
    const [layer, setLayer] = useState({'--layer': '20'})
    const changeLayer = (value) => setLayer({'--layer': `${value}`})
    const [admin_id, setAdminId] = useState(JSON.parse(window.sessionStorage.getItem('admin_data')).id)
    
    useEffect(_ => {
        setAdminId(JSON.parse(window.sessionStorage.getItem('admin_data')).id)
        setOpen(true)
        changeLayer(20)
    }, [])

    const handleRadioChange = event => setRadio(event.target.value)

    const handleAddMember = _ => {

        const name = document.getElementById('name').value.trim()
        const surname = document.getElementById('surname').value.trim()
        const birth_date = document.getElementById('birth_date').value.trim()
        const birth_place = document.getElementById('birth_place').value.trim()
        const classe = document.getElementById('classe').value.trim()
        const numero = document.getElementById('numero').value.trim()
        const email = document.getElementById('email').value.trim()
        const parent_name = document.getElementById('parent_name').value.trim()
        const parent_surname = document.getElementById('parent_surname').value.trim()
        const parent_numero = document.getElementById('parent_numero').value.trim()
        if (data_verification()) {
            const data = {
                admin_id: admin_id,
                birth_date: birth_date,
                birth_place: birth_place,
                classe: classe,
                name: name,
                parent_name: parent_name,
                numero: numero,
                parent_numero: parent_numero,
                sexe: radio,
                surname: surname,
                parent_surname: parent_surname,
                email: email
            }
            
            axios.post(API + '/new_member', data)
                .then(res => {
                    if (res.data.err) alert(res.data.err)
                    else {
                        handleClosing()
                        // setOpen(false)
                        changeLayer(-20)
                        go_to_member(res.data.id)
                    }
                }
            )
            .catch(err => console.log(err))
        }
    }

    return(
        <dialog open={open} className={styles.main} style={layer}>
            <div className={styles.container}>
                <h2>Information Nouveau Jeune</h2>

                <div className={styles.form}>
                    <div className={styles.block}>
                        <label>Noms:</label>
                        <input type="text" name="name" id="name" />
                    </div>
                    <div className={styles.block}>
                        <label>Prenoms:</label>
                        <input type="text" name="surname" id="surname" />
                    </div>
                    <div className={styles.block}>
                        <label>Sexe:</label>
                        <div className={styles.sexe}>
                            <label htmlFor='sexe-M'>M</label><input type="radio" name="sexe-M" id="sexe-M" value={'M'} checked={radio === 'M'} onChange={handleRadioChange} />
                            <label htmlFor='sexe-F'>F</label><input type="radio" name="sexe-F" id="sexe-F" value={'F'} checked={radio === 'F'} onChange={handleRadioChange} />
                        </div>
                    </div>
                    <div className={styles.block}>
                        <label>Date de naissance:</label>
                        <input type="date" name="birth_date" id="birth_date" />
                    </div>
                    <div className={styles.block}>
                        <label>Lieu de naissance:</label>
                        <input type="text" name="birth_place" id="birth_place" />
                    </div>
                    <div className={styles.block}>
                        <label>Occupation:</label>
                        <input type="text" name="classe" id="classe" />
                    </div>
                    <div className={styles.block}>
                        <label>Telephone:</label>
                        <input type="tel" name="numero" id="numero" />
                    </div>
                    <div className={styles.block}>
                        <label>E-mail:</label>
                        <input type="email" name="email" id="email" />
                    </div>

                    <div className={styles.block}>
                        <label>Noms parent:</label>
                        <input type="text" name="parent_name" id="parent_name" />
                    </div>
                    <div className={styles.block}>
                        <label>Prenoms parent:</label>
                        <input type="text" name="parent_surname" id="parent_surname" />
                    </div>
                    <div className={styles.block}>
                        <label>Telephone parent:</label>
                        <input type="tel" name="parent_numero" id="parent_numero" />
                    </div>
                </div>

                <div className={styles.action}>
                    <button className={styles.action_del} onClick={handleClosing}>Annuler</button>
                    <button className={styles.action_add} onClick={handleAddMember}>Suivant</button>
                </div>
            </div>
        </dialog>
    )
}