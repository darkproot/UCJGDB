import { useEffect } from 'react'
import styles from './TopBar.module.css'

function TopBar({ refresh, go_to_admin }) {
    
    let tabs = ['jeunes']
    const user = JSON.parse(window.sessionStorage.getItem('admin_data'))
    if (user.id === 1) tabs.push('Admins')
    useEffect(_ => {    
        document.getElementById('jeunes')
            .addEventListener('click', () => refresh())

        if (user.id === 1)
            document.getElementById('Admins')
                .addEventListener('click', () => go_to_admin())
    }, [])

    return(
        <nav className={styles.top}>
            <label className={styles.title}>UCJG <span>Database</span></label>
            <div className={styles.gap}></div>

            <div className={styles.tab_container}>
                {tabs.map((element, index) => <label className={styles.tab} key={index} id={element}>{element}</label>)}
            </div>

            <div className={styles.conn}>
                <label className={styles.user}>{"Guest" && user.name} </label>
                <span>connected</span>
            </div>
        </nav>
    )
}

export default TopBar