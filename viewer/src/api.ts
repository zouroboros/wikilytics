export interface Statistics {
    mainPage: string;
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