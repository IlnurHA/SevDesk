use crate::command_handler::CommandHandler;
use crate::read_line;
use rdev::{listen, Event, EventType, Key};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Copy)]
pub struct KeyBoardState {
    is_left_ctrl_pressed: bool,
    is_left_shift_pressed: bool,
    is_left_alt_pressed: bool,
}

impl KeyBoardState {
    pub fn new() -> Self {
        Self {
            is_left_ctrl_pressed: false,
            is_left_shift_pressed: false,
            is_left_alt_pressed: false,
        }
    }
}

pub struct AppManager {}

impl AppManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_parameters(
        &mut self,
        parameters: Option<String>,
        command_handler: &mut CommandHandler,
    ) {
        if parameters.is_none() {
            return;
        }

        println!("Executing prescribed commands");

        for command in parameters.unwrap().split(";").collect::<Vec<_>>() {
            if let Err(message) = command_handler.parse_command(command.to_string()) {
                println!(
                    "Cannot execute prescribed command: \n\t{}\n{}",
                    command, message
                );
            }
        }
    }

    pub fn start_program(
        &mut self,
        command_handler: &mut CommandHandler,
        keyboard_state: KeyBoardState,
    ) {
        let command_queue = Arc::new(Mutex::new(vec![]));
        let keyboard_arc = command_queue.clone();
        let console_arc = command_queue.clone();
        let flush_arc = command_queue.clone();

        let keyboard = Arc::new(Mutex::new(keyboard_state));

        thread::spawn(move || {
            if let Err(_) = listen(move |event: Event| {
                AppManager::callback(keyboard_arc.clone(), keyboard.clone(), event)
            }) {
                println!("Cannot listen keyboard");
            }
        });

        thread::spawn(move || loop {
            let command = read_line();
            console_arc.lock().unwrap().push(command);
        });

        loop {
            let mut data = flush_arc.lock().unwrap();
            let mut copied_data = vec![];
            if !data.is_empty() {
                copied_data = data.clone();
                data.clear();
            }
            drop(data);

            for command in copied_data {
                if let Err(message) = command_handler.parse_command(command.clone()) {
                    println!(
                        "Cannot execute following command\n\t'{}'\n{}",
                        command, message
                    );
                } else {
                    println!("Command successfully handled!");
                }
            }
        }
    }

    pub fn callback(
        data: Arc<Mutex<Vec<String>>>,
        keyboard_state: Arc<Mutex<KeyBoardState>>,
        event: Event,
    ) {
        match event.event_type {
            EventType::KeyPress(Key::ControlLeft) => {
                keyboard_state.lock().unwrap().is_left_ctrl_pressed = true;
            }
            EventType::KeyRelease(Key::ControlLeft) => {
                keyboard_state.lock().unwrap().is_left_ctrl_pressed = false;
            }
            EventType::KeyPress(Key::ShiftLeft) => {
                keyboard_state.lock().unwrap().is_left_shift_pressed = true;
            }
            EventType::KeyRelease(Key::ShiftLeft) => {
                keyboard_state.lock().unwrap().is_left_shift_pressed = false;
            }
            EventType::KeyPress(Key::Alt) => {
                keyboard_state.lock().unwrap().is_left_alt_pressed = true;
            }
            EventType::KeyRelease(Key::Alt) => {
                keyboard_state.lock().unwrap().is_left_alt_pressed = false;
            }
            EventType::KeyPress(key) => {
                let key_string = AppManager::key_to_string(key);
                let keyboard = keyboard_state.lock().unwrap();
                if key_string != ""
                    && keyboard.is_left_ctrl_pressed
                    && keyboard.is_left_shift_pressed
                    && keyboard.is_left_alt_pressed
                {
                    data.lock().unwrap().push(key_string);
                }
            }
            _ => {}
        }
    }

    fn key_to_string(key: Key) -> String {
        match key {
            Key::Num0 => "0".to_string(),
            Key::Num1 => "1".to_string(),
            Key::Num2 => "2".to_string(),
            Key::Num3 => "3".to_string(),
            Key::Num4 => "4".to_string(),
            Key::Num5 => "5".to_string(),
            Key::Num6 => "6".to_string(),
            Key::Num7 => "7".to_string(),
            Key::Num8 => "8".to_string(),
            Key::Num9 => "9".to_string(),
            _ => "".to_string(),
        }
    }
}
