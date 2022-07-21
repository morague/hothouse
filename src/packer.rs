// RECTANGLE PACKING METHOD

use std::vec;
use std::string::String;
use rand::Rng;
use std::collections::HashMap;  
use math::round::{ceil, floor}; 








#[derive(Debug, Clone)]
pub struct HotBuilder<'a>{
    pub order: Vec<&'a char>,
    pub collection: RectCollection,
    pub references: Vec<char>,
    pub shapes: Vec<String>,
    pub dimensions: Vec<HashMap<&'a str, usize>>,
    pub scores: Vec<f64>,
    pub global_score: f64,
    pub global_counter: Vec<HashMap<String, usize>>,
    pub global_grid: Vec<Vec<Vec<Position>>> 
}


impl<'a> HotBuilder<'a> {
    
    pub fn new  (  
                order: Vec<&'a char>, 
                collection: RectCollection,
                references: Vec<char>,
                shapes: Vec<String>,
                dimensions: Vec<HashMap<&'a str, usize>>
                ) -> Self {
        Self { 
            order: order,
            collection: collection,
            references: references, 
            shapes: shapes,
            dimensions: dimensions,
            scores: Vec::new(),
            global_score: 0.0,
            global_counter: Vec::new(),
            global_grid: Vec::new()
        }
    }


    pub fn run(&mut self) {
        
        for section in self.order.clone() {
            println!("____________{section}____________");
            match section {
                'f' => self.iterator(5, *section),
                'b' => self.iterator(1, *section),
                'l' => self.iterator(1, *section),
                'r' => self.iterator(1, *section),
                't' => self.iterator(1, *section),
                _   => ()
            }
        }
    }


    pub fn build  (&mut self, shape: String, 
                                dim: HashMap<&str, usize>, 
                                collection: RectCollection,
                    ) -> (f64, HashMap<String, usize>, Vec<Vec<Position>>, RectCollection) {

        let mut packer = Packer::new(   
                                            shape.clone(),
                                            dim,
                                            collection
                                            );
        
        packer.container.init_grid();
        match shape.as_str() {
            "door"  =>   { let left: usize = packer.container.rand_left_door_point();
                        packer.container.draw_door(left);
                        },
            "obl"   => packer.container.draw_obl_roof(),
            _       => ()
        }

        packer.calculate();
        packer.score();
        
        (packer.score, packer.counter, packer.container.grid, packer.rect_collection)
    }


    pub fn iterator(&mut self, n:usize, face: char) {
        let index: usize = self.references.iter().position(|&r| r == face).unwrap();
        let mut  cached_score: f64 = 0.0;
        let mut cached_counter: HashMap<String, usize> = HashMap::new();
        let mut cached_grid: Vec<Vec<Position>> = Vec::new();
        let mut cached_collection: RectCollection = self.collection.clone();


        let collection: RectCollection = self.collection.clone();
        let shifter = self.shift(collection);

        for shift in shifter {
            for _ in 0..n {
                let c: RectCollection = shift.clone();
                
                let (score, 
                    counter, 
                    grid,
                    c
                    ) = self.build( self.shapes[index].clone(),
                                    self.dimensions[index].clone(), 
                                    c);
    
                if score > cached_score {
                    cached_score = score;
                    cached_counter = counter;
                    cached_grid = grid;
                    cached_collection = c;
                }
            }
        }
        /////// must had best possible front to cached structure
        self.global_score += cached_score;
        self.scores.push(cached_score);
        self.global_counter.push(cached_counter);
        self.global_grid.push(cached_grid);
        self.collection = cached_collection;
        self.collection.sort_collection();

    }


    pub fn shift(&self, collection: RectCollection) -> Vec<RectCollection> {
        //return vec of shifted +1..+j collection
        
        let mut shift: Vec<RectCollection> = Vec::new();
        for i in 0..collection.collection.len() {
            let mut c: Vec<Rect> = collection.collection.clone();
            for _ in 0..i {
                let elm: Rect = c[0].clone();
                c.remove(0);
                c.push(elm);
            }
            shift.push(RectCollection{collection: c});
        }
        shift
    }

