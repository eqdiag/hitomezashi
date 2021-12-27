use core::num;

use nannou::{prelude::*, daggy::petgraph::visit::NodeIndexable};
use nannou_egui::Egui as egui;

fn main(){
    nannou::app(model).update(update).run();
}

const MAX_DIM: usize = 100;

pub const ALL_NAMED_COLORS: &[nannou::color::Srgb<u8>] = &[
    ALICEBLUE,
    ANTIQUEWHITE,
    AQUA,
    AQUAMARINE,
    AZURE,
    BEIGE,
    BISQUE,
    BLACK,
    BLANCHEDALMOND,
    BLUE,
    BLUEVIOLET,
    BROWN,
    BURLYWOOD,
    CADETBLUE,
    CHARTREUSE,
    CHOCOLATE,
    CORAL,
    CORNFLOWERBLUE,
    CORNSILK,
    CRIMSON,
    CYAN,
    DARKBLUE,
    DARKCYAN,
    DARKGOLDENROD,
    DARKGRAY,
    DARKGREEN,
    DARKGREY,
    DARKKHAKI,
    DARKMAGENTA,
    DARKOLIVEGREEN,
    DARKORANGE,
    DARKORCHID,
    DARKRED,
    DARKSALMON,
    DARKSEAGREEN,
    DARKSLATEBLUE,
    DARKSLATEGRAY,
    DARKSLATEGREY,
    DARKTURQUOISE,
    DARKVIOLET,
    DEEPPINK,
    DEEPSKYBLUE,
    DIMGRAY,
    DIMGREY,
    DODGERBLUE,
    FIREBRICK,
    FLORALWHITE,
    FORESTGREEN,
    FUCHSIA,
    GAINSBORO,
    GHOSTWHITE,
    GOLD,
    GOLDENROD,
    GRAY,
    GREEN,
    GREENYELLOW,
    GREY,
    HONEYDEW,
    HOTPINK,
    INDIANRED,
    INDIGO,
    IVORY,
    KHAKI,
    LAVENDER,
    LAVENDERBLUSH,
    LAWNGREEN,
    LEMONCHIFFON,
    LIGHTBLUE,
    LIGHTCORAL,
    LIGHTCYAN,
    LIGHTGOLDENRODYELLOW,
    LIGHTGRAY,
    LIGHTGREEN,
    LIGHTGREY,
    LIGHTPINK,
    LIGHTSALMON,
    LIGHTSEAGREEN,
    LIGHTSKYBLUE,
    LIGHTSLATEGRAY,
    LIGHTSLATEGREY,
    LIGHTSTEELBLUE,
    LIGHTYELLOW,
    LIME,
    LIMEGREEN,
    LINEN,
    MAGENTA,
    MAROON,
    MEDIUMAQUAMARINE,
    MEDIUMBLUE,
    MEDIUMORCHID,
    MEDIUMPURPLE,
    MEDIUMSEAGREEN,
    MEDIUMSLATEBLUE,
    MEDIUMSPRINGGREEN,
    MEDIUMTURQUOISE,
    MEDIUMVIOLETRED,
    MIDNIGHTBLUE,
    MINTCREAM,
    MISTYROSE,
    MOCCASIN,
    NAVAJOWHITE,
    NAVY,
    OLDLACE,
    OLIVE,
    OLIVEDRAB,
    ORANGE,
    ORANGERED,
    ORCHID,
    PALEGOLDENROD,
    PALEGREEN,
    PALETURQUOISE,
    PALEVIOLETRED,
    PAPAYAWHIP,
    PEACHPUFF,
    PERU,
    PINK,
    PLUM,
    POWDERBLUE,
    PURPLE,
    REBECCAPURPLE,
    RED,
    ROSYBROWN,
    ROYALBLUE,
    SADDLEBROWN,
    SALMON,
    SANDYBROWN,
    SEAGREEN,
    SEASHELL,
    SIENNA,
    SILVER,
    SKYBLUE,
    SLATEBLUE,
    SLATEGRAY,
    SLATEGREY,
    SNOW,
    SPRINGGREEN,
    STEELBLUE,
    TAN,
    TEAL,
    THISTLE,
    TOMATO,
    TURQUOISE,
    VIOLET,
    WHEAT,
    WHITE,
    WHITESMOKE,
    YELLOW,
    YELLOWGREEN,
];

