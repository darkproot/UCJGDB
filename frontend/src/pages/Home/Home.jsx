import { useEffect } from 'react'
import { useState } from 'react'
import axios from 'axios'

import styles from './Home.module.css'
import TopBar from '../../components/TopBar/TopBar'
import InfoBar from '../../components/InfoBar/InfoBar'
import Table from '../../components/Table/Table'
import BottomBar from '../../components/Bottombar/BottomBar'
import Member from '../../components/Member/Member'
import MemberInfoBar from '../../components/MemberInfoBar/MemberInfoBar'
import NewMember from '../../components/NewMember/NewMember'
import Admins from '../../components/Admins/Admins'
import Admin from '../../components/Admin/Admin'
import AdminInfoBar from '../../components/AdminInfoBar/AdminInfoBar'
import AdminsInfoBar from '../../components/AdminsInfoBar/AdminsInfoBar'

const API = 'http://192.168.1.104:5000'


function Home() {

    const [openDialog1, setOpenDialog1] = useState(false)
    const toggleDialog1 = () => {setOpenDialog1(!openDialog1)}

    const [data, setData] = useState()
    const [memberId, setMemberId] = useState()
    const [admins, setAdmins] = useState()
    const [readOnly, setReadOnly] = useState(true)
    const [admin, setAdmin] = useState()

    const fetchData = () => {
        const data = {
            admin_id: JSON.parse(window.sessionStorage.getItem('admin_data')).id
        }
        
        axios.post(API + '/members', data).then(res => {
            if (res.data.err) alert(res.data.err)
                else setData(res.data)
        }).catch(err => console.log(err))
    }
    
    useEffect(() => { 
        fetchData() 
    }, [])
    
    const [info, setInfo] = useState(<InfoBar open={toggleDialog1} />)
    const [content, setContent] = useState("table")

    const go_to_admin = (adminId, readOnly) => {
        setReadOnly(readOnly)
        setAdmin(adminId)
        setInfo(<AdminInfoBar back={go_to_admins} delId={adminId} />)
        setContent('admin')
    }

    const go_to_admins = () => {
        setInfo(<AdminsInfoBar go_to_admin={go_to_admin} />)
        const admin_id = JSON.parse(window.sessionStorage.getItem('admin_data')).id

        axios.get(API + `/admin/${admin_id}`)
            .then(res => {
                if (res.data.err) alert(res.data.err)
                else {
                    const dat = res.data
                    setAdmins(dat)
                    setContent('admins')
                }
            })
            .catch(err => console.log(err))
    }
    const go_to_table = () => {
        fetchData()
        setInfo(<InfoBar open={toggleDialog1} />)
        setContent("table")
    }
    const go_to_member = (id) => {
        setMemberId(id)
        setInfo(<MemberInfoBar back={go_to_table} memberId={memberId} />)
        setContent("member")
    }
    const refresh_members = () => {
        fetchData()
        setInfo(<InfoBar open={toggleDialog1} />)
        setContent('table')
    }

    return(
        <>
            {openDialog1 && <NewMember handleClosing={toggleDialog1} go_to_member={go_to_member} />}
            <div className={styles.main}>
                <TopBar refresh={refresh_members} go_to_admin={go_to_admins} />
                <div className={styles.action} >
                    {info}
                    {admins && content === 'admins' && <Admins data={admins} go_to_admin={go_to_admin} />}
                    {data && content === 'table' && <Table data={data} go_to_member={go_to_member} />}
                    {content === 'member' && <Member back={go_to_table} memberId={memberId}/>}
                    {content === 'admin' && <Admin back={go_to_admins} readOnly={readOnly} AdminId={admin} />}
                </div>
                <BottomBar />
            </div>
        </>
    )
}

export default Home