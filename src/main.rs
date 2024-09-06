use colored::Colorize;
use core::time;
use rfd::FileDialog;
use std::{
    env,
    path::PathBuf,
    process::{exit, Command},
    str::FromStr,
    thread::{self},
};

const DATA_SERVER_ARGS: &str = "--cors -l 8589934592 -p 8001"; // upload-limit 8GB
const IGV_SERVER_ARGS: &str = "--cors -l 8589934592 -p 8000 -i";
const IGV_WEBAPP_PATH: &str = "Resources/igv-webapp";

fn get_server_executable_name() -> Result<PathBuf, String> {
    let os_str = match env::consts::OS {
        "linux" => "unknown-linux-musl",
        "windows" => "pc-windows-msvc",
        "darwin" => "apple-darwin",
        "macos" => "apple-darwin",
        _ => {
            return Err(format!("OS not supported: {}", env::consts::OS));
        }
    };

    let aarch_str = match env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        _ => {
            return Err(format!("Architecture not supported: {}", env::consts::ARCH));
        }
    };

    let ext_str = if env::consts::OS == "windows" {
        ".exe"
    } else {
        ""
    };

    Ok(
        PathBuf::from_str(format!("{aarch_str}-{os_str}-simple-http-server{ext_str}").as_str())
            .unwrap(),
    )
}

fn main() {
    let root_app_folder = env::current_exe()
        .expect("Unable to get path of the current executable")
        .parent()
        .unwrap()
        .to_path_buf();

    let server_exec = match get_server_executable_name() {
        Ok(exec) => exec,
        Err(err) => {
            eprintln!("{}", err);
            exit(1)
        }
    };

    let server_path = root_app_folder
        .join(PathBuf::from_str("Resources/simple-http-server").unwrap())
        .join(server_exec);

    let Some(data_folder) = FileDialog::new()
        .set_title("Select the folder containing the data")
        .set_directory("/Users/")
        .pick_folder()
    else {
        println!("{}", "You need to select a folder".red().bold());
        exit(1)
    };

    let absolute_igv_webapp_path = root_app_folder.join(IGV_WEBAPP_PATH);
    println!(
        "Executing command: {:?} {} {:?}",
        server_path, IGV_SERVER_ARGS, absolute_igv_webapp_path
    );
    let mut process_igv_server = Command::new(&server_path)
        .args(shlex::split(IGV_SERVER_ARGS).unwrap())
        .arg(absolute_igv_webapp_path)
        // .stdout(Stdio::piped())
        // .stderr(Stdio::piped())
        .spawn()
        .expect("Unable to start IGV server");

    println!(
        "Executing command: {:?} {} {:?}",
        &server_path, DATA_SERVER_ARGS, &data_folder,
    );
    let mut process_media_server = Command::new(&server_path)
        .args(shlex::split(DATA_SERVER_ARGS).unwrap())
        .arg(&data_folder)
        .spawn()
        .expect("Unable to start the server that makes the data available to igv");

    if webbrowser::open("http://127.0.0.1:8000/index.html").is_ok() {
        println!("Opening IGV");
    } else {
        eprintln!("Error in automatically opening IGV on the browser. Try going to http://127.0.0.1:8000/index.html")
    }

    loop {
        if process_igv_server
            .try_wait()
            .is_ok_and(|exit_status| exit_status.is_some())
        {
            process_media_server
                .kill()
                .expect("Couldn't kill the data server after IGV has exited");
            break;
        };

        if process_media_server
            .try_wait()
            .is_ok_and(|exit_status| exit_status.is_some())
        {
            process_igv_server
                .kill()
                .expect("Couldn't kill igv server after data server has exited");
            break;
        };

        thread::sleep(time::Duration::from_millis(1))
    }
}
