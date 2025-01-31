use handlebars::HelperDef;

#[derive(Clone, Copy)]
pub struct MissingHelper;

impl HelperDef for MissingHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &handlebars::Helper<'rc>,
        _r: &'reg handlebars::Handlebars<'reg>,
        _ctx: &'rc handlebars::Context,
        _rc: &mut handlebars::RenderContext<'reg, 'rc>,
        out: &mut dyn handlebars::Output,
    ) -> handlebars::HelperResult {
        let params: String = h.params().iter().fold(String::new(), |acc, param| {
            let param = param.render();
            format!("{} {}", acc, param)
        });

        out.write(&format!("{{{{ {} }}}}", params))?;
        Ok(())
    }
}
