import { Statistics } from '../api'



export function Summary({ statistics }: { statistics: Statistics;}) {
    return <>
        <table>
            <tbody>
                <tr>
                    <td>Number of articles</td><td>{statistics.numberOfNodes}</td>
                </tr>
                <tr>
                    <td>Number of links</td><td>{statistics.numberOfEdges}</td>
                </tr>
                <tr>
                    <td>Articles with the most links</td><td>{statistics.nodesOfMaxOutDegree.join(",")}</td>
                </tr>
                <tr>
                    <td>Articles which are linked the most</td><td>{statistics.nodesOfMaxInDegree.join(",")}</td>
                </tr>
            </tbody>
        </table>
    </>
}