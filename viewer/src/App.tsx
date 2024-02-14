import { useState } from 'react'
import { Statistics, fetchStatistics } from './api'
import { Summary } from './components/summary';
import { DegreeChart } from './components/degreeChart';

function App() {

  const [statistics, setStatistics] = useState<Statistics | null>(null)

  if (statistics === null) {
    fetchStatistics().then(setStatistics);
  }

  return (
    <>
      <h1>Wikilytics</h1>
      { statistics === null ? 
        <>
          Statistics not loaded
        </> 
        : 
        <div>
          <div>
            <Summary statistics={statistics} />
          </div>
          <div>
            <details>
              <summary>Outdegree distribution</summary>
                <figure>
                  <figcaption>Outdegree distribution</figcaption>
                  <DegreeChart data={statistics.outDegreeDistribution.map((numberOfNodes, degree) => [degree, numberOfNodes])} degreeLabel='Outdegree'/>
                </figure>
            </details>
          </div>
          <div>
            <details>
              <summary>Indegree distribution</summary>
                <figure>
                  <figcaption>Indegree distribution</figcaption>
                  <DegreeChart data={statistics.inDegreeDistribution.map((numberOfNodes, degree) => [degree, numberOfNodes])} degreeLabel='Indegree'/>
                </figure>
            </details>
          </div>
        </div>
      }
      
    </>
  )
}

export default App
