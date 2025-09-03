import styles from '../Table/Table.module.css'

function Admins({ data, go_to_admin }) {

    const head = ['id', 'Name', 'Password']

    return(
        <div className={styles.container}>
            <table className={styles.main}>
                <thead>
                    <tr>
                        {head.map((e, i) => <th key={i}>{e}</th>)}
                    </tr>
                </thead>
                <tbody>
                    {data.map((e, i) => <tr key={i} onClick={() => go_to_admin(e.id, true)}>
                        <td>{e.id}</td>
                        <td>{e.name}</td>
                        <td>{e.password}</td>
                    </tr>)}
                </tbody>
            </table>
        </div>
    )
}

export default Admins