export type AnalyzePrInput = {
  prLink: string
  githubToken?: string
  openaiToken?: string
  anthropicToken?: string
  relatedRepositories?: string[]
}


export async function analyzePrQuery(input: AnalyzePrInput) {
  const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/analyze`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(input)
  });

  const jsonData = await response.json();

  return jsonData;
}