    // pub fn return_best(&self) ->(Vec<f64>, 
    //                             f64, 
    //                             Vec<HashMap<String, usize>>, 
    //                             Vec<Vec<Vec<Position>>> ,
    //                             RectCollection) {
        
    //     (self.scores, self.global_score, self.global_counter, self.global_grid, self.collection)
    // }

}





#[derive(Debug, Clone)]
pub struct Packer {
    pub container: Container,
    pub rect_collection: RectCollection,
    pub score: f64,
    pub counter: HashMap<String, usize>
}


impl Packer {
    pub fn new  (
                shape: String,
                dim: HashMap<&str, usize>,
                collection: RectCollection
                ) -> Self {

        Self {
            container: Container::new(  
                                        shape,
                                        dim
                                    ),
            rect_collection: collection,
            score: 0.0,
            counter: HashMap::new()
        }
    }



    pub fn calculate(&mut self) {
        let w = self.container.width;
        let h = self.container.height;
        let mut rect_counter: usize = 0;
        let mut index: usize = 0;


        while index < self.rect_collection.collection.len() {
            let rect: &mut Rect = &mut self.rect_collection.collection[index];
            
            // println!("{:?}", rect); 

            'reverse: for r in vec![false, true]{

                if rect.stock == 0 {
                    // avoid panic when
                    break 'reverse
                }

                let height = match r {
                    false => rect.height,
                    true => rect.width,
                };

                let width = match r {
                    false => rect.width,
                    true => rect.height,
                };


                for y in 0..self.container.height {
                    for x in 0..self.container.width {
    
                        // CHECK IF NOT OVERLAPPING EDGES
                        if (y + height) >= h || (x + width) >= w {
                            continue;
                        }
    
                        // CHECK IF TOPLEFT & BOTTOMRIGHT CORNER ARE NOT PACKED
                        if !self.container.grid[y][x].packed 
                            && !self.container.grid[y + height][x + width].packed {
                                // println!("is_checking");
    
                            // CHECK IF ALL INNER PIXELS ARE VALID
                            let mut validity: bool = true;
                            'inner: for j in x..x + width - 1 {
                                for i in y.. y + height - 1 {
                                    if self.container.grid[i][j].packed {
                                        validity = false;
                                        break 'inner;
                                    }
                                }
                            }
    
                            if validity {
                                // println!("valid");
                                rect_counter += 1;
                                rect.pack();

                                if self.counter.contains_key(&rect.id.to_string()) {
                                    *self.counter.get_mut(&rect.id.to_string()).unwrap() += 1;
                                } else {
                                    self.counter.insert(rect.id.to_string(), 1);
                                }
    
                                for j in x..x + width - 1 {
                                    for i in y.. y + height - 1 {
                                        self.container.grid[i][j].packed();
                                        self.container.grid[i][j].block_id = rect_counter;
                                        self.container.grid[i][j].rect_id = rect.id;
                                        self.container.grid[i][j].label = String::from("glass");
                                    }
                                }
    
                                if !rect.remain() {
                                    index += 1;
                                    break 'reverse;
                                }
                            }
                        }
                    } //width
                } //height
            } //reverse dim
            index += 1;
        } //while
    }

    pub fn score(&mut self) {
        let container_area = self.container.height * self.container.width;
        let mut coverage = 0;
        for y in 0..self.container.height {
            for x in 0..self.container.width { 
                if self.container.grid[y][x].packed 
                    && self.container.grid[y][x].rect_id != 0 {
                    coverage += 1;
                }
            }
        }

        self.score = ((coverage as f64 / container_area as f64) * 100.0) as f64;
    }


}




#[derive(Debug, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub packed: bool,
    pub block_id: usize,
    pub rect_id: usize,
    pub label: String
}

impl Position {

    pub fn new(y: usize, x: usize) -> Self {
        Self {
            x: x,
            y: y,
            packed: false,
            block_id: 0,
            rect_id: 0,
            label: String::from("wood")
        }
    }

