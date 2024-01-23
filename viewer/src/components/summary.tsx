import { Statistics } from '../api'
import { pageUrl } from '../urls';

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
                    <td>Articles with the most links</td><td>{statistics.nodesOfMaxOutDegree.map(pageName => <a href={pageUrl(statistics.mainPage, pageName).toString()}>{pageName}</a>)}</td>
                </tr>
                <tr>
                    <td>Number of links in the articles with the most links</td><td>{statistics.maxOutDegree}</td>
                </tr>
                <tr>
                    <td>Articles which are linked the most</td><td>{statistics.nodesOfMaxInDegree.map(pageName => <a href={pageUrl(statistics.mainPage, pageName).toString()}>{pageName}</a>)}</td>
                </tr>
                <tr>
                    <td>Number of times the most linked article is linked</td><td>{statistics.maxInDegree}</td>
                </tr>
            </tbody>
        </table>
    </>
}