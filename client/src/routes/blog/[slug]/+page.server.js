export async function load({ fetch, params }) {
    const resp = await fetch(`http://127.0.0.1:3000/api/article/${params.slug}`);
    const data = await resp.json();
    return data;
}