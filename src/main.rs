extern crate gtk;
extern crate gio;
extern crate gtk_sys;
extern crate cpufreq;

use gio::ApplicationExt;
use std::env;
use std::string::String;
use gtk::WidgetExt;
use gtk::BoxExt;
use gtk::ComboBoxExt;

mod result;
mod error;

const APPLICATION_WINDOW_SRC: &'static str = include_str!("application-window.xml");


pub struct FantasticChainsaw {
    application: gtk::Application
}

impl FantasticChainsaw {

    pub fn new() -> FantasticChainsaw {
        let application = gtk::Application::new(Some("local.fantastic-chainsaw"), gio::APPLICATION_FLAGS_NONE).unwrap();
        let chainsaw = FantasticChainsaw { application: application };

        chainsaw.application.connect_activate(FantasticChainsaw::activate);

        return chainsaw;
    }

    pub fn activate(application: &gtk::Application) {
        let builder = gtk::Builder::new_from_string(APPLICATION_WINDOW_SRC);

        let window = builder.get_object::<gtk::ApplicationWindow>("application-window").unwrap();

        let application_box = builder.get_object::<gtk::FlowBox>("flow-box-cpus").unwrap();

        for cpu_id in 0.. {
            if cpufreq::Cpu::exists(cpu_id) {
                let cpu_widget = FantasticChainsaw::get_cpu_widget(0).unwrap();
                application_box.insert(&cpu_widget, 0);
            } else {
                break;
            }
        }

        application.add_window(&window);

        window.show_all();
    }

    pub fn get_cpu_widget(cpu_id: cpufreq::CpuId) -> ::result::Result<gtk::Grid> {
        let grid = gtk::Grid::new();
        grid.insert_row(0);
        grid.insert_row(1);

        grid.insert_column(0);
        grid.insert_column(1);

        let cpu = cpufreq::Cpu::new(cpu_id);

        let driver_label = gtk::Label::new(Some("Driver"));
        grid.attach(&driver_label, 0, 0, 1, 1);

        let driver_combo = gtk::ComboBoxText::new();
        for governor in try!(cpu.get_available_governors()) {
            driver_combo.append(Some(&governor), &governor);
        }

        let current_policy = try!(cpu.get_policy());
        print!("Current driver: {}", current_policy.governor);

        assert!(driver_combo.set_active_id(Some(&current_policy.governor)));

        grid.attach(&driver_combo, 1, 0, 1, 1);

        let cpu_frequency_label = gtk::Label::new(Some("Cpu frequency"));
        grid.attach(&cpu_frequency_label, 0, 1, 1, 1);

        let cpu_frequency_combo = gtk::ComboBoxText::new();
        for frequency in try!(cpu.get_available_frequencies()) {
            let freq_id = frequency.to_string();
            let freq_text = format!("{} MHz", frequency / 1000);
            cpu_frequency_combo.append(Some(&freq_id), &freq_text);
        }

        let current_frequency = try!(cpu.get_freq());
        cpu_frequency_combo.set_active_id(Some(&current_frequency.to_string()));

        grid.attach(&cpu_frequency_combo, 1, 1, 1, 1);

        return Ok(grid);
    }
}


fn main() {
    let application = FantasticChainsaw::new();

    let env_args: Vec<String> = env::args().collect();

    let args: Vec<&str> = env_args.iter().map(|s| &**s).collect();

    application.application.run(args.iter().count() as i32, args.as_slice());
}
