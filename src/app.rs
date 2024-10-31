use rubbl_casatables::{GlueDataType, Table, TableOpenMode};
use rubbl_core::ndarray::{Array1, Array2, Array3, Ix1, Ix2};
use rubbl_core::{Array, Complex};

pub enum CurrentScreen {
    Main,
    Exiting,
}

pub enum CurrentlyEditing {
    Table,
    Column,
    Information,
}

pub struct App {
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: CurrentlyEditing, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
    pub ms_name: String,
    pub ms_table: Table,
    pub tables: Vec<String>,
    pub columns: Vec<String>,
    pub current_table: usize,
    pub current_column: usize,
    pub text_buffer: String,
    pub text_scroll: u16,
    pub tab_scroll: u64,
    pub line_height: u16,
}

impl App {
    pub fn new(ms_in: String) -> App {
        let mut t = Table::open(ms_in.clone(), TableOpenMode::Read).expect("Failed to read MS.");

        let mut tables = vec!["MAIN".to_string()];
        tables.append(&mut t.table_keyword_names().expect("Failed"));
        let columns = t.column_names().expect("Failed to read column names.");

        let mut app = App {
            current_screen: CurrentScreen::Main,
            currently_editing: CurrentlyEditing::Table,
            ms_name: ms_in.trim_end_matches("/").to_string(),
            ms_table: t,
            tables,
            columns,
            current_column: 0,
            current_table: 0,
            text_buffer: "".to_string(),
            text_scroll: 0,
            tab_scroll: 0,
            line_height: 50,
        };
        app.select(true);
        app
    }

    pub fn toggle_editing(&mut self, forwards: bool) {
        match &self.currently_editing {
            CurrentlyEditing::Table => {
                if forwards {
                    self.currently_editing = CurrentlyEditing::Column
                } else {
                    self.currently_editing = CurrentlyEditing::Information
                }
            }
            CurrentlyEditing::Column => {
                if forwards {
                    self.currently_editing = CurrentlyEditing::Information
                } else {
                    self.currently_editing = CurrentlyEditing::Table
                }
            }
            CurrentlyEditing::Information => {
                if forwards {
                    self.currently_editing = CurrentlyEditing::Table
                } else {
                    self.currently_editing = CurrentlyEditing::Column
                }
            }
        };
    }

    pub fn increase_soltab(&mut self, amount: u16) {
        match &self.currently_editing {
            CurrentlyEditing::Information => {
                //self.text_scroll += amount;
                self.tab_scroll += amount as u64;
            }
            CurrentlyEditing::Column => {
                self.current_column += 1;
                if self.current_column >= self.columns.len() {
                    self.current_column = 0;
                }
            }
            CurrentlyEditing::Table => {
                self.current_table += 1;
                if self.current_table >= self.tables.len() {
                    self.current_table = 0;
                }
            }
        }
    }

    pub fn decrease_soltab(&mut self, amount: u16) {
        match &self.currently_editing {
            CurrentlyEditing::Information => {
                if self.tab_scroll > 0 {
                    if amount as u64 <= self.tab_scroll {
                        //self.text_scroll -= amount;
                        self.tab_scroll -= amount as u64;
                    } else {
                        self.tab_scroll = 0;
                    }
                }
            }
            CurrentlyEditing::Table => {
                if self.current_table == 0 {
                    self.current_table = self.tables.len() - 1;
                } else {
                    self.current_table -= 1;
                }
                self.update_soltabs();
            }
            CurrentlyEditing::Column => {
                if self.current_column == 0 {
                    self.current_column = self.columns.len() - 1;
                } else {
                    self.current_column -= 1;
                }
                self.update_soltabs();
            }
        }
    }

    pub fn update_soltabs(&mut self) {
        match &self.currently_editing {
            _ => {}
        }
    }

