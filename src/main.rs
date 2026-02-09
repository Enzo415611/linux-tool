// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, io, process::{Child, Command, Output}, rc::Rc};

use raur::{Package, Raur};
use slint::{ComponentHandle, Model, ModelRc, SharedString, ToSharedString, VecModel};

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let logic = ui.global::<Logic>();

    let ui_handle = ui.as_weak();
    
    logic.on_search_pkg({
        move |app_name| {
            let handle = ui_handle.unwrap();

            slint::spawn_local(async_compat::Compat::new(async move {
                let logic = handle.global::<Logic>();

                let pkgs = search_pkg(&app_name.to_string()).await;

                if let Ok(pkgs) = pkgs {
                    let mut pkgs_info: Vec<PackagesInfo> = vec![];

                    for pkg in pkgs {
                        let description = match pkg.description {
                            Some(d) => d,
                            None => String::from("NA"),
                        };

                        let maintainer = match pkg.maintainer {
                            Some(ma) => ma,
                            None => String::from("NA"),
                        };

                        let mut co_maintainers: ModelRc<SharedString> = ModelRc::default();

                        for cm in pkg.co_maintainers {
                            co_maintainers =
                                ModelRc::new(VecModel::from(vec![cm.to_shared_string()]));
                        }

                        let mut depends: ModelRc<SharedString> = ModelRc::default();

                        for depe in pkg.depends {
                            depends = ModelRc::new(VecModel::from(vec![depe.to_shared_string()]));
                        }

                        let packages_info = PackagesInfo {
                            package_base: pkg.package_base.into(),
                            version: pkg.version.into(),
                            description: description.into(),
                            maintainer: maintainer.into(),
                            co_maintainers: co_maintainers,
                            depends: depends,
                        };

                        pkgs_info.push(packages_info);
                    }

                    let the_model = Rc::new(VecModel::from(pkgs_info));
                    logic.set_pkgs_info(ModelRc::from(the_model));
                }

                println!("{:?}", app_name.clone());
            }))
            .unwrap();
        }
    });
    
    logic.on_install_pkg(|pkg_name| {
        println!("{}", pkg_name);
        println!("{:?}", install_pkg(pkg_name.into()));
    });

    ui.run()?;

    Ok(())
}

async fn search_pkg(app_name: &String) -> Result<Vec<Package>, raur::Error> {
    let raur = raur::Handle::new();
    let pkgs = raur.search(app_name).await?;
    for pkg in &pkgs {
        println!("{}", pkg.package_base);
    }
    Ok(pkgs)
}

fn install_pkg(pkg_name: String) -> Result<Child, io::Error> {
    let output = Command::new("pkexec")
        .args(["yay", "-S", "--noconfirm" ,pkg_name.as_str()])
        .spawn()?;
    
    Ok(output)
}
