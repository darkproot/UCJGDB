import { useState } from 'react'
import Login from './pages/Login/Login'
import Home from './pages/Home/Home'


function App() {

  const [page, setPage] = useState('login')

  const handleClick = () => { setPage('home') }

  return (
    <>
      {page === 'login' && <Login changePage={handleClick} />}
      {page === 'home' && <Home />}
    </>
  )
} 

export default App
