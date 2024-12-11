//
//  Author: Richard I. Christopher, NeoTec Digital.
//  Date : 2024-05-05
//  Description: Combining Audio Graphics, Midi, Music Theory, Analysis and Synthesis.
//  License: Proprietary - All Rights Reserved, Big Stick Studio - The NEXUS Project.
//  Version: 0.1.0
//  Status: In Development
//  

// #![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
// TODO: Remove These
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::{ops::Deref, thread::sleep};

use rodio::Sink;
use tokio::stream;

fn main() {
    use std::fs::File;
    use std::sync::{Arc, Mutex};
    use tokio::time::{self, sleep, Duration};
    use rodio::{OutputStream, Source, dynamic_mixer};
    use audiotheorem::{runtime::{Events, Engine, Sequence, Waveform}, types::Tuning};

    const GRID_SIZE: u8 = 12;

    // Multi-threaded Runtime
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Midi Sequence Buffer
    let gfx_read_theorem: Arc<Mutex<Sequence>> = Arc::new(Mutex::new(Sequence::new()));
    let audio_read_theorem: Arc<Mutex<Sequence>> = Arc::new(Mutex::new(Sequence::new()));

    // Clone the write_theorem for the audio and graphics loops
    let gfx_write = gfx_read_theorem.clone();
    let audio_write = audio_read_theorem.clone();

    //////////
    // MIDI //
    //////////

    // Midi Loop = // Used as a buffer to store the midi events for the graphics and audio loop
    rt.spawn(async move { Events::read_midi(move |index, velocity| { 
        gfx_write.lock().unwrap().process_input(index, velocity); 
        audio_write.lock().unwrap().process_input(index, velocity); 
    })});



    ///////////
    // AUDIO //
    ///////////

    // Audio Loop
    rt.spawn(async move {
        // Audio Settings
        let wave_table_size = 1440;     // 120 samples per octave - 10 samples per pitchclass
        let sample_rate = 44100;

        let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
        for i in 0..wave_table_size { wave_table.push((i as f32 / wave_table_size as f32 * 2.0 * std::f32::consts::PI).sin()); }

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        loop {
            // get our midi data
            let read_theorem = audio_read_theorem.lock().unwrap();
            let size = read_theorem.get_size();
            
            // clear the sink
            sink.clear();

            // if we have no data, sleep for a bit
            if size <= 0 { let _ = sleep(Duration::from_millis(10)); continue; } 

            // create a new mixer
            let (controller, mixer) = dynamic_mixer::mixer::<f32>(2, sample_rate);

            // get all the tones and add them to the mixer, and throw them into the sink
            for tone in read_theorem.tones() {
                let mut oscillator = Waveform::new(sample_rate, wave_table.clone());
                oscillator.set_frequency(tone.pitch().unwrap().frequency(Tuning::A4_440Hz));
                controller.add(oscillator.convert_samples());
            }

            // play the sink
            sink.append(mixer);
            sink.sleep_until_end();
        }
    });


    //////////////
    // GRAPHICS //
    //////////////

    // Graphics Loop
    rt.block_on(async move {
        use winit::event::{Event, WindowEvent, ElementState, KeyboardInput, VirtualKeyCode};
        use winit::event_loop::{ControlFlow, EventLoop};
        use winit::window::WindowBuilder;
        use audiotheorem::runtime::TexturedSquare;
    
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let mut gfx = Engine::new(window, GRID_SIZE.into(), &TexturedSquare::new()).await;
        let mut last_sequence_size = gfx_read_theorem.lock().unwrap().get_size();

        event_loop.run(move |event, _, control_flow| 
            match event {

                Event::WindowEvent 
                    {    // Handle Window Events
                        ref event,
                        window_id,
                    } 
                    if window_id == gfx.window.id() => 
                        if !gfx.input(event) 
                            {
                                match event {
                                    WindowEvent::CloseRequested
                                    | WindowEvent::KeyboardInput {
                                        input: 
                                            KeyboardInput {
                                                state: ElementState::Pressed,
                                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                                ..
                                            },
                                        ..
                                    } => *control_flow = ControlFlow::Exit,
                                    WindowEvent::Resized(physical_size) => {
                                        gfx.resize(*physical_size);
                                    },
                                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                        gfx.resize(**new_inner_size);
                                    },
                                    _ => {


                                    }
                                }
                            },

                Event::RedrawRequested(window_id) 
                    if window_id == gfx.window.id() => 
                        {          // Redraw the window
                            gfx.update();
                            match gfx.render() 
                                {
                                    Ok(_) => {},
                                    Err(wgpu::SurfaceError::Lost) => gfx.resize(gfx.size),
                                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                                    Err(e) => eprintln!("{:?}", e),
                                }
                        },

                Event::MainEventsCleared => { gfx.window.request_redraw(); }, // Request the redraw

                _ => { // On any other event we want to update our state
                    // TODO: Use a channel here instead of a mutex/arc
                    let read_sequence = Arc::clone(&gfx_read_theorem); // This is most likely overkill - but I think I needed it..  (if you can't see this my screen is bigger than yours)
                    let t_read = read_sequence.lock().unwrap();
                    let size = t_read.get_size();

                    if size != last_sequence_size {
                        last_sequence_size = size;
                        t_read.print_state();
                        gfx.refresh_instances();

                        // If we don't have anything we want to clear the buffer and exit
                        if size <= 0 { 
                            gfx.update_instance_buffer();
                            return; 
                        }

                        // TODO: Need to integrate caching to only update the changed notes

                        gfx.enable_tones(t_read.tones());

                        gfx.update_instance_buffer();
                    }
                }
            }
        );
    });



}