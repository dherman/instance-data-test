use neon::prelude::*;
use once_cell::sync::OnceCell;

static CELL: OnceCell<Root<JsObject>> = OnceCell::new();

fn init(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let s: Handle<JsString> = cx.argument(0)?;
    let o: Handle<JsObject> = cx.empty_object();
    o.set(&mut cx, "value", s)?;
    let r: Root<JsObject> = Root::new(&mut cx, &o);
    if let Err(_) = CELL.set(r) {
        cx.throw_error("already initialized")?;
    }
    Ok(cx.undefined())
}

fn get(mut cx: FunctionContext) -> JsResult<JsValue> {
    let r = match CELL.get() {
        Some(r) => r,
        None => { return cx.throw_error("not initialized")?; }
    };
    let o: Handle<JsObject> = r.to_inner(&mut cx);
    let v: Handle<JsValue> = o.get(&mut cx, "value")?;
    Ok(v)
}

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello from neon"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("init", init)?;
    cx.export_function("get", get)?;
    cx.export_function("hello", hello)?;
    Ok(())
}
