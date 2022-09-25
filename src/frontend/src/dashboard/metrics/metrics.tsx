import React from 'react'
import Navbar from '../navbar'

export default function Metrics(): JSX.Element {
  function renderHome(): JSX.Element {
    return (
      <div>
        <div className="w-full pl-28 pr-28 pt-10">
          <div className="flex flex-col justify-center">
            <div className="text-3xl mb-10">Template website</div>

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
              </div>
            </div>
          </div>
        </div>
      </div>
    )
  }
  return <Navbar activeCompName="Metrics" activeComp={renderHome}></Navbar>
}
