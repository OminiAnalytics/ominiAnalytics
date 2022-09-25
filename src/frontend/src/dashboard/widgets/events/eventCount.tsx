import React from 'react'
import interactionImg from '../../../icons/interaction_icon.png'
import financialImg from '../../../icons/financial_icon.png'

export default function EventCount(): JSX.Element {
  const eventImg: { [key: string]: string } = {
    Interactions: interactionImg,
    Financial: financialImg
  }

  const eventsRender = (): JSX.Element => {
    const events: JSX.Element[] = []
    ;[
      { name: 'Login button clicked', count: 162, category: 'Interactions' },
      { name: 'New subscription', count: 11, category: 'Financial' }
    ].forEach((event) => {
      events.push(
        <div className="grid  h-32 card bg-base-300 rounded-box place-items-center text-xl">
          <div className="w-full h-full flex flex-row justify-start items-center">
            <img src={eventImg[event.category]} className="w-8 h-auto mx-6" />
            <div>
              <span className="font-bold text-[1.60rem]">{event.count}</span>
              <span> {event.name}</span>
              <br />
              <span className="stat-desc text-lg">Today</span>
            </div>
          </div>
        </div>
      )
    })
    return <>{events}</>
  }
  return (
    <div className="grid gap-8 grid-cols-4  justify-self-center place-content-center">
      {eventsRender()}
    </div>
  )
}
