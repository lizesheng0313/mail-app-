export async function runPromisePool<TItem, TResult>(
  items: TItem[],
  concurrency: number,
  worker: (item: TItem, index: number) => Promise<TResult>
): Promise<TResult[]> {
  if (!items.length) {
    return []
  }

  const limit = Math.max(1, Math.min(concurrency, items.length))
  const results: TResult[] = new Array(items.length)
  let nextIndex = 0

  const runWorker = async () => {
    while (true) {
      const currentIndex = nextIndex
      nextIndex += 1

      if (currentIndex >= items.length) {
        return
      }

      results[currentIndex] = await worker(items[currentIndex], currentIndex)
    }
  }

  await Promise.all(Array.from({ length: limit }, () => runWorker()))
  return results
}
