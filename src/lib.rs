pub struct Map {

    map_data:Vec<char>,
    width:u32,
    height:u32,
    upper_text:String,
    lower_text:String,

}

impl Map {

    // Function to create a new map
    pub fn new(w:u32, h:u32) -> Map {
        
        // Create a new map
        let mut local_data = Vec::<char>::new();

        // Populate the map
        for i in 0..w {
            
            for j in 0..h {

                local_data.push(
                    if i == 0 || j == 0 || (i == (w - 1)) || (j == (h - 1)) {
                        '*'
                    } else {
                        ' '
                    }
                );

            }

        }

        // Return the new map
        Map{
            map_data:local_data,
            width:w,
            height:h,
            upper_text:String::new(),
            lower_text:String::new(),
        }

    }

    // Function to udpate the map on the console
    pub fn update(&mut self) -> &mut Map {
        
        // Print the upper text (if exists)
        if self.upper_text.len() > 0 {
            println!("");
            println!("{}", self.upper_text);
            println!("");
        }

        // Print the map to the console
        for i in 0..self.width {
            
            for j in 0..self.height {

                print!("{} ", self.map_data[(j * self.width + i) as usize]);
            }

            println!("");
        }

        // Print the lower text (if exists)
        if self.lower_text.len() > 0 {
            println!("");
            println!("{}", self.lower_text);
            println!("");
        }

        self
    }

    // Function to clear the map
    pub fn clear(&mut self) -> &mut Map {
        
        // Populate the map
        for i in 0..self.width {
            
            for j in 0..self.height {

                self.map_data[(j * self.width + i) as usize] = 
                    if i == 0 || j == 0 || (i == (self.width - 1)) || (j == (self.height - 1)) {
                        '*'
                    } else {
                        ' '
                    }
                ;

            }

        }

        self

    }

    // Function to add a sprite on the map
    pub fn add_sprite(&mut self, opts:((u32, u32), char)) -> &mut Map {

        let x = (opts.0).0;
        let y = (opts.0).1;
        let s = opts.1;

        self.map_data[(y * self.width + x) as usize] = s;

        self

    }

    // Function to remove a sprite from the map
    pub fn remove_sprite(&mut self, pos:(u32, u32)) -> &mut Map {

        self.map_data[(pos.1 * self.width + pos.0) as usize] = ' ';

        self

    }

    // Function to move a sprite on the map
    pub fn move_sprite(&mut self, pos:(u32, u32), new_pos:(u32, u32)) -> &mut Map {

        let x = pos.0;
        let y = pos.1;
        let new_x = new_pos.0;
        let new_y = new_pos.1;

        self.map_data[(new_y * self.width + new_x) as usize] = self.map_data[(y * self.width + x) as usize];
        self.map_data[(y * self.width + x) as usize] = ' ';

        self

    }

    // Function to get the width of the map
    pub fn get_width(&self) -> u32 {
        
        self.width

    }

    // Function to get the height of the map
    pub fn get_height(&self) -> u32 {

        self.height

    }

    // Function to set the upper text of the map
    pub fn set_uppertext<S>(&mut self, utext:S) -> &mut Map where S:Into<String> {

        self.upper_text = utext.into();

        self

    }

    // Function to set the lower text of the map
    pub fn set_lowertext<S>(&mut self, ltext:S) -> &mut Map where S:Into<String> {

        self.lower_text = ltext.into();

        self

    }

    // Function to get the upper text of the map
    pub fn get_uppertext(&self) -> String {

        self.upper_text.clone()

    }

    // Function to get the lower text of the map
    pub fn get_lowertext(&self) -> String {

        self.lower_text.clone()

    }

}