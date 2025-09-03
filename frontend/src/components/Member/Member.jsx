import { useEffect, useState } from 'react'
import styles from './Member.module.css'
import axios from 'axios'
import API from '../../API'

// const API = 'http://192.168.1.104:5000'

// function newDate(date) {
//     let [jour, mois, annee] = date.split('-')
//     return `${annee}-${mois}-${jour}`
// }   

export default function Member({ memberId, back }) {

    const [name, setName] = useState('')
    const handleName = (event) => setName(event.target.value)
    const [surname, setSurname] = useState('')
    const handleSurname = (event) => setSurname(event.target.value)
    const [sexe, setSexe] = useState('')
    const handleSexe = (event) => setSexe(event.target.value)
    const [birth_date, setBirthDate] = useState('')
    const handleBirthDate = (event) => setBirthDate(event.target.value)
    const [birth_place, setBirthPlace] = useState('')
    const handleBirthPlace = (event) => setBirthPlace(event.target.value)
    const [classe, setClasse] = useState('')
    const handleClasse = (event) => setClasse(event.target.value)
    const [numero, setNumero] = useState('')
    const handleNumero = (event) => setNumero(event.target.value)
    const [parent_name, setParentName] = useState('')
    const handleParentName = (event) => setParentName(event.target.value)
    const [parent_surname, setParentSurname] = useState('')
    const handleParentSurname = (event) => setParentSurname(event.target.value)
    const [parent_numero, setParentNumero] = useState('')
    const handleParentNumero = (event) => setParentNumero(event.target.value)
    const [email, setEmail] = useState('')
    const handleEmail = (event) => setEmail(event.target.value)

    const [social_media, setSocialMedia] = useState()

    const [facebook, setFacebook] = useState('/')
    const handleFacebook = (event) => setFacebook(event.target.value)
    const [x, setX] = useState('/')
    const handleX = (event) => setX(event.target.value)
    const [tiktok, setTiktok] = useState('/')
    const handleTiktok = (event) => setTiktok(event.target.value)
    const [insta, setInsta] = useState('/')
    const handleInsta = (event) => setInsta(event.target.value)
    const [threads, setThreads] = useState('/')
    const handleThreads = (event) => setThreads(event.target.value)
    const [snap, setSnap] = useState('/')
    const handleSnap = (event) => setSnap(event.target.value)

    const [admin_id, setAdminId] = useState(JSON.parse(window.sessionStorage.getItem('admin_data')).id)
    const [member_id, setMemberId] = useState(memberId)

    const social_update = (social_media) => {
        const cache = social_media
        const result = []
        for (let i = 0; i < cache.length; i++) {
            const element = cache[i];
            if (element.id) 
                result.push([element.name, element.id])
        }
        setSocialMedia(result)
    }

    const fetchData = () => {
        const data = {
            admin_id: admin_id,
            id: member_id
        }
        axios.post(API + '/member', data)
            .then(res => {
                if (res.data.err) alert(res.data.err)
                else {
                    const member = res.data[0]
                    const social = member.social
                    setSocialMedia(social)
                    social_update(social)

                    setName(member.name.toUpperCase() || '/')
                    setSurname(member.surname.toUpperCase() || '/')
                    setSexe(member.sexe.toUpperCase() || '/')
                    setBirthDate(member.birth_date || '/')
                    setBirthPlace(member.birth_place || '/')
                    setClasse(member.classe || '/')
                    setNumero(member.numero || '/')
                    setParentName(member.parent_name.toUpperCase() || '/')
                    setParentSurname(member.parent_surname.toUpperCase() || '/')
                    setParentNumero(member.parent_numero || '/')
                    setEmail(member.email || '/')

                    for (let i = 0; i < social.length; i++) {
                        const element = social[i]
                        switch (element.name) {
                            case 'facebook': setFacebook(element.pseudo || '/'); break;
                            case 'snapchat': setSnap(element.pseudo || '/'); break;
                            case 'x': setX(element.pseudo || '/'); break;
                            case 'instagram': setInsta(element.pseudo || '/'); break;
                            case 'threads': setThreads(element.pseudo || '/'); break;
                            case 'tiktok': setTiktok(element.pseudo || '/'); break;
                        }
                    }
                }
            })
            .catch(err => console.log(err))
    }
    const data_verification = () => {
        const data = [
            [name, 'Il manque un NOM'],
            [surname, 'Il manque un PRENOM'],
            [sexe, 'Il manque un SEXE'],
            [birth_date, 'Il manque une DATE DE NAISSANCE'],
            [birth_place, 'Il manque un LIEU DE NAISSANCE'],
            [classe, 'Il manque une OCCUPATION'],
            [numero, 'Il manque un NUMERO DE TELEPHONE'],
            [parent_name, 'Il manque un NOM DE PARENT'],
            [parent_surname, 'Il manque un PRENOM DE PARENT'],
            [parent_numero, 'Il manque un NUMERO DE PARENT'],
            [email, 'Il manque un E-MAIL'],
        ]

        if (!facebook) setFacebook('/')
        if (!snap) setSnap('/')
        if (!x) setX('/')
        if (!tiktok) setTiktok('/')
        if (!insta) setInsta('/')
        if (!threads) setThreads('/')

        for (let i = 0; i < data.length; i++) {
            const element = data[i];
            if (!element[0]) {
                alert('⚠️' + element[1])
                return false
            }
        }
        return true
    }

    const handleUpdate = () => {
        if (data_verification()) {
            const social = [
                {name: "facebook", pseudo: facebook},
                {name: "x", pseudo: x},
                {name: "instagram", pseudo: insta},
                {name: "threads", pseudo: threads},
                {name: "snapchat", pseudo: snap},
                {name: "tiktok", pseudo: tiktok}                    
            ]
            for (let i = 0; i < social_media.length; i++) {
                const sm = social_media[i];
                for (let j = 0; j < social.length; j++) {
                    const s = social[j];
                    if (sm[0] == s.name) s.id = sm[1]
                }
            }

            const data = {
                id: memberId,
                name: name,
                surname: surname,
                sexe: sexe,
                birth_date: birth_date,
                birth_place: birth_place,
                classe: classe,
                numero: numero,
                parent_name: parent_name,
                parent_surname: parent_surname,
                parent_numero: parent_numero,
                email: email,
                admin_id: admin_id,
                social: social
            }

            axios.put(API + '/update_member', data)
                .then(res => {
                    if (res.data.err) alert(res.data.err)
                    else {
                        console.log(res.data)
                        back()
                    }
                })
                .catch(err => console.log(err))
        }
    }

    useEffect(_ => { 
        setAdminId(JSON.parse(window.sessionStorage.getItem('admin_data')).id)
        setMemberId(memberId)
        fetchData() 
    }, [])
    

    return (
        <>
            <div className={styles.container}>
                <div className={styles.block}>
                    <p>Noms:</p>
                    <input type="text" value={name} onChange={handleName} />
                </div>
                <div className={styles.block}>
                    <p>Prenoms:</p>
                    <input type="text" value={surname} onChange={handleSurname} />
                </div>
                <div className={styles.block}>
                    <p>Sexe:</p>
                    <input type="text" value={sexe} onChange={handleSexe} />
                </div>
                <div className={styles.block}>
                    <p>Date de naissance:</p>
                    <input type="date" value={birth_date} onChange={handleBirthDate} />
                </div>
                <div className={styles.block}>
                    <p>Lieu de naissance:</p>
                    <input type="text" value={birth_place} onChange={handleBirthPlace} />
                </div>
                <div className={styles.block}>
                    <p>Occupation:</p>
                    <input type="text" value={classe} onChange={handleClasse} />
                </div>
                <div className={styles.block}>
                    <p>Telephones:</p>
                    <input type="tel" value={numero} onChange={handleNumero} />
                </div>
                <div className={styles.block}>
                    <p>Noms du parent:</p>
                    <input type="text" value={parent_name} onChange={handleParentName} />
                </div>
                <div className={styles.block}>
                    <p>Prenoms du parent:</p>
                    <input type="text" value={parent_surname} onChange={handleParentSurname} />
                </div>
                <div className={styles.block}>
                    <p>Telephones du parent:</p>
                    <input type="tel" value={parent_numero} onChange={handleParentNumero} />
                </div>
                <div className={styles.block}>
                    <p>E-mail:</p>
                    <input type="email" value={email} onChange={handleEmail} />
                </div>
            </div>

            <div className={styles.container}>
                <div className={styles.block}>
                    <p>Facebook:</p>
                    <input type="text" value={facebook} onChange={handleFacebook} />
                </div>
                <div className={styles.block}>
                    <p>X (Twitter):</p>
                    <input type="text" value={x} onChange={handleX} />
                </div>
                <div className={styles.block}>
                    <p>Tiktok:</p>
                    <input type="text" value={tiktok} onChange={handleTiktok} />
                </div>
                <div className={styles.block}>
                    <p>Instagram:</p>
                    <input type="text" value={insta} onChange={handleInsta} />
                </div>
                <div className={styles.block}>
                    <p>Threads:</p>
                    <input type="text" value={threads} onChange={handleThreads} />
                </div>
                <div className={styles.block}>
                    <p>Snapchat:</p>
                    <input type="text" value={snap} onChange={handleSnap} />
                </div>
            </div>

            <div className={styles.validation}>
                <button onClick={handleUpdate}>Enregistrer</button>
            </div>
        </>
    )
}