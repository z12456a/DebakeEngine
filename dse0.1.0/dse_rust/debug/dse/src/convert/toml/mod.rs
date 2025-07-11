#[cfg(feature = "env_bit_64bit")]
pub mod env {

    ///
    /// "### " in debake toml means block
    /// use "### BEGIN" and "### END" to flag a block range
    /// ### 用于标记文件中块 读取时根据标记的块进行读取
    ///  
    #[derive(Default)]
    pub struct TomlDecoderE {
        id : u64,
        raw: String,
        block_name: String,
        table_name: String,
    }

    impl TomlDecoderE {
        pub fn build() -> Self {
            return Default::default();
        }

        pub fn build_raw(mut self, raw_in: String) -> Self {
            self.raw = raw_in;
            return self;
        }

        pub fn build_specify_block(mut self, block_in: String) -> Self  {
            self.block_name = block_in;
            return self;
        }

        pub fn build_specify_table(mut self, table_in: String) -> Self  {
            self.table_name = table_in;
            return self;
        }

        pub fn into_block(self) -> String {
            let mut _r = String::new();
            let mut switch = false;
            for si in self.raw.lines() {
                if switch {
                    if si.contains("###") && si.contains("END") && si.contains(&self.block_name) {
                        switch = false;
                    } else {
                        _r = _r + si + "\n";
                    }
                } else {
                    if si.contains("###") && si.contains("BEGING") && si.contains(&self.block_name)
                    {
                        switch = true;
                    }
                }
            }
            match _r.is_empty() {
                true => {
                    crate::send2logger_dev!(
                        crate::log::code::TYPE_EXE_ERROR
                            | crate::log::code::CONDI_UNEXPECTED_RESULT
                            | crate::log::code::FILE_CONVERT_TOML
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogCodePart::Line)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogCodePart::Id)
                                .get_code()
                    );
                    return self.raw;
                }
                false => return _r,
            }
        }


        pub fn into_table(self) -> Vec<String> {
            let mut _r = Vec::<String>::default();
            let _table_name = self.table_name.clone();
            let _raw = self.into_block();
            let mut _table = String::default();
            let mut _switch = false;
            for si in _raw.lines() {
                let is_title = !si.contains("=")
                    && !si.contains("#")
                    && si.contains("[[")
                    && si.contains("]]");
                if _switch {
                    if is_title {
                        _r.push(_table.clone());
                        _table.clear();
                        if !si.contains(_table_name.as_str()) && !_table_name.is_empty() {
                            _switch = false;
                        }
                    } else {
                        _table = _table + "\n" + si;
                    }
                } else if is_title {                     
                        if si.contains(_table_name.as_str()) || _table_name.is_empty() {
                            _switch = true;
                        }
                    
                }
            }
            _r.push(_table);
            return _r;
        }
    }
}
