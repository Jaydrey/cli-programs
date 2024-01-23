use crate::project::project_args::{
    ProjectCommand,
    ProjectSubCommand,
    CreateProject,
};
use std::{
    process::Command,
    fs::{self, File},
    os::unix::fs::PermissionsExt,
    process::exit,
};
use std::io::Write;


pub fn handle_project_command(project: ProjectCommand) -> Result<String, Box<dyn std::error::Error>> {
    let command = project.command;
    match command {
        ProjectSubCommand::Create(project) => {
            create_project(project);
        }
    }
    Ok("".to_string())
}

pub fn create_project(project: CreateProject) -> () {
    println!("Creating a Django Project {:?}", project);
    // check os type
    if cfg!(target_os = "windows") {
        create_windows_project(&project);
    }
    create_linux_project(&project);
}

pub fn create_windows_project(_project: &CreateProject) -> () {
    return;
}

pub fn create_linux_project(project: &CreateProject) -> () {
//     TODO deactivate any running virtual environments
//     check if python installed
    let is_windows = false;
    let python_installed = check_python_installed(is_windows);
    if python_installed == false {
        eprintln!("Install Python in your device to continue");
        return;
    }
    // python is installed
    // pip install virtualenv
    let virtualenv_installed = check_virtualenv_installed(is_windows);
    if virtualenv_installed == false {
        eprintln!("Installing virtualenv python package");
        let result = install_virtualenv(is_windows);
        if result == false {
            return;
        }
    }
    // create the project directory
    let result = create_project_directory(&project.name);
    if let Err(_error) = result {
        return;
    }
    // get in the folder
    if let Err(_error) = std::env::set_current_dir(&project.name) {
        eprintln!("Couldn't change directory to the project");
        return;
    }

    // create the venv folder
    let result = create_virtual_env(is_windows);
    if result == false {
        return;
    }

    // activate venv
    let result = activate_venv(is_windows);
    if result == false {
        return;
    }

    // add python packages
    let result = add_django_packages(is_windows);
    if result == false {
        return;
    }

    // write the packages to a requirements .txt file
    let result = create_requirements_file(is_windows);
    if result == false{
        return;
    }

    // create the django project with django-admin
    let _result = create_django_project(is_windows, &project.name);
    return;
}

fn remove_file(file_name: &str){
//     check if file exists
    if !fs::metadata(file_name).is_ok(){
        exit(1);
    }
    if let Err(error) = fs::remove_file(file_name){
        eprintln!("Failed to remove the file.\nError: {}", error);
        exit(1);
    }
}

fn create_django_project(is_windows: bool, project_name: &str) -> bool {
    if is_windows == false {
        let mut child = Command::new("django-admin")
            .args(["startproject", project_name, "."])
            .spawn()
            .ok();
        if let Some(mut result) = child {
            let output = result.wait().ok();
            if let Some(result) = output {
                if result.success() == true {
                    println!("{} has been created successfully", project_name);
                } else {
                    eprintln!("Something went wrong while creating the django project {}", project_name);
                }
                return result.success();
            }
            eprintln!("Something went wrong while creating the django project {}", project_name);
            return false;
        }
        eprintln!("Something went wrong while creating the django project {}", project_name);
        return false;
    }
    eprintln!("This command works on linux computers only");
    return false;
}

fn create_requirements_file(is_windows: bool) -> bool{
    if is_windows == false{
        if fs::metadata("requirements.txt").is_ok(){
            return true;
        }

        let mut requirements_file = File::create("requirement.txt");
        if let Err(error) = requirements_file{
            eprintln!("Something went wrong while creating requirements.txt file\
            \nError {:?}", error);
            return false;
        }

        let packages = vec![
          "django",
            "djangorestframework",
            "django-cors-headers",
            "drf-spectacular",
            "django-filter",
            "python-decouple",
            "djangorestframework-simplejwt",
        ];

        if let Ok(mut file) = requirements_file {
            for package in packages {
                if let Ok(result) = writeln!(file, "{}", package) {
                    println!("writing to requirements.txt file .");
                }
            }
        }

        return true;
    }
    eprintln!("This command works on linux computers only");
    return false;
}

fn add_django_packages(is_windows: bool) -> bool {
    if is_windows == false {
        let mut child = Command::new("pip")
            .args(["install", "django", "djangorestframework", "python-decouple", "django-cors-headers",
                "django-filter", "drf-spectacular", "djangorestframework-simplejwt"])
            .spawn()
            .ok();
        if let Some(mut result) = child {
            let output = result.wait().ok();
            if let Some(result) = output {
                if result.success() == true {

                    println!("Successfully installed ¸\nDjango, \nDjangoRestFramework, ¸\nPython Decouple, \nDjango Cors Headers,\
                     \nDjango Filters, \nDRF Spectacular and \nDRF Simple JWT");
                } else {
                    eprintln!("Something went wrong while installing django packages");
                }
                eprintln!("Something went wrong while installing django packages");
                return result.success();
            }
            eprintln!("Something went wrong while installing django packages");
            return false;
        }
        eprintln!("Something went wrong while installing django packages");
        return false;
    }
    eprintln!("This command works on linux computers only");
    return false;
}

