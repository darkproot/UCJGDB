import styles from './Table.module.css'

const titles = [
    "id", "nom", "prenom", "sexe", "Date de naissance",
    "lieu de naissance", "classe", "Numero", "Nom parent",
    "prenom parent", "numero parent", "email"
]

const data_cache = [
    {
        id: 1,
        nom: "Woumo Birwe",
        prenom: "Elie",
        sexe: "M",
        birth_date: "04-10-1999",
        birth_place: "Garoua",
        classe: "2 annee",
        num: "+237 650 096 347",
        nom_p: "Woumo",
        prenom_p: "JP",
        num_p: "+237 650 096 347",
        email: "woumojohnny@gmail.com"
    },
    {
        id: 2,
        nom: "Woumo Birwe",
        prenom: "Elie",
        sexe: "M",
        birth_date: "04-10-1999",
        birth_place: "Garoua",
        classe: "2 annee",
        num: "+237 650 096 347",
        nom_p: "Woumo",
        prenom_p: "JP",
        num_p: "+237 650 096 347",
        email: "woumojohnny@gmail.com"
    },
    {
        id: 3,
        nom: "Woumo Birwe",
        prenom: "Elie",
        sexe: "M",
        birth_date: "04-10-1999",
        birth_place: "Garoua",
        classe: "2 annee",
        num: "+237 650 096 347",
        nom_p: "Woumo",
        prenom_p: "JP",
        num_p: "+237 650 096 347",
        email: "woumojohnny@gmail.com"
    }
]


function Table(props) {
    const data = props.data
    const real_head = Object.keys(data["0"]) // Contient tous les clef du dictionnaire
    let head = [
        "id", "Noms", "Prenoms", "Sexe", "Date de naissance", "Lieu de naissance",
        "Classe", "Tel", "Nom parent", "Prenom parent", "No Parent", "E-mail", "Admin_id" 
    ]


    return(
        <div className={styles.container}>
            <table className={styles.main}>
                <thead>
                    <tr>
                        {head.map((e, i) => <th key={i}>{e}</th>)}
                    </tr>
                </thead>
                <tbody>
                    {data.map((e, i) => <tr key={i} onClick={_ => props.go_to_member(e.id)}>
                        <td>{e.id}</td>
                        <td>{e.name.toUpperCase()}</td>
                        <td>{e.surname.toUpperCase()}</td>
                        <td>{e.sexe}</td>
                        <td>{e.birth_date}</td>
                        <td>{e.birth_place}</td>
                        <td>{e.classe}</td>
                        <td>{e.numero}</td>
                        <td>{e.parent_name.toUpperCase()}</td>
                        <td>{e.parent_surname.toUpperCase()}</td>
                        <td>{e.parent_numero}</td>
                        <td>{e.email}</td>
                        <td>{e.admin_id}</td>
                    </tr>)}
                </tbody>
            </table>
        </div>
    )
}

export default Table