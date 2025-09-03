import styles from './AdminsInfoBar.module.css'

function AdminsInfoBar({ go_to_admin }) {
    return(
        <div className={styles.container}>
            <span className={styles.title}>Admins</span>

            <button onClick={() => go_to_admin(null, false)}>Ajouter</button>
        </div>
    )   
}

export default AdminsInfoBar