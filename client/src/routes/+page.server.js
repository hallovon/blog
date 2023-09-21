export async function load({ fetch }) {
    const resp = await fetch("http://127.0.0.1:3000/");
    const data = await resp.json();
    // console.log(data);
    return data;
}