import React from 'react'
import Login from './auth/loginPage'
import './style/output.css'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import Metrics from './dashboard/metrics/metrics'
import Home from './dashboard/home/home'

function App(): JSX.Element {
  return (
    <div className="App">
      <div className="wrapper">
        <BrowserRouter>
          <Routes>
            <Route path={process.env.REACT_APP_HOST + '/home'} element={<Home />}></Route>
            <Route path={process.env.REACT_APP_HOST + '/metrics'} element={<Metrics />}></Route>
            <Route path={process.env.REACT_APP_HOST + '/login'} element={<Login />}></Route>
          </Routes>
        </BrowserRouter>
      </div>
    </div>
  )
}

export default App
