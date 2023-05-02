import { useAtom } from "jotai"
import { useEffect, useRef, useState } from "react"

import { FenceApi } from "@/lib/api"
import { displayStateAtom, regionStateAtom } from "@/lib/store"

export const DisplaysCanvas: React.FC = () => {
  const canvas = useRef(null)

  const [displays, setDisplays] = useAtom(displayStateAtom)
  const [regions, setRegions] = useAtom(regionStateAtom)

  const [topOffset, setTopOffset] = useState(0)
  const [leftOffset, setLeftOffset] = useState(0)
  const [factor, setFactor] = useState(0.1)
  const [canvasWidth, setCavasWidth] = useState(0)
  const [canvasHeight, setCanvasHeight] = useState(0)

  const drawCanvas = () => {
    setTopOffset(-displays.reduce((acc, display) => Math.min(acc, display.top), 0))
    setLeftOffset(-displays.reduce((acc, display) => Math.min(acc, display.left), 0))
    const highestRight = displays.reduce(
      (acc, display) => Math.max(acc, display.left + leftOffset + display.width),
      0,
    )
    const highestBottom = displays.reduce(
      (acc, display) => Math.max(acc, display.top + topOffset + display.height),
      0,
    )

    setCavasWidth(highestRight * factor)
    setCanvasHeight(highestBottom * factor)
  }
  useEffect(() => {
    drawCanvas()

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [displays])

  return (
    <div className="bg-gray-300 p-4 dark:bg-gray-950">
      <div>
        <div
          className="relative"
          ref={canvas}
          style={{ width: `${canvasWidth}px`, height: `${canvasHeight}px` }}
        >
          {/* {#each $displays as display}
				<div
					className="bg-gray-400 dark:bg-gray-800 border border-gray-600 dark:border-gray-700"
					style="
					position: absolute;
					top: {display.top * factor + topOffset * factor}px;
					left: {display.left * factor + leftOffset * factor}px;
					width: {display.width * factor}px;
					height: {display.height * factor}px;"
				/>
			{/each}

			{#each $regions as region}
				<div
					className="absolute bg-red-400/10"
					style="
					top: {region.y * factor + topOffset * factor}px;
					left: {region.x * factor + leftOffset * factor}px;
					width: {region.width * factor}px;
					height: {region.height * factor}px;"
				/>
			{/each} */}
          {displays.map((display, idx) => (
            <div
              key={idx}
              className="absolute border border-gray-600 bg-gray-400 dark:border-gray-700 dark:bg-gray-800"
              style={{
                top: `${display.top * factor + topOffset * factor}px`,
                left: `${display.left * factor + leftOffset * factor}px`,
                width: `${display.width * factor}px`,
                height: `${display.height * factor}px`,
              }}
            />
          ))}

          <div
            className="absolute h-2 w-2 rounded-full bg-violet-800"
            // style="top: {cursorY * factor + topOffset * factor}px; left: {cursorX * factor +
            // 		leftOffset * factor}px"
          />
        </div>
      </div>

      <div className="mt-4 flex items-center justify-end space-x-4 text-xs font-semibold text-gray-700 dark:text-gray-400">
        <div className="flex items-center">
          <input
            // bind:checked={showScreenshots}
            // on:change={handleShowScreenshots}
            type="checkbox"
            name="showScreenshots"
            id="checkbox-show-screenshots"
          />
          <label htmlFor="checkbox-show-screenshots" className="ml-2">
            {" "}
            Show Screenshots{" "}
          </label>
        </div>
        <button
          //   onClick={() => void FenceApi.refreshDisplays()}
          className="flex items-center space-x-2 rounded bg-gray-400/60 px-5 py-1 transition-all hover:bg-gray-400/80 dark:bg-gray-800 dark:hover:bg-gray-700"
        >
          {/* <Fa icon={faRefresh} /> */}
          <p>Refresh</p>
        </button>
      </div>
    </div>
  )
}