    pub fn packed(&mut self) {
        self.packed = true
    }

}



#[derive(Debug, Clone)]
pub struct Container {
    pub shape: String, // current shapes "rect"; "obl" with non straigt top ;"door" (door)
    pub width: usize,
    pub height: usize,
    pub height_small: usize,
    pub door_width: usize,
    pub door_height: usize,
    pub grid: Vec<Vec<Position>>
}

impl Container {

    pub fn new  ( 
                shape: String,
                dim: HashMap<&str, usize>
                ) -> Self {

        Self {
            shape: shape,
            width: *dim.get(&"container_width").unwrap(),
            height: *dim.get(&"container_height").unwrap(),
            height_small: *dim.get(&"container_small_height").unwrap(),
            door_width: *dim.get(&"door_width").unwrap(),
            door_height: *dim.get(&"door_height").unwrap(),
            grid: Vec::new()
        }
    }

    pub fn init_grid(&mut self) {
        self.grid = Vec::new();
        for i in 0..self.height {
            let mut row: Vec<Position> = Vec::new();

            for j in 0..self.width {
                row.push(
                    Position::new(i, j)
                )
            }
            self.grid.push(row);
        }
    }



    pub fn draw_obl_roof(&mut self) {
        let height_dif: usize = self.height - self.height_small;
        let width: usize = self.width;

        let step: usize = ceil(width as f64 / height_dif as f64, 0) as usize;
        let section_debt: f64 =  step as f64 - (width as f64 / height_dif as f64);

        // DRAW
        let mut debt: f64 = 0.0;
        for y in 0..height_dif - 1 {
            debt += section_debt;
            let correction: usize = floor(debt, 0) as usize ;
            let current_step_size: usize = (y + 1) * step - correction;
            let mut current_step: usize = 0;
            for x in 0..width {
                if current_step < current_step_size {
                    current_step += 1;
                } else {
                    self.grid[y][x].packed = true;
                    self.grid[y][x].label = String::from("none");
                }
            }
        }
    }

    pub fn draw_door(&mut self, left: usize) {

        let top: usize = self.height - self.door_height;
        let right: usize = left + self.door_width;
        
        for y in top..self.height {
            for x in left..right{
                self.grid[y][x].packed = true;
                self.grid[y][x].label = String::from("door");
            }
        }
    }


    pub fn rand_left_door_point(&self) -> usize {
        let floor: usize = 10;
        let ceil: usize = self.width - self.door_width - 10;
        let left = rand::thread_rng().gen_range(floor..ceil);

        left
    }

}






#[derive(Debug, Clone)]
pub struct RectCollection {
    pub collection: Vec<Rect>
}

impl RectCollection {

    pub fn new() -> Self {
        Self { collection: Vec::new() }
    }

    pub fn build_collection(&mut self,
                            v_id: Vec<usize>,
                            v_width: Vec<usize>,
                            v_height: Vec<usize>,
                            v_stock: Vec<usize>) {

        for i in 0..v_stock.len() {

            let rect = Rect::new(
                                    v_id[i],
                                    v_width[i],
                                    v_height[i],
                                    v_stock[i]
                                );

            self.collection.push(rect);
        }

        self.sort_collection();
    }

    pub fn sort_collection(&mut self) {
        self.collection.sort_by(|a, b| b.area.partial_cmp(&a.area).unwrap());
    }    
}



#[derive(Debug, Clone)]
pub struct Rect {
    pub id: usize,
    pub width: usize,
    pub height: usize,
    pub area: usize,
    pub stock: usize
}

impl Rect {

    pub fn new(id: usize, width: usize, height: usize, stock:usize) -> Self {
        Self {
            id: id,
            width: width,
            height: height,
            area: width*height,
            stock: stock
        }
    }

    pub fn pack(&mut self) {
        self.stock -= 1
    }

    pub fn remain(&self) -> bool {
        if self.stock > 0 {
            true
        } else {
            false
        }
    }
}   