fn activate_venv(is_windows: bool) -> bool {
    if is_windows == false {
        // change permission of the file to be accessible by anyone
        if let Err(err) = fs::set_permissions("venv/bin/activate", fs::Permissions::from_mode(0o755)) {
            eprintln!("Failed to set execute permission for activation script: {}", err);
            exit(1);
        }
        // create a bash script file
        let mut script_file = File::create("activate_venv.sh");
        if let Err(error) = script_file{
            eprintln!("Something went wrong when creating the activate_venv bash file\
            \n Error: {}", error);
            exit(1);
        }
        // write to the script
        let script_content = r#"source venv/bin/activate"#;
        if let Err(error) = script_file.unwrap().write_all(script_content.as_bytes()){
            eprintln!("Something went wrong while writing to the activate_venv bash file\
            \nError: {}", error);
            exit(1);
        }

        // make the script executable
        if let Err(error) = Command::new("chmod").args(&["+x", "activate_venv.sh"]).status(){
            eprintln!("Something went wrong while making the file executable\
            \nError: {}", error);
            exit(1);
        }

        // execute the file
        let mut child = Command::new("bash")
            .arg("activate_venv.sh")
            .spawn();
        if let Ok(mut result) = child {
            let output = result.wait();
            if let Ok(result) = output {
                if result.success() == true {
                    println!("Successfully activated the  virtual envs");
                } else {
                    eprintln!("Something went wrong while activating virtual envs");
                }
                remove_file("activate_venv.sh");
                return result.success();
            }
            eprintln!("Something went wrong while activating virtual envs\
        \nError: {:?}
        ", output.err());
            return false;
        }
        eprintln!("Something went wrong while activating virtual envs\
        \nError: {:?}
        ", child.err());
        return false;
    }
    eprintln!("This command works on linux computers only");
    return false;
}

fn create_virtual_env(is_windows: bool) -> bool {
    if !fs::metadata("venv").is_ok() {
        if is_windows == false {
            let mut child = Command::new("python3")
                .args(["-m", "virtualenv", "venv"])
                .spawn();
            if let Ok(mut result) = child {
                let output = result.wait();
                if let Ok(result) = output {
                    if result.success() == true {
                        println!("Successfully created virtual envs");
                    } else {
                        eprintln!("Something went wrong while creating virtual envs");
                    }
                    return result.success();
                }
                eprintln!("Something went wrong while creating virtual envs\
                \nError: {:?}
                ", output.err());
                return false;
            }
            eprintln!("Something went wrong while creating virtual envs\
            \nError: {:?}", child.err());
            return false;
        }
        return false;
    }
    println!("Virtual environment folder exists");
    return false;
}

fn create_project_directory(project_name: &str) -> Result<(), String> {
    if !fs::metadata(project_name).is_ok() {
        if let Err(err) = fs::create_dir(project_name) {
            println!("Failed to created django project {}", err);
            return Err(format!("Failed to create django project {}\n Error: {}", project_name, err));
        }
        println!("Project folder {} created successfully", project_name);
        return Ok(());
    }
    println!("Project folder {} already exists", project_name);
    return Ok(());
}


fn install_virtualenv(is_windows: bool) -> bool {
    if is_windows == false {
        let mut child = Command::new("python3")
            .args(["-m", "pip", "install", "virtualenv"])
            .spawn()
            .ok();
        if let Some(mut result) = child {
            let status = result.wait().ok();
            if let Some(result) = status {
                if result.success() == true {
                    println!("Virtualenv installed successfully");
                    return true;
                }
            }
            eprintln!("Something went wrong while installing virtualenv");
            return false;
        }

        eprintln!("Something went wrong while installing virtualenv");
        return false;
    }
    eprintln!("This command works on linux computers only");
    return false;
}

fn check_virtualenv_installed(is_windows: bool) -> bool {
    if is_windows == false {
        let mut child = Command::new("sh")
            .args(["-c", "command", "-v", "virtualenv"])
            .spawn()
            .ok();

        if let Some(mut result) = child {
            let status = result.wait().ok();
            if let Some(result) = status {
                return result.success();
            }
        }
        return false;
    }
    eprintln!("This command works on linux computers only");
    return false;
}

fn check_python_installed(is_windows: bool) -> bool {
    if is_windows == false {
        let mut child = Command::new("which")
            .arg("python3")
            .spawn()
            .ok();

        if let Some(mut result) = child {
            let status = result.wait().ok();
            if let Some(result) = status {
                return result.success();
            }
            return false;
        }
        eprintln!("Something went wrong");
        return false;
    }
    eprintln!("This command works on linux computers only");
    return false;
}