#[derive(Debug, Copy, Clone)]
pub struct Config<'a> {
    pub interface_name: &'a str,
    pub rt_index: &'a i32,
    pub route_table_name: &'a str,
    pub network: &'a str,
    pub tunnel_entry: &'a str,
    pub tunnel_exit: &'a str,
}
