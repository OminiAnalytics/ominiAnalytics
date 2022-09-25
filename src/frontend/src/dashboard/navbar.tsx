import React from 'react'
import { ReactComponent as MetricsLogo } from '../icons/metrics_icon.svg'
import { ReactComponent as OminiLogo } from '../icons/omini_icon.svg'
import { verifySession } from '../auth/verifySession'
import { Link } from 'react-router-dom'

type compname = 'Home' | 'Metrics'
async function verifySessionAndRedirect(): Promise<boolean> {
  return await verifySession()
}

export default function Navbar({
  activeCompName,
  activeComp
}: {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  activeComp: () => any
  activeCompName: compname
}): JSX.Element {
  const isValidUser = verifySessionAndRedirect()
  isValidUser.then((res) => {
    if (!res) {
      window.location.href = process.env.REACT_APP_HOST + '/login'
    } else {
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
      const [isHome, _setIsHome] = React.useState(activeCompName === 'Home' ? true : false)
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
      const [isMetrics, _setIsMetrics] = React.useState(activeCompName === 'Metrics' ? true : false)
      const _renderElement = (
        width: string,
        isElem: boolean,
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        Logo: any,
        link: string
      ): JSX.Element => {
        return (
          <Link to={process.env.REACT_APP_HOST + link} className="w-full">
            <div
              className={
                boxClass +
                (isElem ? ' border-l-[5px] border-[#098FDF]' : ' border-l-[5px] border-white')
              }
            >
              <Logo className={`w-[${width}rem] ` + imgClass} />
            </div>
          </Link>
        )
      }

      const imgClass = 'h-auto'
      const boxClass = 'w-full h-24 cursor-pointer flex align-center justify-center items-center'
      return (
        <div>
          <div className="flex flex-col justify-start items-center bg-slate-50 w-24 fixed h-full top-0">
            {_renderElement('4', isHome, OminiLogo, '/home')}
            {_renderElement('2.9', isMetrics, MetricsLogo, '/metrics')}
          </div>
          <div className="ml-24">{activeComp()}</div>
        </div>
      )
    }
  })
  return <div></div>
}
