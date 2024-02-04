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
            <DegreeChart data={statistics.inDegreeDistribution.map((numberOfNodes, degree) => [degree, numberOfNodes])} degreeLabel='Indegree'/>
          </div>
        </div>
      }
      
    </>
  )
}

export default App
