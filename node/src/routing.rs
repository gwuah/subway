use crate::config::Config;
use crate::errors::SubwayError;
use crate::SUBWAY_PACKET_MARK;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Router {
    cfg: Config<'static>,
}

impl Router {
    pub fn new(cfg: Config<'static>) -> Router {
        return Router { cfg };
    }

    pub fn configure_nat_rules(&self) -> Result<(), SubwayError> {
        let _ = Command::new("iptables")
            .args(&[
                "-t",
                "nat",
                "-A",
                "POSTROUTING",
                "-s",
                self.cfg.network,
                "-j",
                "MASQUERADE",
            ])
            .output()
            .map_err(SubwayError::IOError)?;
        Ok(())
    }

    fn route_table_exists(&self) -> Result<bool, SubwayError> {
        let data = fs::read_to_string("/etc/iproute2/rt_tables").map_err(SubwayError::IOError)?;
        Ok(data.contains(self.cfg.route_table_name))
    }

    pub fn configure_route_table(&self) -> Result<(), SubwayError> {
        if self.route_table_exists()? {
            self.detach_route_table()?;
        }

        // add a new route table to the master routing table
        // i dont want to mess up existing routes on my computer.
        // echo 100 custom >> /etc/iproute2/rt_tables
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("/etc/iproute2/rt_tables")
            .map_err(SubwayError::IOError)?;
        let _ = file
            .write_all(format!("{} {}\n", self.cfg.rt_index, self.cfg.route_table_name).as_bytes())
            .map_err(SubwayError::IOError)?;

        // use tun interface as default route for table
        // ip route add default dev wg0 table 2468
        let _ = Command::new("ip")
            .args(&[
                "route",
                "add",
                "default",
                "dev",
                self.cfg.interface_name,
                "table",
                &self.cfg.route_table_name,
            ])
            .output()
            .map_err(SubwayError::IOError)?;

        // add a new ip rule to lookup our table
        // ip rule add not fwmark 0x50000 lookup ittoc
        let _ = Command::new("ip")
            .args(&[
                "rule",
                "add",
                "not",
                "fwmark",
                &SUBWAY_PACKET_MARK.to_string(),
                "table",
                &self.cfg.route_table_name,
            ])
            .output()
            .map_err(SubwayError::IOError)?;

        // add a new ip rule to lookup our table
        // ip rule add table main suppress_prefixlength 0
        let _ = Command::new("ip")
            .args(&["rule", "add", "table", "main", "suppress_prefixlength", "0"])
            .output()
            .map_err(SubwayError::IOError)?;

        // flush kernel cache so new rules are loaded
        // ip route flush cache
        let _ = Command::new("ip")
            .args(&["route", "flush", "cache"])
            .output()
            .map_err(SubwayError::IOError)?;

        Ok(())
    }

    pub fn detach_route_table(&self) -> Result<(), SubwayError> {
        // ip rule del not fwmark 0x50000 lookup ittoc
        let _ = Command::new("ip")
            .args(&[
                "rule",
                "del",
                "not",
                "fwmark",
                &SUBWAY_PACKET_MARK.to_string(),
                "table",
                &self.cfg.route_table_name,
            ])
            .output()
            .map_err(SubwayError::IOError)?;

        // ip rule del table main suppress_prefixlength 0
        let _ = Command::new("ip")
            .args(&["rule", "del", "table", "main", "suppress_prefixlength", "0"])
            .output()
            .map_err(SubwayError::IOError)?;

        // use sed to replace line in master table
        let _ = Command::new("sed")
            .args(&[
                "-i",
                &format!(r"/{} {}/c\", &self.cfg.rt_index, &self.cfg.route_table_name,),
                "/etc/iproute2/rt_tables",
            ])
            .output()
            .map_err(SubwayError::IOError)?;

        // flush kernel cache so new rules are loaded
        // ip route flush cache
        let _ = Command::new("ip")
            .args(&["route", "flush", "cache"])
            .output()
            .map_err(SubwayError::IOError)?;

        Ok(())
    }

    pub fn up(&self, name: &str) -> Result<(), SubwayError> {
        Command::new("ip")
            .args(&["link", "set", "up", "dev", name])
            .output()
            .map_err(SubwayError::IOError)?;
        Ok(())
    }

    pub fn down(&self, name: &str) -> Result<(), SubwayError> {
        let _ = Command::new("ip")
            .args(&["link", "set", "down", "dev", name])
            .output()
            .map_err(SubwayError::IOError)?;
        Ok(())
    }

    pub fn delete(&self, name: &str) -> Result<(), SubwayError> {
        let _ = Command::new("ip")
            .args(&["link", "del", "dev", name])
            .output()
            .map_err(SubwayError::IOError)?;
        Ok(())
    }

    pub fn set_network(&self, name: &str, network: &str) -> Result<(), SubwayError> {
        let _ = Command::new("ip")
            .args(&["addr", "add", network, "dev", name])
            .output()
            .map_err(SubwayError::IOError)?;
        Ok(())
    }
}
