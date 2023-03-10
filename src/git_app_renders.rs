use egui::Ui;

use crate::{
    git_app::{GitApps, TagSymbol},
    GitApp,
};

pub fn render_git_app(app: &mut GitApp, ui: &mut Ui) {
    egui::Grid::new("git_apps_grid").show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.heading("Git Apps");

            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", app.selected_gits_app))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut app.selected_gits_app,
                        GitApps::TagDiffWeb,
                        "Web Diff",
                    );
                    ui.selectable_value(
                        &mut app.selected_gits_app,
                        GitApps::TagDiffCommand,
                        "Command Diff",
                    );
                });
        });

        ui.end_row();

        ui.vertical(|ui| match app.selected_gits_app {
            GitApps::TagDiffWeb => render_web_diff(app, ui),
            GitApps::TagDiffCommand => render_tag_diff(app, ui),
        });

        ui.end_row();
    });
}

fn render_tag_diff(app: &mut GitApp, ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.add_space(20.0);
        actions(app, ui);
        ui.end_row();
        settings(app, ui);

        ui.end_row();
    });
}

fn combine_choosen(app: &GitApp) -> String {
    let base = app.base_url.clone();

    let delimiter = "/".to_owned();

    let owner = app.owner.clone();

    let repo = app.repo.clone();

    let compare = "compare".to_owned();

    let one = app.choosen.get_choosen_symbol_chain();

    let points = "...".to_owned();

    let two = app.choosen_other.get_choosen_symbol_chain();

    let pre = vec![
        base,
        delimiter.clone(),
        owner,
        delimiter.clone(),
        repo,
        delimiter.clone(),
        compare,
        delimiter,
        one,
        points,
        two,
    ];
    pre.concat()
}

fn render_web_diff(app: &mut GitApp, ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.add_space(20.0);
        actions(app, ui);
        ui.end_row();
        settings(app, ui);
        ui.end_row();
        ui.label(combine_choosen(app));
        ui.hyperlink(combine_choosen(app))
    });
}

fn actions(app: &mut GitApp, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.collapsing("Actions", |ui| {
            ui.horizontal(|ui| {
                if ui.button("create new").clicked() {
                    app.choosen.choose(TagSymbol::new(1, "prefix", "something"));
                    app.choosen.choose(TagSymbol::new(2, "delitmiter1", "/"));
                    app.choosen
                        .choose(TagSymbol::new(3, "inner", "someversion"));
                    app.choosen.choose(TagSymbol::new(4, "delitmiter11", "/"));
                    app.choosen
                        .choose(TagSymbol::new(5, "delimiter", "someversion"));
                };
            });

            if ui.button("take over").clicked() {
                app.choosen_other.take_over(&app.choosen)
            };
        });
    });
}

fn settings(app: &mut GitApp, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.collapsing("Settings", |ui| {
            ui.vertical(|ui| {
                ui.text_edit_singleline(&mut app.base_url);
                ui.text_edit_singleline(&mut app.owner);
                ui.text_edit_singleline(&mut app.repo);
            });

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let mut cloned = app.choosen.get_choosen_symbols();
                    for choosen in &mut cloned {
                        ui.text_edit_singleline(&mut choosen.symbol);
                        ui.add_space(1.0);
                    }
                    for new in cloned.clone() {
                        app.choosen.choose(new)
                    }
                });

                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    let mut cloned_other = app.choosen_other.get_choosen_symbols();
                    for choosen_other in &mut cloned_other {
                        ui.text_edit_singleline(&mut choosen_other.symbol);
                        ui.add_space(2.0);
                    }
                    for new in cloned_other.clone() {
                        app.choosen_other.choose(new)
                    }
                });
            });
        })
    });
}
