use neon::prelude::*;
use clipboard::{ClipboardContext, ClipboardProvider};

fn get_clipboard(mut cx: FunctionContext) -> JsResult<JsString> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let contents = ctx.get_contents().unwrap();
    Ok(cx.string(contents))
}

fn set_clipboard(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let contents = cx.argument::<JsString>(0)?.value(&mut cx);
    ctx.set_contents(contents).unwrap();
    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("getClipboard", get_clipboard)?;
    cx.export_function("setClipboard", set_clipboard)?;
    Ok(())
}
