pub mod state;

pub mod egui {
   pub mod gui;
   pub mod gui_example;
}

pub mod inbuilt {
   pub mod setup;
   pub mod vertex_library;
   pub mod vertex_package;
   pub mod event_loop;
}

pub mod packages {
   pub mod time_package;
   pub mod camera_package;
   pub mod input_manager_package;
}

pub mod pipelines {
   pub mod test_render_pipeline;
}

pub mod bundles {
   pub mod automata {
      pub mod automata_package;
      pub mod automata_pipeline;
      pub mod automata_compute_pipeline;
      pub mod automata_queue_compute_pipeline;
      pub mod automata_bundle;
      pub mod automata_gui;
   }
}

pub mod utility {
   pub mod macros;
   pub mod functions;
   pub mod structs;
}
