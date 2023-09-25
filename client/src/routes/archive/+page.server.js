export async function load({ fetch }) {
  const resp = await fetch(`http://127.0.0.1:3000/api/article/archive`);
  const data = await resp.json();

  return data.data;
}
