use bevy::app::App;
use bevy::core_pipeline::core_3d::graph::Core3d;
use bevy::prelude::Plugin;
use bevy::render::render_graph::RenderGraphApp;
use bevy::render::RenderApp;
use crate::pipeline::cellular_automata::CellularAutomataLabel;
use crate::pipeline::colour::ColourLabel;
use crate::pipeline::drawing::DrawingLabel;

pub mod cellular_automata;
pub mod drawing;
pub mod colour;

pub struct PipelinesPlugin;
impl Plugin for PipelinesPlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_plugins((
                drawing::DrawingPipelinePlugin,
                cellular_automata::CellularAutomataPipelinePlugin,
                colour::ColourPipelinePlugin));

        render_app
            .add_render_graph_node::<cellular_automata::CellularAutomataNode>(Core3d, CellularAutomataLabel)
            .add_render_graph_node::<drawing::DrawingNode>(Core3d, DrawingLabel)
            .add_render_graph_node::<colour::ColourNode>(Core3d, ColourLabel);

        // User Drawing Input -> Cellular Automata Simulation -> Cell Rendering (i.e. colour output) -> Camera
        render_app.add_render_graph_edges(Core3d, (DrawingLabel, CellularAutomataLabel, ColourLabel, bevy::render::graph::CameraDriverLabel));
    }
}
