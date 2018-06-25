

#[derive(Debug)]
struct NetCDF {
    filename: String
}

impl NetCDF {
    pub fn open(filename: String) -> NetCDF {
        return NetCDF {
            filename: filename 
        } 
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_create_a_netcdf_file() {
        let expected = "awesome.nc".to_string();
        let actual = NetCDF::open("awesome.nc".to_string());
        assert_eq!(expected, actual.filename);
    }
}