    pub fn read_scalar_value_into_buffer(
        &mut self,
        buf: &mut String,
        column_name: &str,
        start_row: u64,
        end_row: u64,
    ) -> String {
        buf.push_str("Values: \n");
        let col_desc = self.ms_table.get_col_desc(column_name).expect("Failed");
        let mut main_row = self.ms_table.get_row_reader().expect("Failed");

        buf.push_str(&format!(":{:^5}: ", "ROW"));
        buf.push_str("VALUE");
        buf.push_str("\n");
        for row_num in start_row..end_row {
            buf.push_str(&format!(":{:>5}: ", row_num));
            self.ms_table
                .read_row(&mut main_row, row_num)
                .expect("Failed");
            match col_desc.data_type() {
                GlueDataType::TpChar => {
                    let data = main_row.get_cell::<i8>(column_name).expect("Failed");
                    buf.push_str(&format!("{}", data));
                }
                GlueDataType::TpUChar => {
                    let data = main_row.get_cell::<u8>(column_name).expect("Failed");
                    buf.push_str(&format!("{}", data));
                }
                GlueDataType::TpShort => {
                    let data = main_row.get_cell::<i16>(column_name).expect("Failed");
                    buf.push_str(&format!("{}", data));
                }
                GlueDataType::TpUShort => {
                    let data = main_row.get_cell::<u16>(column_name).expect("Failed");
                    buf.push_str(&format!("{}", data));
                }
                GlueDataType::TpInt => {
                    let data = main_row.get_cell::<i32>(column_name).expect("Failed");
                    buf.push_str(&format!("{}", data));
                }
                GlueDataType::TpInt64 => {
                    let data = main_row.get_cell::<i64>(column_name).expect("Failed");
                    buf.push_str(&format!("{}", data));
                }
                GlueDataType::TpUInt => {
                    let data = main_row.get_cell::<u32>(column_name).expect("Failed");
                    buf.push_str(&format!("{}", data));
                }
                GlueDataType::TpFloat => {
                    let data = main_row.get_cell::<f32>(column_name).expect("Failed");
                    buf.push_str(&format!("{}", data));
                }
                GlueDataType::TpDouble => {
                    let data = main_row.get_cell::<f64>(column_name).expect("Failed");
                    buf.push_str(&format!("{}", data));
                }
                GlueDataType::TpBool => {
                    let data = main_row.get_cell::<bool>(column_name).expect("Failed");
                    buf.push_str(&format!("{}", data));
                }
                GlueDataType::TpString => {
                    let data = main_row.get_cell::<String>(column_name).expect("Failed");
                    buf.push_str(&format!("{}", data));
                }
                GlueDataType::TpComplex => {
                    let data = main_row
                        .get_cell::<Complex<f32>>(column_name)
                        .expect("Failed");
                    buf.push_str(&format!("{}", data));
                }
                GlueDataType::TpDComplex => {
                    let data = main_row
                        .get_cell::<Complex<f64>>(column_name)
                        .expect("Failed");
                    buf.push_str(&format!("{}", data));
                }
                _ => {
                    let data = "Not implemented";
                    buf.push_str(&format!("{}", data));
                }
            }
            buf.push_str("\n");
        }
        buf.to_string()
    }

    fn read_var_col<
        T: rubbl_casatables::CasaScalarData + Copy + std::fmt::Debug + std::fmt::Display,
    >(
        &mut self,
        buf: &mut String,
        column_name: &str,
        row_num: u64,
    ) {
        let mut main_row = self.ms_table.get_row_reader().expect("Failed");
        let _data = self.ms_table.read_row(&mut main_row, row_num).unwrap();
        let mdata = main_row.get_cell::<Array1<T>>(column_name);
        match mdata {
            Ok(x) => buf.push_str(&format!("{}\n", x)),
            Err(..) => {
                let mdata = main_row.get_cell::<Array2<T>>(column_name);
                match mdata {
                    Ok(x) => buf.push_str(&format!("{}\n", x)),
                    Err(..) => {
                        let mdata = main_row.get_cell::<Array3<T>>(column_name);
                        match mdata {
                            Ok(x) => buf.push_str(&format!("{}\n", x)),
                            Err(..) => buf.push_str("Failed to parse field.\n"),
                        }
                    }
                }
            }
        }
    }

