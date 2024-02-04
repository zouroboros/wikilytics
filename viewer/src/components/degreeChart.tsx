import * as d3 from "d3"
import { useEffect, useRef } from "react"

export function DegreeChart({ data, degreeLabel }: { data: [number, number][], degreeLabel: string }) {
    const svgRef = useRef<SVGSVGElement>(null)

    useEffect(() => {
        const width = 640
        const height = 400
        const marginTop = 50;
        const marginRight = 50;
        const marginBottom = 50;
        const marginLeft = 50;

        const xValues = data.map(entry => entry[0])
        const yValues = data.map(entry => entry[1])
    
        const xMax = xValues.reduce((max, x) => Math.max(max, x), Number.MIN_VALUE)
        const yMax = yValues.reduce((max, y) => Math.max(max, y), Number.MIN_VALUE)

        const xScale = d3.scaleLog([1, xMax], [marginLeft, width - marginRight])
        const yScale = d3.scaleLog([1, yMax], [height - marginBottom, marginTop])

        if (svgRef.current !== null) {
            svgRef.current.innerHTML = ""
        }

        const chartSvg = d3.select(svgRef.current)
            .attr("viewBox", [0, 0, width, height])

        const radius = 2;

        chartSvg.append("g")
            .attr("fill", "steelblue")
            .selectAll()
            .data(data.filter(xy => xy[1] !== 0))
            .join("circle")
                .attr("cx", xy => xScale(xy[0]))
                .attr("cy", xy => yScale(xy[1]))
                .attr("r", radius)

        chartSvg.append("g")
            .attr("transform", `translate(0, ${height - marginBottom})`)
            .call(d3.axisBottom(xScale))
            .call(g => g.append("text")
                .attr("x", width / 2)
                .attr("y", 30)
                .attr("fill", "currentColor")
                .text(degreeLabel))

        chartSvg.append("g")
            .attr("transform", `translate(${marginLeft}, 0)`)
            .call(d3.axisLeft(yScale))
            .call(g => g.append("text")
                .attr("x", 40)
                .attr("y", 40)
                .attr("fill", "currentColor")
                .text("Number of Nodes"))

    }, [data])

    return <>
        <svg ref={svgRef}></svg>
    </>
};