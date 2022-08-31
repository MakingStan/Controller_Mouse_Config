use std::fs;
use std::fs::File;
use std::path::Path;
use egui::{TextBuffer, Vec2};

fn make_controller_mouse_directory_and_config_file(mut controller_mouse_path: String)
{
    let dir_path = Path::new(&controller_mouse_path);

    //check if the directory even exists, if not make it.
    if !dir_path.is_dir()
    {
        fs::create_dir(&dir_path).unwrap();
    }

    let config_file_path: String = controller_mouse_path+"/config.txt";

    match File::open(config_file_path.clone()) {
        Ok(ref _file) => { },
        Err(error) => {
            //the config file doesn't exist make it.
            File::create(config_file_path.clone()).expect("Could not create config file.");
            fs::write(config_file_path.clone(), "20").expect("Could not write initial config file settings.");
        },
    };
}

struct ControllerMouseConfig {
    sensivity: u32,
    pid: String,
    vid: String,
}

impl Default for ControllerMouseConfig {
    fn default() -> Self {
        Self {
            sensivity: 82,
            pid: get_config(1),
            vid: "0x054c".to_string(),
        }
    }
}

fn get_config(index: usize) -> String
{

    return fs::read_to_string(home::home_dir().unwrap().to_str().unwrap().to_owned() + "/.controller_mouse/config.txt")
        .unwrap()
        .as_str()
        .split("\n")
        .collect::<Vec<&str>>()
        .get(index)
        .unwrap()
        .to_string();
}


impl eframe::App for ControllerMouseConfig {


    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mouse Controller Config");
            ui.add(egui::Slider::new(&mut self.sensivity, 2..=100).text("sensivity"));

            ui.heading("Pid (Product id)");
            ui.text_edit_singleline(&mut self.pid);
            ui.heading("Vid (Vendor id)");
            ui.text_edit_singleline(&mut self.vid);


            ui.heading("\nIf you don't know your vid or/and pid follow the guide at the github page: ");
            ui.text_edit_singleline(&mut "https://github.com/makingstan/controller_mouse/blob/master/README.md");


            if ui.button("Update Data").clicked() {
                fs::write(home::home_dir().unwrap().to_str().unwrap().to_owned() + "/.controller_mouse/config.txt", (102-self.sensivity).to_string()+"\n".as_str()+self.vid.as_str()+"\n".as_str()+self.pid.as_str()).expect("Could not write to config.txt");
            }

        });
    }
}


fn main() {
    make_controller_mouse_directory_and_config_file(home::home_dir().unwrap().to_str().unwrap().to_owned() + "/.controller_mouse");
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Mouse Controller Config",
        options,
        Box::new(|_cc| Box::new(ControllerMouseConfig::default())),
    );

}