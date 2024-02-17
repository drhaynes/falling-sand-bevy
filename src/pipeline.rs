use bevy::app::App;
use bevy::prelude::Plugin;
use bevy::render::render_graph::RenderGraph;
use bevy::render::RenderApp;

pub mod cellular_automata;
pub mod drawing;
pub mod colour;

pub struct PipelinesPlugin;
impl Plugin for PipelinesPlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_plugin(drawing::DrawingPipelinePlugin)
            .add_plugin(cellular_automata::CellularAutomataPipelinePlugin)
            .add_plugin(colour::ColourPipelinePlugin);

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();

        let automata_id = render_graph.add_node("falling_sand", cellular_automata::CellularAutomataNode::default());
        let drawing_id = render_graph.add_node("drawing", drawing::DrawingNode::default());
        let colour_id = render_graph.add_node("colour", colour::ColourNode::default());

        // User Drawing Input -> Cellular Automata Simulation -> Cell Rendering (i.e. colour output) -> Camera
        render_graph.add_node_edge(drawing_id, automata_id);
        render_graph.add_node_edge(automata_id, colour_id);
        render_graph.add_node_edge(colour_id, bevy::render::main_graph::node::CAMERA_DRIVER);
    }
}
