use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::fs::File;
use std::time::Duration;
use rdev::{listen, EventType};

use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc, widget::Label, Color, AppDelegate, DelegateCtx, Event, commands, WindowId, WindowHandle, Env, Target, Command, Handled, TimerToken, EventCtx};
use druid::commands::CLOSE_WINDOW;
use druid::widget::{Button, Flex, Padding};

#[derive(Clone, Data)]
struct AppState {
    label_string: String
}
struct MyDelegate;

struct TimerWidget {
    timer_token: Option<TimerToken>, // Per gestire il timer
    target_window: Option<WindowId>, // Per identificare la finestra
}

impl TimerWidget {
    fn new() -> Self {
        TimerWidget {
            timer_token: None,
            target_window: None,
        }
    }
}

impl Widget<AppState> for TimerWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut AppState, _env: &Env) {
        match event {
            Event::WindowConnected => {
                println!("Finestra connessa: avvio il timer.");
                // Avvia il timer di 2 secondi
                self.target_window = Some(ctx.window_id());
                self.timer_token = Some(ctx.request_timer(Duration::from_secs(5)));
            }
            Event::Timer(token) => {
                if let Some(timer) = self.timer_token {
                    if token == &timer {
                        if let Some(window_id) = self.target_window {
                            println!("Timer scaduto. Chiusura finestra con ID: {:?}", window_id);
                            ctx.submit_command(commands::CLOSE_ALL_WINDOWS); //così le chiude tutte
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
    }

    fn update(
        &mut self,
        _ctx: &mut druid::UpdateCtx,
        _old_data: &AppState,
        _data: &AppState,
        _env: &Env,
    ) {
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> druid::Size {
        bc.max()
    }

    fn paint(&mut self, _ctx: &mut druid::PaintCtx, _data: &AppState, _env: &Env) {}
}


impl AppDelegate<AppState> for MyDelegate {
    fn window_added(&mut self, id: WindowId, handle: WindowHandle, data: &mut AppState, env: &Env, ctx: &mut DelegateCtx) {
     //creazione della finestra e inizializzazione
    }



    fn event(
        &mut self,
        _ctx: &mut DelegateCtx,
        _window_id: druid::WindowId,
        event: druid::Event,
        data: &mut AppState,
        _env: &druid::Env,
    ) -> Option<Event> {
        match event.clone() {
            // Gestisce l'evento della finestra creata
            Event::WindowConnected => {

            }
            Event::Timer(e) => {


                _ctx.submit_command(CLOSE_WINDOW.to(Target::Window(_window_id)));
            }

            _ => {}
        }
        Some(event)
    }
}

fn build_ui_with_timer() -> impl Widget<AppState> {
    // Crea il widget con il timer
    let timer_widget = TimerWidget::new();

    // Crea l'etichetta con il testo centrato
    let label_widget = Label::new(
        "Rettangolo rilevato. Disegnare il segno meno\nper procedere con l'operazione di backup.\n",
    )
        .with_text_size(15.0)
        .with_text_color(Color::BLACK)
        .center()
        .fix_height(200.0)
        .background(Color::WHITE);

    //label_widget

    // Combina il `TimerWidget` e il `label_widget` in un layout verticale
    Flex::column().with_child(label_widget)// Aggiungi l'etichetta
        .with_child(timer_widget) // Aggiungi il TimerWidget

}


pub fn listener(screen_width: f64, screen_height: f64) {
    let state = Arc::new(Mutex::new(1));
    let cvar = Arc::new(Condvar::new());

    let tolerance = 100.0;

    // Avvia il thread per il mouse listener
    thread::spawn({
        let state = Arc::clone(&state);
        let cvar = Arc::clone(&cvar);
        move || {
            let mut points = Vec::new();
            let mut tracking_active = false;

            if let Err(error) = listen(move |event| {
                match event.event_type {
                    EventType::ButtonPress(button) if button == rdev::Button::Left => {
                        tracking_active = true;
                        points.clear();
                    }
                    EventType::ButtonRelease(button) if button == rdev::Button::Left => {
                        tracking_active = false;
                        let current_state = *state.lock().unwrap();

                        if current_state == 1 && check_for_rectangle(&points, screen_width, screen_height, tolerance) {
                            points.clear();
                            println!("current_ state {:?}", current_state);
                            let mut state = state.lock().unwrap();
                            *state = 2;

                            cvar.notify_all(); // Notifica gli altri thread che la condizione è stata soddisfatta
                        } else if current_state == 2 && check_minus_sign(&points, screen_width, tolerance) {
                            println!("current_ state {:?}", current_state);
                            println!("Segno meno rilevato!!");
                            let mut state = state.lock().unwrap();
                            *state = 3;
                            cvar.notify_all(); // Notifica gli altri thread che la condizione è stata soddisfatta
                            println!("current_ state {:?}", state);
                        }
                    }
                    EventType::MouseMove { x, y } if tracking_active => {
                        points.push((x, y));
                    }
                    _ => {}
                }
            }) {
                eprintln!("Errore nel listener mouse: {:?}", error);
            }
        }
    });

    {
        let mut state = state.lock().unwrap();
        while *state != 2 {
            state = cvar.wait(state).unwrap();
            println!("current_ state {:?}", state);
        }
    }

    let app_state = Arc::clone(&state);
    let app_cvar = Arc::clone(&cvar);
    // Crea la finestra con un titolo e dimensioni specifiche
    let main_window = WindowDesc::new(build_ui_with_timer()) //nuova build ui   \
        .title("Backup operation")
        .window_size((500.0, 200.0)); // Imposta la dimensione della finestra

    let initial_state = AppState {
        label_string: "Rettangolo rilevato. Disegnare il segno meno\nper procedere con l'operazione di backup.\nChiudere la finestra corrente per procedere.".to_string(),
    };

    let launcher = AppLauncher::with_window(main_window).delegate(MyDelegate)
        .launch(initial_state)
        .expect("Failed to launch application");


    { //aspettando di rilevare il segno meno
        let mut state = state.lock().unwrap();
        while *state != 3 {
            state = cvar.wait(state).unwrap();
            println!("current_ state {:?}", state);
        }
    }

}
// Funzione per rilevare un rettangolo
fn check_for_rectangle(points: &[(f64, f64)], screen_width: f64, screen_height: f64, tolerance: f64) -> bool {
    if points.len() < 4 {
        return false;
    }

    let mut edges_detected = [false; 4]; // Bordo superiore, inferiore, sinistro, destro

    for &(x, y) in points {
        if y >= -tolerance && y <= tolerance {
            edges_detected[0] = true; // Bordo superiore
        } else if y >= screen_height - tolerance && y <= screen_height + tolerance {
            edges_detected[1] = true; // Bordo inferiore
        } else if x >= -tolerance && x <= tolerance {
            edges_detected[2] = true; // Bordo sinistro
        } else if x >= screen_width - tolerance && x <= screen_width + tolerance {
            edges_detected[3] = true; // Bordo destro
        }
    }

    edges_detected.iter().all(|&detected| detected)
}

// Funzione per rilevare un segno meno
fn check_minus_sign(points: &[(f64, f64)], screen_width: f64, tolerance: f64) -> bool {
    if points.len() < 2 {
        return false;
    }

    let mut edges_detected = [false; 2]; // Bordo sinistro e destro

    for &(x, _) in points {
        if x >= -tolerance && x <= tolerance {
            edges_detected[0] = true;
        } else if x >= screen_width - tolerance && x <= screen_width + tolerance {
            edges_detected[1] = true;
        }
    }

    edges_detected.iter().all(|&detected| detected)
}
