#![deny(warnings)]
use sauron::{
    html::{
        attributes::*,
        *,
    },
    mt_dom::patch::*,
    node,
    Patch,
    *,
};

use test_fixtures::simple_program;
use wasm_bindgen_test::*;

mod test_fixtures;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_lines() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();
    let _document = web_sys::window().unwrap().document().unwrap();

    let view0: Node<()> = node!(
    <div class="app">
       <h1>"Lines"</h1>
       <div>
          <div class="grid__wrapper">
             <div class="grid grid__number_wide1">
                <div class="grid__number__line" key="623356695095054844">
                   <div class="grid__number">"0"</div>
                   <div class="grid__line">
                      <div>"C"</div>
                      <div>"J"</div>
                      <div>"K"</div>
                      <div>""</div>
                   </div>
                </div>
                <div class="grid__number__line" key="9824372840226575955">
                   <div class="grid__number">"1"</div>
                   <div class="grid__line">
                      <div>"T"</div>
                      <div>"h"</div>
                      <div>"e"</div>
                      <div>""</div>
                   </div>
                </div>
                <div class="grid__number__line" key="17452480237312370641">
                   <div class="grid__number">"2"</div>
                   <div class="grid__line">
                      <div>"S"</div>
                      <div>"v"</div>
                      <div>"g"</div>
                      <div>"b"</div>
                      <div>""</div>
                   </div>
                </div>
                <div class="grid__number__line" key="16116632904053897079">
                   <div class="grid__number">"3"</div>
                   <div class="grid__line">
                      <div>"w"</div>
                      <div>"h"</div>
                      <div>"i"</div>
                      <div>"c"</div>
                      <div>""</div>
                   </div>
                </div>
                <div class="grid__number__line" key="1175237220632156455">
                   <div class="grid__number">"4"</div>
                   <div class="grid__line">
                      <div>"t"</div>
                      <div>"o"</div>
                      <div>" "</div>
                      <div>"a"</div>
                      <div>""</div>
                   </div>
                </div>
             </div>
             <div class="grid__status">"line: 0, column: 0"</div>
          </div>
       </div>
    </div>
    );

    log::trace!("render: {}", view0.render_to_string());

    let view1: Node<()> = node!(
    <div class="app">
       <h1>"Lines"</h1>
       <div>
          <div class="grid__wrapper">
             <div class="grid grid__number_wide1">
                <div class="grid__number__line" key="4638962052468762037">
                   <div class="grid__number">"0"</div>
                   <div class="grid__line">
                      <div>""</div>
                   </div>
                </div>
                <div class="grid__number__line" key="623356695095054844">
                   <div class="grid__number">"1"</div>
                   <div class="grid__line">
                      <div>"C"</div>
                      <div>"J"</div>
                      <div>"K"</div>
                      <div>""</div>
                   </div>
                </div>
                <div class="grid__number__line" key="9824372840226575955">
                   <div class="grid__number">"2"</div>
                   <div class="grid__line">
                      <div>"T"</div>
                      <div>"h"</div>
                      <div>"e"</div>
                      <div>""</div>
                   </div>
                </div>
                <div class="grid__number__line" key="17452480237312370641">
                   <div class="grid__number">"3"</div>
                   <div class="grid__line">
                      <div>"S"</div>
                      <div>"v"</div>
                      <div>"g"</div>
                      <div>"b"</div>
                      <div>""</div>
                   </div>
                </div>
                <div class="grid__number__line" key="16116632904053897079">
                   <div class="grid__number">"4"</div>
                   <div class="grid__line">
                      <div>"w"</div>
                      <div>"h"</div>
                      <div>"i"</div>
                      <div>"c"</div>
                      <div>""</div>
                   </div>
                </div>
                <div class="grid__number__line" key="1175237220632156455">
                   <div class="grid__number">"5"</div>
                   <div class="grid__line">
                      <div>"t"</div>
                      <div>"o"</div>
                      <div>" "</div>
                      <div>"a"</div>
                      <div>""</div>
                   </div>
                </div>
             </div>
             <div class="grid__status">"line: 1, column: 0"</div>
          </div>
       </div>
    </div>
        );

    let patch1_diff = diff(&view0, &view1);
    log::trace!("patch1_diff: {:#?}", patch1_diff);

    let inserted = div(
        vec![class("grid__number__line"), key("4638962052468762037")],
        vec![
            div(vec![class("grid__number")], vec![text(0)]),
            div(vec![class("grid__line")], vec![div(vec![], vec![text("")])]),
        ],
    );

    let patch1_expected: Vec<Patch<()>> = vec![
        ChangeText::new(8, "0", "1").into(),
        ChangeText::new(20, "1", "2").into(),
        ChangeText::new(32, "2", "3").into(),
        ChangeText::new(46, "3", "4").into(),
        ChangeText::new(60, "4", "5").into(),
        InsertNode::new(Some(&"div"), 6, &inserted).into(),
        ChangeText::new(73, "line: 0, column: 0", "line: 1, column: 0").into(),
    ];
    assert_eq!(patch1_diff, patch1_expected);

    let simple_program = simple_program();
    let mut dom_updater = DomUpdater::new_append_to_mount(
        &simple_program,
        view0.clone(),
        &sauron_core::body(),
    );

    dom_updater.patch_dom(&simple_program, patch1_expected);

    let patch2_expected: Vec<Patch<()>> = vec![
        ChangeText::new(14, "1", "2").into(),
        ChangeText::new(26, "2", "3").into(),
        ChangeText::new(38, "3", "4").into(),
        ChangeText::new(52, "4", "5").into(),
        ChangeText::new(66, "5", "6").into(),
        ChangeText::new(79, "line: 1, column: 0", "line: 2, column: 0").into(),
    ];

    dom_updater.patch_dom(&simple_program, patch2_expected);

    let view2 = dom_updater.current_vdom.clone();
    log::trace!("view2_rendered: {}", view2.render_to_string());
    let patch2_diff = diff(&view1, &view2);
    log::trace!("patch2_diff: {:#?}", patch2_diff);
    //assert_eq!(patch2_diff, patch2_expected);

    let patch3: Vec<Patch<()>> = vec![
        ChangeText::new(20, "2", "3").into(),
        ChangeText::new(32, "3", "4").into(),
        ChangeText::new(44, "4", "5").into(),
        ChangeText::new(58, "5", "6").into(),
        ChangeText::new(72, "6", "7").into(),
        RemoveNode::new(Some(&"div"), 5).into(),
        ChangeText::new(85, "line: 2, column: 0", "line: 3, column: 0").into(),
    ];

    // will FAIL here
    // this is a change text: ChangeText { node_idx: 20, old: "2", new: "3" } for "<div node_idx=\"14\">K</div>"
    //
    // console.log div contained:
    //    panicked at 'internal error: entered unreachable code: Elements should not receive ChangeText patches.'
    dom_updater.patch_dom(&simple_program, patch3);
}
