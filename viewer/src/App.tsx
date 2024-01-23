import { useState } from 'react'
import { Statistics, fetchStatistics } from './api'
import { Summary } from './components/summary';

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
          <Summary statistics={statistics} />
        </div>
      }
      
    </>
  )
}

export default App