    pub fn read_array_value_into_buffer(
        &mut self,
        buf: &mut String,
        column_name: &str,
        row_start: u64,
        row_end: u64,
    ) -> String {
        buf.push_str("Values: \n");
        let col_desc = self.ms_table.get_col_desc(column_name).expect("Failed");
        if !col_desc.is_fixed_shape() {
            buf.push_str(&format!(":{:^5}: ", "ROW"));
            buf.push_str("VALUE");
            buf.push_str("\n");
            for row_num in row_start..row_end {
                buf.push_str(&format!(":{:>5}: ", row_num));
                match col_desc.data_type() {
                    GlueDataType::TpBool => self.read_var_col::<bool>(buf, column_name, row_num),
                    GlueDataType::TpChar => self.read_var_col::<i8>(buf, column_name, row_num),
                    GlueDataType::TpUChar => self.read_var_col::<u8>(buf, column_name, row_num),
                    GlueDataType::TpShort => self.read_var_col::<i16>(buf, column_name, row_num),
                    GlueDataType::TpUShort => self.read_var_col::<u16>(buf, column_name, row_num),
                    GlueDataType::TpInt => self.read_var_col::<i32>(buf, column_name, row_num),
                    GlueDataType::TpUInt => self.read_var_col::<u32>(buf, column_name, row_num),
                    GlueDataType::TpInt64 => self.read_var_col::<i64>(buf, column_name, row_num),
                    GlueDataType::TpFloat => self.read_var_col::<f32>(buf, column_name, row_num),
                    GlueDataType::TpDouble => self.read_var_col::<f64>(buf, column_name, row_num),
                    GlueDataType::TpComplex => {
                        self.read_var_col::<Complex<f32>>(buf, column_name, row_num)
                    }
                    GlueDataType::TpDComplex => {
                        self.read_var_col::<Complex<f64>>(buf, column_name, row_num)
                    }
                    GlueDataType::TpString => {
                        let mut main_row = self.ms_table.get_row_reader().expect("Failed");
                        let _data = self.ms_table.read_row(&mut main_row, row_num).unwrap();
                        let mdata = main_row
                            .get_cell::<Vec<String>>(column_name)
                            .expect("Failed to parse string column.");
                        buf.push_str(&format!("[{}]\n", mdata.join(", ")));
                    }
                    _ => {
                        let data = format!("Not implemented for {}", col_desc.data_type());
                        buf.push_str(&format!("{}\n", data));
                    }
                }
            }
            return buf.to_string();
        }
        let mut main_row = self.ms_table.get_row_reader().expect("Failed");
        buf.push_str(&format!(":{:^5}: ", "ROW"));
        buf.push_str("VALUE");
        buf.push_str("\n");
        for row_num in row_start..row_end {
            self.ms_table
                .read_row(&mut main_row, row_num)
                .expect("Failed");

            buf.push_str(&format!(":{:>5}: ", row_num));
            match col_desc.data_type() {
                GlueDataType::TpArrayChar | GlueDataType::TpChar => {
                    match col_desc.shape().unwrap().len() {
                        1 => {
                            let data = main_row
                                .get_cell::<Array<i8, Ix1>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        2 => {
                            let data = main_row
                                .get_cell::<Array<i8, Ix2>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        _ => {}
                    };
                }
                GlueDataType::TpArrayUChar | GlueDataType::TpUChar => {
                    match col_desc.shape().unwrap().len() {
                        1 => {
                            let data = main_row
                                .get_cell::<Array<u8, Ix1>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        2 => {
                            let data = main_row
                                .get_cell::<Array<u8, Ix2>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        _ => {}
                    };
                }
                GlueDataType::TpArrayShort | GlueDataType::TpShort => {
                    match col_desc.shape().unwrap().len() {
                        1 => {
                            let data = main_row
                                .get_cell::<Array<i16, Ix1>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        2 => {
                            let data = main_row
                                .get_cell::<Array<i16, Ix2>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        _ => {}
                    };
                }
                GlueDataType::TpArrayUShort | GlueDataType::TpUShort => {
                    match col_desc.shape().unwrap().len() {
                        1 => {
                            let data = main_row
                                .get_cell::<Array<u16, Ix1>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        2 => {
                            let data = main_row
                                .get_cell::<Array<u16, Ix2>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        _ => {}
                    };
                }
                GlueDataType::TpArrayInt | GlueDataType::TpInt => {
                    match col_desc.shape().unwrap().len() {
                        1 => {
                            let data = main_row
                                .get_cell::<Array<i32, Ix1>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        2 => {
                            let data = main_row
                                .get_cell::<Array<i32, Ix2>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        _ => {}
                    };
                }
                GlueDataType::TpArrayInt64 | GlueDataType::TpInt64 => {
                    match col_desc.shape().unwrap().len() {
                        1 => {
                            let data = main_row
                                .get_cell::<Array<i64, Ix1>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        2 => {
                            let data = main_row
                                .get_cell::<Array<i64, Ix2>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        _ => {}
                    };
                }
                GlueDataType::TpArrayUInt | GlueDataType::TpUInt => {
                    match col_desc.shape().unwrap().len() {
                        1 => {
                            let data = main_row
                                .get_cell::<Array<u32, Ix1>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        2 => {
                            let data = main_row
                                .get_cell::<Array<u32, Ix2>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        _ => {}
                    };
                }
                GlueDataType::TpArrayFloat | GlueDataType::TpFloat => {
                    match col_desc.shape().unwrap().len() {
                        1 => {
                            let data = main_row
                                .get_cell::<Array<f32, Ix1>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        2 => {
                            let data = main_row
                                .get_cell::<Array<f32, Ix2>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        _ => {}
                    };
                }
                GlueDataType::TpArrayDouble | GlueDataType::TpDouble => {
                    match col_desc.shape().unwrap().len() {
                        1 => {
                            let data = main_row
                                .get_cell::<Array<f64, Ix1>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        2 => {
                            let data = main_row
                                .get_cell::<Array<f64, Ix2>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        _ => {}
                    };
                }
                GlueDataType::TpArrayBool | GlueDataType::TpBool => {
                    match col_desc.shape().unwrap().len() {
                        1 => {
                            let data = main_row.get_cell::<Vec<bool>>(column_name).expect("Failed");
                            buf.push_str(&format!("{:?}", data));
                        }
                        2 => {
                            let data = main_row
                                .get_cell::<Array<bool, Ix2>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        _ => {
                            let data = main_row.get_cell::<Vec<bool>>(column_name).expect("Failed");
                            buf.push_str(&format!("{:?}", data));
                        }
                    };
                }
                GlueDataType::TpArrayString | GlueDataType::TpString => {
                    let data = main_row.get_cell::<String>(column_name).expect("Failed");
                    buf.push_str(&format!("{}", data));
                }
                GlueDataType::TpArrayComplex | GlueDataType::TpComplex => {
                    match col_desc.shape().unwrap().len() {
                        1 => {
                            let data = main_row
                                .get_cell::<Array<Complex<f32>, Ix1>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        2 => {
                            let data = main_row
                                .get_cell::<Array<Complex<f32>, Ix2>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        _ => {}
                    };
                }
                GlueDataType::TpArrayDComplex | GlueDataType::TpDComplex => {
                    match col_desc.shape().unwrap().len() {
                        1 => {
                            let data = main_row
                                .get_cell::<Array<Complex<f64>, Ix1>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        2 => {
                            let data = main_row
                                .get_cell::<Array<Complex<f64>, Ix2>>(column_name)
                                .expect("Failed");
                            buf.push_str(&format!("{}", data));
                        }
                        _ => {}
                    };
                }
                _ => {
                    let data = format!("Not implemented for {}", col_desc.data_type());
                    buf.push_str(&format!("{}", data));
                }
            }
            buf.push_str("\n");
        }
        buf.to_string()
    }

    pub fn select(&mut self, reset_view: bool) {
        match &self.currently_editing {
            CurrentlyEditing::Information => {
                let mut buf = "".to_string();

                let column_name = self.columns[self.current_column].clone();
                let col_desc = self.ms_table.get_col_desc(&column_name).expect("Failed");
                let col_kw = self
                    .ms_table
                    .column_keyword_names(&column_name)
                    .expect("Failed");
                buf.push_str(&format!("Column name: {}\n", column_name));
                buf.push_str(&format!("Column data type: {}\n", col_desc.data_type()));
                buf.push_str(&format!("Column keywords: {}\n", col_kw.join(", ")));
                buf.push_str(&format!("Scalar: {}\n", col_desc.is_scalar()));
                if !col_desc.is_scalar() {
                    buf.push_str(&format!("Fixed shape: {}\n", col_desc.is_fixed_shape()));
                }

                match col_desc.is_scalar() {
                    true => {
                        if self.ms_table.n_rows() < self.line_height.into() {
                            self.read_scalar_value_into_buffer(
                                &mut buf,
                                &column_name,
                                0,
                                self.ms_table.n_rows(),
                            )
                        } else {
                            self.read_scalar_value_into_buffer(
                                &mut buf,
                                &column_name,
                                self.tab_scroll as u64,
                                self.tab_scroll as u64 + self.line_height as u64,
                            )
                        }
                    }
                    false => {
                        if self.ms_table.n_rows() < self.line_height.into() {
                            self.read_array_value_into_buffer(
                                &mut buf,
                                &column_name,
                                self.text_scroll as u64,
                                self.ms_table.n_rows(),
                            )
                        } else {
                            self.read_array_value_into_buffer(
                                &mut buf,
                                &column_name,
                                self.tab_scroll as u64,
                                self.tab_scroll as u64 + self.line_height as u64,
                            )
                        }
                    }
                };

                self.text_buffer = buf;
            }
            CurrentlyEditing::Column => {
                let mut buf = "".to_string();

                let column_name = self.columns[self.current_column].clone();
                let col_desc = self.ms_table.get_col_desc(&column_name).expect("Failed");
                let col_kw = self
                    .ms_table
                    .column_keyword_names(&column_name)
                    .expect("Failed");
                buf.push_str(&format!("Column name: {}\n", column_name));
                buf.push_str(&format!("Column data type: {}\n", col_desc.data_type()));
                buf.push_str(&format!("Column keywords: {}\n", col_kw.join(", ")));
                buf.push_str(&format!("Scalar: {}\n", col_desc.is_scalar()));
                if !col_desc.is_scalar() {
                    buf.push_str(&format!("Fixed shape: {}\n", col_desc.is_fixed_shape()));
                }

                match col_desc.is_scalar() {
                    true => {
                        if self.ms_table.n_rows() < 50 {
                            self.read_scalar_value_into_buffer(
                                &mut buf,
                                &column_name,
                                0,
                                self.ms_table.n_rows(),
                            )
                        } else {
                            self.read_scalar_value_into_buffer(&mut buf, &column_name, 0, 50)
                        }
                    }
                    false => {
                        if self.ms_table.n_rows() < 50 {
                            self.read_array_value_into_buffer(
                                &mut buf,
                                &column_name,
                                self.text_scroll as u64,
                                self.ms_table.n_rows(),
                            )
                        } else {
                            self.read_array_value_into_buffer(&mut buf, &column_name, 0, 50)
                        }
                    }
                };
                self.text_buffer = buf;
            }
            CurrentlyEditing::Table => {
                let table_name = &self.tables[self.current_table];
                let t = if table_name == "MAIN" {
                    Table::open(format!("{}", self.ms_name), TableOpenMode::Read)
                        .expect("Failed to read MS.")
                } else {
                    Table::open(
                        format!("{}/{}", self.ms_name, table_name),
                        TableOpenMode::Read,
                    )
                    .expect("Failed to read MS.")
                };
                self.ms_table = t;
                self.columns = self.ms_table.column_names().expect("Failed");
                self.current_column = 0;
            }
        }
        if reset_view {
            self.text_scroll = 0;
            self.tab_scroll = 0;
        }
    }
}