#[derive(Clone)]
struct Cell{
    color_index: usize,
    pos: (usize,usize),
    visited: bool,
}

impl Cell{
    fn new() -> Self{
        Cell { color_index: 0, pos: (0,0), visited: false }
    }
}
struct SquareSwatch{
    size: f32,
    num_segments: usize,
    offsets: Vec<Vec<bool>>,
    cell_grid: Vec<Cell>
}


struct Model{
    window_id: window::Id,
    swatch: SquareSwatch,
    egui: egui,
}


impl SquareSwatch{

    pub fn new(size: f32,num_segments: usize) -> SquareSwatch{
        SquareSwatch{
            size,
            num_segments,
            offsets: vec![vec![false;num_segments*2],vec![false;num_segments*2]],
            cell_grid: vec![Cell::new();4*num_segments*num_segments],
        }
    }

    fn get_neighbors(&mut self,cell_index: usize) -> Option<Vec<usize>>{
        let i = cell_index / (self.num_segments * 2);
        let j = cell_index % (self.num_segments * 2);

        let mut neighbors = vec![];

        //Left 
        if j != 0{
            if (i % 2 == 0 && self.offsets[1][j-1]) || (i % 2 == 1 && !self.offsets[1][j-1]){
                neighbors.push(cell_index - 1);
            }            
        }

        //Right
        if j != (2*self.num_segments - 1){
            if (i % 2 == 0 && self.offsets[1][j]) || (i % 2 == 1 && !self.offsets[1][j]){
                neighbors.push(cell_index + 1);
            }   
        }

        //Bottom
        if i != 0{
            if (j % 2 == 0 && self.offsets[0][i-1]) || (j % 2 == 1 && !self.offsets[0][i-1]){
                neighbors.push(cell_index - 2*self.num_segments);
            }
        }

        //Top
        if i != (2*self.num_segments - 1){
            if (j % 2 == 0 && self.offsets[0][i]) || (j % 2 == 1 && !self.offsets[0][i]){
                neighbors.push(cell_index + 2*self.num_segments);
            }
        }

        if neighbors.len() > 0 {
            return Some(neighbors)
        }
        None
    }

    fn visit(&mut self,cell_index: usize,color_index: usize){
        if !self.cell_grid[cell_index].visited{
            self.cell_grid[cell_index].visited = true;
            self.cell_grid[cell_index].color_index = color_index;
            let _neighbors = self.get_neighbors(cell_index);
            match _neighbors{
                Some(neighbors) => {
                    for neighbor_index in neighbors{
                        self.visit(neighbor_index,color_index);
                    }
                },
                None => {}
            }
        }
    }

    pub fn fill_cells(&mut self){

        //Fill visit list
        let mut to_visit: Vec<usize> = vec![];
        for i in 0..self.num_segments*2{
            for j in 0..self.num_segments*2{
                to_visit.push(i*2*self.num_segments + j);
            }
        }

        let mut color_index: usize = 0;
        while(!to_visit.is_empty()){
            if let Some(cell_index) = to_visit.pop(){
                if !self.cell_grid[cell_index].visited{
                    self.visit(cell_index, color_index);
                    color_index += 1;
                    color_index %= ALL_NAMED_COLORS.len();
                }
            }
        }
    }


    pub fn generate_offsets(&mut self){
        for seq in self.offsets.iter_mut(){
            for seg in seq.iter_mut(){
                *seg = random_f32() < 0.5;
            }
        }
    }

    pub fn change_dim(&mut self,num_segments: usize){
        self.num_segments = num_segments;
        self.offsets = vec![vec![false;num_segments*2],vec![false;num_segments*2]];
        self.cell_grid = vec![Cell::new();4*num_segments*num_segments];
        self.generate_offsets();
        self.fill_cells();
    }

