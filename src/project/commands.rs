use crate::project::project_args::{
    ProjectCommand,
    ProjectSubCommand,
    CreateProject,
};
use std::{
    env,
    io::Write,
    process::Command,
    fs::{self, File},
    os::unix::fs::PermissionsExt,
    // process::exit,
    path::Path,
};



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

    // copy dockerfile to the project directory
    add_dockerfile(&project.name);
    // copy requirements.txt to project directory
    add_requirements_txt(&project.name);

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

    // create the django project with django-admin
    let _result = create_django_project(is_windows, &project.name);
    if result == false {
        return;
    }

    // add settings file
    add_settings_py_file(&project.name);
}

fn remove_file<P: AsRef<Path>>(file_name: &P) -> bool{
//     check if file exists
    if !fs::metadata(file_name).is_ok(){
        // exit(1);
        return true;
    }
    if let Err(error) = fs::remove_file(file_name){
        eprintln!("Failed to remove the file.\nError: {}", error);
        // exit(1);
        return false;
    }
    return true;
}

fn add_settings_py_file(project_name: &str) ->(){
    // remove default settings py file
    let project_dir = env::current_dir();
    if let Err(_error) = project_dir{
        eprintln!("Couldn't get the cwd");
        return;
    }
    let settings_file = project_dir.unwrap().join(project_name).join("settings.py");
    if remove_file(&settings_file) == false{
        return;
    }

    let django_settings_file = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("settings.py");
    let project_dir = env::current_dir();
    let destination_dir = project_dir.unwrap().join(project_name).join("settings.py");

    if let Err(_error) = fs::copy(django_settings_file, destination_dir){
        eprintln!("Couldn't copy the django settings file to the project");
    }
}

fn create_django_project(is_windows: bool, project_name: &str) -> bool {
    if is_windows == false {
        let child = Command::new("django-admin")
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

fn add_django_packages(is_windows: bool) -> bool {
    if is_windows == false {
        let child = Command::new("pip")
            .args(["install", "-r", "requirements.txt"])
            .spawn()
            .ok();
        if let Some(mut result) = child {
            let output = result.wait().ok();
            if let Some(result) = output {
                if result.success() == true {
                    return result.success();
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
            // exit(1);
            return true;
        }
        // create a bash script file
        let script_file = File::create("activate_venv.sh");
        if let Err(error) = script_file{
            eprintln!("Something went wrong when creating the activate_venv bash file\
            \n Error: {}", error);
            // exit(1);
            return true;
        }
        // write to the script
        let script_content = r#"source venv/bin/activate"#;
        if let Err(error) = script_file.unwrap().write_all(script_content.as_bytes()){
            eprintln!("Something went wrong while writing to the activate_venv bash file\
            \nError: {}", error);
            // exit(1);
            return true;
        }

        // make the script executable
        if let Err(error) = Command::new("chmod").args(&["+x", "activate_venv.sh"]).status(){
            eprintln!("Something went wrong while making the file executable\
            \nError: {}", error);
            // exit(1);
            return true;
        }

        // execute the file
        let child = Command::new("bash")
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
            let child = Command::new("python3")
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

fn add_requirements_txt(project_name: &str) -> (){
    let docker_file = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("requirements.txt");
    let result = env::current_dir();

    if let Err(_error) = result{
        return;
    }
    let destination_dir = result.unwrap().join(project_name).join("requirements.txt");

    // copy file
    if let Err(_error) = fs::copy(docker_file, destination_dir){
        eprintln!("Failed to copy the requirements.txt to the project");
        return;
    }
    println!("Successfully copied the requirements.txt");
    return;
}

fn add_dockerfile(project_name: &str) -> () {
    let docker_file = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("Dockerfile");
    let docker_compose_file = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("docker-compose.yml");
    let result = env::current_dir();

    if let Err(_error) = result{
        // exit(1);
        return;
    }

    // copy dockerfile
    let destination_dir = result.unwrap().join(project_name).join("Dockerfile");
    if let Err(_error) = fs::copy(docker_file, destination_dir){
        eprintln!("Failed to copy the Dockerfile to the project");
        // exit(1);
        return;
    }

    // copy docker compose
    let result = env::current_dir();
    let destination_dir = result.unwrap().join(project_name).join("docker-compose.yml");
    if let Err(_error) = fs::copy(docker_compose_file, destination_dir){
        eprintln!("Failed to copy the docker-compose.yml file to the project");
        // exit(1);
        return;
    }
    println!("Successfully copied the Dockerfile and docker-compose.yml");
    return;
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
        let child = Command::new("python3")
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
        let child = Command::new("sh")
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
        let child = Command::new("which")
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