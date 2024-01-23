export interface Statistics {
    numberOfNodes: number;
    numberOfEdges: number;
    nodesOfMaxOutDegree: [string];
    maxOutDegree: number;
    nodesOfMaxInDegree: [string];
    maxInDegree: number;
    outDegreeDistribution: [number];
    inDegreeDistribution: [number];
}

export function fetchStatistics(): Promise<Statistics> {
    return fetch("statistics.json")
        .then(response => response.json());
}