    pub fn increase_dim(&mut self){
        if(self.num_segments < MAX_DIM){
            self.change_dim(self.num_segments+1)
        }
    }

    pub fn decrease_dim(&mut self){
        if(self.num_segments > 0){
            self.change_dim(self.num_segments-1)
        }
    }
}


fn model(app: &App) -> Model{
    let window_id = app.new_window()
        .size(600, 600)
        .view(view)
        .key_pressed(key_pressed)
        .raw_event(raw_window_event)
        .build().unwrap();
    let window = app.window(window_id).unwrap();
    let mut swatch = SquareSwatch::new(400.0, 10);
    swatch.generate_offsets();
    swatch.fill_cells();
    Model { window_id: window_id,swatch,egui: egui::from_window(&window) }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key{
        Key::Space => {
            model.swatch.change_dim(model.swatch.num_segments);
        },
        Key::W => {
            model.swatch.increase_dim();
        },
        Key::S => {
            model.swatch.decrease_dim();
        },
        _ => {}
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn update(app: &App,model: &mut Model,update: Update){
    let ctx = model.egui.begin_frame();
    let mut x = 1.0;
    nannou_egui::egui::Window::new("Settings").show(&ctx, |ui| {
        let mut changed = false;
        let mut inc_dec = false;
        let old_num_segments = model.swatch.num_segments;
        changed |= ui.button("Generate").clicked();
        inc_dec |= ui
            .add(nannou_egui::egui::Slider::new(&mut model.swatch.num_segments, 0..=45).text("Decrement"))
            .changed();
        if changed {
            model.swatch.change_dim(model.swatch.num_segments);
        }
        if inc_dec{
            if model.swatch.num_segments > old_num_segments{
                model.swatch.increase_dim();
            }else{
                model.swatch.decrease_dim();
            }
        }

    });
}

fn view(app: &App,model: &Model,frame: Frame){
    let draw = app.draw();

    let size = model.swatch.size;
    let w = size;
    let h = size;
    let num_segments = model.swatch.num_segments;
    let segment_length = size/(2.0*num_segments as f32);
    let line_weight = 4.0;
    let color = ALL_NAMED_COLORS[ALL_NAMED_COLORS.len() - 1];

    draw.background().color(BLACK);

    for i in 0..2*num_segments{
        for j in 0..2*num_segments{
            let color_index = model.swatch.cell_grid[i*2*num_segments + j].color_index;
            draw.rect()
                .w_h(segment_length, segment_length)
                .x_y(segment_length/2.0-w/2.0 +j as f32*segment_length,segment_length/2.0 - h/2.0 + i as f32*segment_length)
                .color(ALL_NAMED_COLORS[color_index]);
        }
    }

    //Rows
    for i in 0..model.swatch.offsets[0].len(){
        let offset = if model.swatch.offsets[0][i] {1.0} else {0.0};
        for j in 0..model.swatch.num_segments{
            draw.line()
            .start(vec2((2*j) as f32*segment_length - w/2.0 + offset*segment_length,(i+1) as f32*segment_length - h/2.0))
            .end(vec2((2*j + 1) as f32*segment_length - w/2.0 + offset*segment_length,(i+1) as f32*segment_length - h/2.0))
            .start_cap_round()
            .end_cap_round()
            .weight(line_weight)
            .color(color);
        }
    }

    //Columns
    for i in 0..model.swatch.offsets[1].len(){
        let offset = if model.swatch.offsets[1][i] {1.0} else {0.0};
        for j in 0..model.swatch.num_segments{
            draw.line()
            .start(vec2((i+1) as f32 *segment_length - w/2.0,(2*j) as f32*segment_length - h/2.0 + offset*segment_length))
            .end(vec2((i+1) as f32 *segment_length - w/2.0,(2*j + 1) as f32*segment_length - h/2.0 + offset*segment_length))
            .weight(4.0)
            .color(color);
        }
    }
    draw.to_frame(app, &frame);

    model.egui.draw_to_frame(&frame);


}



