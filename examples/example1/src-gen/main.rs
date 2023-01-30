// Import types from the simulation library.
use tudelft_xray_sim::*;
// Import enum variants to make this example a bit easier to read.
use Dose::*;
use Mode::*;
use Projection::*;

use log::info;

fn main() {
    // Initialize logger.
    simple_logger::init().unwrap();
    // Run simulation with your own implementation of the control logic.
    run_single_plane_sim(Logic::default())
}

/// Example control logic for a two plane system.
/// The pedal mapping is based on the example mapping given in the DSL assignment.
#[derive(Default)]
struct Logic {
    /// keep track of the selected projection
    selected: Projection,
    // you can have whatever other information that you want here
    current_dose: Dose,
    current_mode: Mode,
    current_status: bool,
    current_magic_value: u8,
    current_activated_number: u8,

    last_dose: Dose,
    last_mode: Mode,
}

impl PedalMapper for Logic {
    type Pedals = ThreePedals;


    fn on_press(&self, pedal: Self::Pedals) -> Option<Request> {
        use ThreePedals::*;
        Some(match pedal {
            Pedal1 => Request::start(Frontal,Low,Video),
            Pedal2 => Request::start(Frontal,High,Video),
            Pedal3 => Request::start(Frontal,Low,Image),
        })
    }

    fn on_release(&self, pedal: Self::Pedals) -> Option<Request> {
        use ThreePedals::*;
        Some(match pedal {
            Pedal1 => Request::stop(Frontal,Low,Video),
            Pedal2 => Request::stop(Frontal,High,Video),
            Pedal3 => Request::stop(Frontal,Low,Image),
        })
    }
}

impl ActionLogic<false> for Logic {
    /// Naive implementation of request handling which does not handle
    /// multiple pedals being pressed at once.
    fn handle_request(&mut self, request: Request, controller: &mut Controller<false>) {
        // This is how you can get access to the planes in case you want to inspect their status.
        let _frontal = controller.frontal();
        self.current_dose = _frontal.dose();
        self.current_mode = _frontal.mode();
        self.current_status = _frontal.active();

        // Custom logging of requests.
        info!("Processing request: {request:?}");

        // In your own code (as well as the code that you generate),
        // you should do checks before using the controller to
        // start and stop X-rays.
        match request {
            Request::Start {
                projection,
                dose,
                mode,
            } => {
                if self.current_status == true && self.current_magic_value != 0{
                    if self.current_dose == High {
                        // do not change, save request to last request
                        self.last_dose = dose;
                        self.last_mode = mode;
                        if dose == Low {
                            if mode == Image {
                                self.current_magic_value += 1;
                                self.current_activated_number += 1;
                            }else {
                                self.current_magic_value += 2;
                                self.current_activated_number += 1;
                            }
                        }else {
                            self.current_magic_value += 3;
                            self.current_activated_number += 1;
                        }
                    }else {
                        self.last_dose = self.current_dose;
                        self.last_mode = self.current_mode;
                        if dose == Low {
                            if mode == Image {
                                self.current_magic_value += 1;
                                self.current_activated_number += 1;
                            }else {
                                self.current_magic_value += 2;
                                self.current_activated_number += 1;
                            }
                        }else {
                            self.current_magic_value += 3;
                            self.current_activated_number += 1;
                        }
                        controller.deactivate_xray();
                        controller.activate_frontal(dose, mode);
                    }
                }else{
                    self.current_magic_value = 0;
                    self.current_activated_number = 0;
                    if dose == Low {
                        if mode == Image {
                            self.current_magic_value += 1;
                            self.current_activated_number += 1;
                        }else {
                            self.current_magic_value += 2;
                            self.current_activated_number += 1;
                        }
                    }else {
                        self.current_magic_value += 3;
                        self.current_activated_number += 1;
                    }
                    controller.activate_frontal(dose, mode);
                }
            },

            Request::Stop {
                projection,
                dose,
                mode,
            } => {
                if self.current_magic_value == 3{
                    if dose == Low { // this means the combination is Low Image + Low Video
                        if mode == Video {
                            self.current_magic_value -= 2;
                            self.last_dose = Low;
                            self.last_mode = Image;
                            controller.deactivate_xray();
                            controller.activate_frontal(self.last_dose, self.last_mode);
                        }else {
                            self.current_magic_value -= 1;
                            self.last_dose = Low;
                            self.last_mode = Video;
                            controller.deactivate_xray();
                            controller.activate_frontal(self.last_dose, self.last_mode);
                        }
                    }else { // this means that only High Video is activated
                        controller.deactivate_xray();
                        self.current_activated_number = 0;
                        self.current_magic_value = 0;
                    }
                } else if self.current_magic_value == 4 { // this means: Low Image + High video
                    if dose == Low {
                        self.current_magic_value -= 1;
                    }else {
                        self.current_magic_value -= 3;
                        self.last_dose = Low;
                        self.last_mode = Image;
                        controller.deactivate_xray();
                        controller.activate_frontal(self.last_dose, self.last_mode);
                    }
                } else if self.current_magic_value == 5 { // this means: Low Video + high Video
                    if dose == Low{
                        self.current_magic_value -= 2;
                    }else {
                        self.current_magic_value -= 3;
                        self.last_dose = Low;
                        self.last_mode = Video;
                        controller.deactivate_xray();
                        controller.activate_frontal(self.last_dose, self.last_mode);
                    }
                } else if self.current_magic_value == 6 { // all activated
                    if dose == Low{
                        if mode == Image{
                            self.current_magic_value -= 1;
                            self.last_dose = Low;
                            self.last_mode = Video;
                        }else {
                            self.current_magic_value -= 2;
                            self.last_dose = Low;
                            self.last_mode = Image;
                        }
                    }else {
                        self.current_magic_value -= 3;
                        controller.deactivate_xray();
                        controller.activate_frontal(self.last_dose, self.last_mode);
                        if self.last_mode == Video {
                            self.last_mode = Image;
                            self.last_dose = Low;
                        }
                    }
                }else {
                    controller.deactivate_xray();
                    self.current_activated_number = 0;
                    self.current_magic_value = 0;
                }
                if self.current_magic_value == 0 || self.current_activated_number == 0 {
                    controller.deactivate_xray();
                }
            },

            Request::ToggleSelectedProjection => {},

            Request::StartSelectedProjection { .. } => {},

            Request::StopSelectedProjection { .. } => {},
        }
    }
}
