import React, { useEffect, useRef } from 'react'
// eslint-disable-next-line @typescript-eslint/no-var-requires
import logo from './../icons/omini_logov2.png'

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export default function Login(): JSX.Element {
  const userRef = useRef() as React.MutableRefObject<HTMLInputElement>
  const errRef = useRef() as React.MutableRefObject<HTMLInputElement>
  const [user, setUser] = React.useState('')
  const [pass, setPass] = React.useState('')
  const [errMsg, setErr] = React.useState('')

  async function sendCreds(username: string, password: string) {
    await fetch(process.env.REACT_APP_API + '/dsh/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        username: username,
        password: password
      })
    })
      .then((data) => {
        if (data.status === 200) {
          return data.json()
        } else {
          setErr('Error communicating with server')
          throw new Error('Error communicating with server')
          // setPass('');
        }
      })
      .then((data) => {
        if (data.status === 'success') {
          window.location.href = '/omidash/home'
        } else {
          setPass('')
          setErr('Bad credentials. Please try again.')
        }
      })
      .catch((error) => {
        console.error('Error:', error)
      })
  }

  useEffect(() => {
    userRef.current.focus()
  }, [])
  useEffect(() => {
    setErr('')
  }, [user, pass])

  return (
    <div>
      <div
        ref={errRef}
        className={'translate-x-1/2 w-1/2 top-8 fixed ' + (errMsg ? 'visible' : 'invisible')}
      >
        <div className="alert alert-error shadow-lg">
          <div>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              className="stroke-current flex-shrink-0 h-6 w-6"
              fill="none"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9
                9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            <span>{errMsg}</span>
          </div>
        </div>
      </div>
      <div
        className="flex items-center justify-center 
        h-screen flex-col"
      >
        <div className="border-gray-300 border-2 rounded-xl">
          <div className="form-control gap-8 m-10 align-center">
            <div
              className="flex flex-row items-center 
            justify-center space-x-4"
            >
              <img src={logo} alt="omini logo" className="w-20 h-20" />
              <div className="flex flex-col items-center justify-center">
                <span className="text-2xl font-semibold black dark:text-white">Omini</span>
                <div className="text-gray-400 text-[11px] -mt-1 -mr-8">
                  <span>dashboard</span>
                </div>
              </div>
            </div>
            <label className="input-group input-group-vertical">
              <span>Username</span>
              <input
                ref={userRef}
                required
                autoComplete="off"
                value={user}
                onChange={(e) => setUser(e.target.value)}
                type="username"
                placeholder="Username"
                className="input input-bordered bg-white text-gray-600"
              />
            </label>
            <label className="input-group input-group-vertical">
              <span>Password</span>
              <input
                required
                autoComplete="off"
                value={pass}
                onChange={(e) => setPass(e.target.value)}
                type="password"
                placeholder="********"
                className="input input-bordered bg-white text-gray-600"
              />
            </label>

            <input
              type="submit"
              value="Submit"
              onClick={() => {
                sendCreds(user, pass)
              }}
              className="
            btn text-white bg-blue-500
          hover:bg-blue-700 w-1/2"
            />
          </div>
        </div>
        <div
          className="tooltip mt-6"
          data-tip='Use the password "Administrator" 
      and your database password'
        >
          <div className="flex items-center">
            <span>First connection</span>
            <span className="ml-2">
              <div className="stat-figure text-blue-500">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  className="inline-block w-7 h-7 stroke-current"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth="2"
                    d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                  ></path>
                </svg>
              </div>
            </span>
          </div>
        </div>
      </div>
    </div>
  )
}
