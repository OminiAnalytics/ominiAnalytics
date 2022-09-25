import React from 'react'
import upArrowImg from '../../../icons/up-arrow.png'
import downArrowImg from '../../../icons/down-arrow.png'

export default function basicMetrics(): JSX.Element {
  const upArrow = (): JSX.Element => {
    return (
      <span>
        <img src={upArrowImg} className="w-6 h-auto m-1" />
      </span>
    )
  }
  const downArrow = (): JSX.Element => {
    return (
      <span>
        <img src={downArrowImg} className="w-6 h-auto m-1" />
      </span>
    )
  }

  return (
    <div className="stats stats-vertical lg:stats-horizontal border-2 border-white w-full">
      <div className="stat">
        <div className="stat-title">Online users</div>
        <div className="stat-value" id="online-users">
          101
        </div>
      </div>
      <div className="stat">
        <div className="stat-title">Unique users</div>
        <div className="stat-value" id="unique-users">
          2K
        </div>
        <div className="stat-desc flex flex-row items-center">Today {upArrow()} +9%</div>
      </div>
      <div className="stat">
        <div className="stat-title">New users</div>
        <div className="stat-value" id="new-users">
          50
        </div>
        <div className=" stat-desc flex flex-row items-center">Today {downArrow()}-2%</div>
      </div>
      <div className="stat">
        <div className="stat-title">Views</div>
        <div className="stat-value" id="views-today">
          14K
        </div>
        <div className=" stat-desc">Today</div>
      </div>
    </div>
  )
}
