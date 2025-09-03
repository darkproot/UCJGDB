import styles from './InfoBar.module.css'

function InfoBar({ open }) {

    return(
        <div className={styles.container}>
            <span className={styles.title}>Jeunes</span>
            <div className={styles.gap}></div>

            <div className={styles.action}>
                <input type="text" id="search-bar" className={styles.search} placeholder='Search'/>
                <input type="button" value="Telecharger" id="download"/>
                <button onClick={open}>Ajouter</button>
            </div>
        </div>
    )
}

export default InfoBar