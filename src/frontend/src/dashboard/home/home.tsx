import React from 'react'
import onlineChart from '../widgets/charts/online'
import eventCount from '../widgets/events/eventCount'
import basicMetrics from '../widgets/metrics/basic'
import Navbar from '../navbar'
import { useEffect } from 'react'

export default function Home(): JSX.Element {
  useEffect(() => {
    onlineChart()
  }, [])

  function renderHome(): JSX.Element {
    return (
      <div>
        <div className="w-full pl-28 pr-28 pt-10">
          <div className="flex flex-col justify-center">
            <div className="text-3xl mb-10">Template website</div>
            {basicMetrics()}

            <div className="mt-8 ">
              <div className="tab tab-lifted tab-active text-xl hover:cursor-default h-11 [--tab-bg:#191D25] ">
                Online users :
              </div>
            </div>

            <div className="bg-white rounded-2xl rounded-tl-none">
              <div id="chart-online" className="m-3"></div>
            </div>
            <div>
              <div className="my-8 text-2xl">
                <div className="tab tab-lifted tab-active text-xl hover:cursor-default h-11 [--tab-bg:#191D25] ">
                  Events :
                </div>
                <div className=" bg-gray-900 w-full rounded-2xl rounded-tl-none">
                  <div className="p-10">{eventCount()}</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    )
  }
  return <Navbar activeCompName="Home" activeComp={renderHome}></Navbar>
}
