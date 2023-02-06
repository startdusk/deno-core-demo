function print(data) {
    Deno.core.print(`${data}\n`)
}

print("starting to fetch...")
let res = await fetch({ url : "http://suggest.taobao.com/sug?code=utf-8&q=商品关键字&callback=cb" })
print(`status: ${res.status}`)
print(`headers: ${JSON.stringify(res.headers, null, 2)}`)
print(`body: ${res.json()}`)