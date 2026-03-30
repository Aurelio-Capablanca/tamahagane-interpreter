pub struct Domain {
    status: bool,
    command: String,
    mode: f64,
    precision: i32,
    name: String,
    sub_mode: Vec<Box<Domain>>,
}

impl Domain {
    pub fn empty() -> Self {
        Self {
            status: true,
            command: "".to_string(),
            mode: 0_f64,
            precision: 0_i32,
            name: "NONE".to_string(),
            sub_mode: Vec::new(),
        }
    }

    pub fn new(
        status: bool,
        command: String,
        mode: f64,
        precision: i32,
        name: String,
        sub_mode: Vec<Box<Domain>>,
    ) -> Self {
        Self {
            status: status,
            command: command,
            mode: mode,
            precision: precision,
            name: name,
            sub_mode: sub_mode,
        }
    }

    //get as reference library
    pub fn get_precision_as_ref(&self) -> &i32 {
        &self.precision
    }

    pub fn get_mode_as_ref(&self) -> &f64 {
        &self.mode
    }

    //set and get library
    pub fn set_precision_and_get(mut self, precision: i32) -> Self {
        self.precision = precision;
        self
    }

    pub fn set_mode_and_get(mut self, mode: f64) -> Self {
        self.mode = mode;
        self
    }

    pub fn set_status_and_get(mut self, status_in: bool) -> Self {
        self.status = status_in;
        self
    }

    pub fn set_command_and_get(mut self, command_in: String) -> Self {
        self.command = command_in;
        self
    }
    
    pub fn add_sub_modes_and_get(mut self, submode: Domain) -> Self {
        self.sub_mode.push(Box::new(submode));
        self
    }
}
