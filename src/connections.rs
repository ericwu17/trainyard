use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use crate::direction::Dir;

/// A struct representing possible active and passive connections on a track, with the data represented as a single byte.
///
/// Conceptually, this struct packs 4 `Dir`s into a single byte, and assigns meaning to each dir.
///
/// The active connection is represented in the least significant 4 bits, and the passive connection is represented in the most significant 4 bits.
/// Each connection is composed of two `Dir`s
/// A value of 0 in both dirs for either connection represents the lack of connection.
#[derive(Component, Default)]
pub struct TileConnections {
    data: u8,
}

// A struct representing a single connection
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Connection {
    data: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ConnectionType {
    None,
    I,
    C,
    H,
    Z,
    M,
    Jc,
    Ji,
}

impl ConnectionType {
    pub fn get_asset_path(&self) -> &'static str {
        match self {
            ConnectionType::None => "sprites/Tracktile_blank.png",
            ConnectionType::I => "sprites/Tracktile_i.png",
            ConnectionType::C => "sprites/Tracktile_c.png",
            ConnectionType::H => "sprites/Tracktile_h.png",
            ConnectionType::Z => "sprites/Tracktile_z.png",
            ConnectionType::M => "sprites/Tracktile_m.png",
            ConnectionType::Jc => "sprites/Tracktile_jc.png",
            ConnectionType::Ji => "sprites/Tracktile_ji.png",
        }
    }
}

impl TileConnections {
    pub fn empty() -> Self {
        TileConnections { data: 0 }
    }
    pub fn from_active(active: Connection) -> Self {
        Self { data: active.data }
    }
    pub fn from_active_passive(active: Connection, passive: Connection) -> Self {
        Self {
            data: active.data | (passive.data << 4),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data == 0
    }
    pub fn get_active_conn(&self) -> Connection {
        Connection {
            data: self.data & 0x0f,
        }
    }
    pub fn get_passive_conn(&self) -> Connection {
        Connection {
            data: (self.data >> 4) & 0x0f,
        }
    }

    pub fn add_connection(&self, d1: Dir, d2: Dir) -> Self {
        let conn_new = Connection::from_dirs(d1, d2).data;
        let conn_old = self.data & 0x0f;

        if conn_new == conn_old {
            return TileConnections { data: conn_old };
        }

        TileConnections {
            data: (conn_old << 4) | conn_new,
        }
    }

    pub fn to_normal_form(&self) -> Self {
        let conn_active = self.get_active_conn().to_normal_form();
        let conn_passive = self.get_passive_conn().to_normal_form();

        if conn_active == conn_passive {
            Self::from_active(conn_active)
        } else {
            Self::from_active_passive(conn_active, conn_passive)
        }
    }

    pub fn type_and_rotation(&self) -> (ConnectionType, Quat) {
        if self.data == 0x00 {
            return (ConnectionType::None, Quat::from_rotation_z(0.0));
        }
        if (self.data >> 4) & 0xf == 0 {
            let rotation_for_type_i =
                self.has_connection_up_to_rot(Connection::from_dirs(Dir::Up, Dir::Down));
            if rotation_for_type_i != -1 {
                return (
                    ConnectionType::I,
                    Quat::from_rotation_z(rotation_for_type_i as f32 * FRAC_PI_2),
                );
            } else {
                let rotation_for_type_c =
                    self.has_connection_up_to_rot(Connection::from_dirs(Dir::Left, Dir::Down));
                return (
                    ConnectionType::C,
                    Quat::from_rotation_z(rotation_for_type_c as f32 * FRAC_PI_2),
                );
            }
        }
        // now we can assume that there is both an active and passive connection
        if self.has_connections(
            Connection::from_dirs(Dir::Up, Dir::Down),
            Connection::from_dirs(Dir::Left, Dir::Right),
        ) {
            if self.get_active_conn() == Connection::from_dirs(Dir::Up, Dir::Down) {
                return (ConnectionType::H, Quat::from_rotation_z(0.0));
            } else {
                return (ConnectionType::H, Quat::from_rotation_z(FRAC_PI_2));
            }
        }
        let rot_for_z = self.has_connections_up_to_rot(
            Connection::from_dirs(Dir::Up, Dir::Right),
            Connection::from_dirs(Dir::Down, Dir::Left),
        );
        if rot_for_z != -1 {
            return (
                ConnectionType::Z,
                Quat::from_rotation_z(rot_for_z as f32 * FRAC_PI_2),
            );
        }

        let rot_for_m = self.has_connections_up_to_rot(
            Connection::from_dirs(Dir::Left, Dir::Down),
            Connection::from_dirs(Dir::Right, Dir::Down),
        );
        if rot_for_m != -1 {
            let flip_quat = if self.get_active_conn().rotate_ccw() == self.get_passive_conn() {
                Quat::IDENTITY
            } else {
                Quat::from_rotation_y(PI)
            };
            return (
                ConnectionType::M,
                Quat::from_rotation_z(rot_for_m as f32 * FRAC_PI_2) * flip_quat,
            );
        }

        let rot_for_j_not_flipped = self.has_connections_up_to_rot(
            Connection::from_dirs(Dir::Down, Dir::Left),
            Connection::from_dirs(Dir::Up, Dir::Down),
        );
        if rot_for_j_not_flipped != -1 {
            let conn_type = if self.get_active_conn() == Connection::from_dirs(Dir::Up, Dir::Down) {
                ConnectionType::Ji
            } else {
                ConnectionType::Jc
            };
            return (
                conn_type,
                Quat::from_rotation_z(rot_for_j_not_flipped as f32 * FRAC_PI_2),
            );
        }

        let rot_for_j_flipped = self.has_connections_up_to_rot(
            Connection::from_dirs(Dir::Down, Dir::Right),
            Connection::from_dirs(Dir::Up, Dir::Down),
        );
        if rot_for_j_flipped != -1 {
            let conn_type = if self.get_active_conn() == Connection::from_dirs(Dir::Up, Dir::Down) {
                ConnectionType::Ji
            } else {
                ConnectionType::Jc
            };
            return (
                conn_type,
                Quat::from_rotation_z(rot_for_j_flipped as f32 * FRAC_PI_2)
                    * Quat::from_rotation_y(PI),
            );
        }

        unreachable!()
    }

    pub fn rotate_cw(&self) -> Self {
        Self::from_active_passive(
            self.get_active_conn().rotate_cw(),
            self.get_passive_conn().rotate_cw(),
        )
    }
    pub fn rotate_ccw(&self) -> Self {
        Self::from_active_passive(
            self.get_active_conn().rotate_ccw(),
            self.get_passive_conn().rotate_ccw(),
        )
    }
    pub fn flip(&self) -> Self {
        Self::from_active_passive(
            self.get_active_conn().flip(),
            self.get_passive_conn().flip(),
        )
    }

    pub fn has_connection_up_to_rot(&self, c: Connection) -> i8 {
        // returns -1 if there is no connection, otherwise returns the rotation amount
        let mut active_conn = self.get_active_conn();
        let mut rot = 0;
        if !active_conn.is_empty() {
            while rot < 4 {
                if active_conn == c {
                    return rot;
                }
                rot += 1;
                active_conn = active_conn.rotate_cw();
            }
        }
        let mut passive_conn = self.get_passive_conn();
        let mut rot = 0;
        if !passive_conn.is_empty() {
            while rot < 4 {
                if passive_conn == c {
                    return rot;
                }
                rot += 1;
                passive_conn = passive_conn.rotate_cw();
            }
        }
        -1
    }

    pub fn has_connections_up_to_rot(&self, c1: Connection, c2: Connection) -> i8 {
        // returns true iff self has both an active and passive connection,
        // and the connections match c1 and c2 (regardless of active/passive)
        // after being rotated a fixed amount
        let mut active = self.get_active_conn();
        let mut passive = self.get_passive_conn();
        if !active.is_empty() && !passive.is_empty() {
            let mut rot = 0;
            while rot < 4 {
                if (active == c1 && passive == c2) || (active == c2 && passive == c1) {
                    return rot as i8;
                }
                rot += 1;
                active = active.rotate_cw();
                passive = passive.rotate_cw();
            }
        }
        -1
    }

    pub fn has_connections(&self, c1: Connection, c2: Connection) -> bool {
        // returns true iff self has both an active and passive connection,
        // and the connections match c1 and c2 (regardless of active/passive)
        let active = self.get_active_conn();
        let passive = self.get_passive_conn();
        (active == c1 && passive == c2) || (active == c2 && passive == c1)
    }
}

impl Connection {
    pub fn from_dirs(d1: Dir, d2: Dir) -> Self {
        Connection {
            data: (u8::from(d1) << 2) | u8::from(d2),
        }
        .to_normal_form()
    }

    /// This function converts a connection to a "normalized form",
    /// since the same connection could be represented by multiple u8 values
    /// (0x08 is the same as 0x02 since those both represent Up connected to Down on the active connection)
    ///
    /// We normalize by having the connection of lesser value be in the least significant bits.
    pub fn to_normal_form(&self) -> Self {
        let dir_1 = self.data & 0x03;
        let dir_2 = (self.data >> 2) & 0x03;

        match dir_1.cmp(&dir_2) {
            std::cmp::Ordering::Equal => Connection { data: 0 },
            std::cmp::Ordering::Less => Connection {
                data: dir_1 | (dir_2 << 2),
            },
            std::cmp::Ordering::Greater => Connection {
                data: dir_2 | (dir_1 << 2),
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data == 0
    }

    pub fn rotate_cw(&self) -> Self {
        let d1 = self.data & 0x03;
        let d2 = (self.data >> 2) & 0x03;
        let d1_new = Dir::rotate_cw_u8(d1);
        let d2_new = Dir::rotate_cw_u8(d2);
        Self {
            data: (d2_new << 2) | d1_new,
        }
        .to_normal_form()
    }
    pub fn rotate_ccw(&self) -> Self {
        let d1 = self.data & 0x03;
        let d2 = (self.data >> 2) & 0x03;
        let d1_new = Dir::rotate_ccw_u8(d1);
        let d2_new = Dir::rotate_ccw_u8(d2);
        Self {
            data: (d2_new << 2) | d1_new,
        }
        .to_normal_form()
    }
    pub fn flip(&self) -> Self {
        let d1 = self.data & 0x03;
        let d2 = (self.data >> 2) & 0x03;
        let d1_new = Dir::flip_u8(d1);
        let d2_new = Dir::flip_u8(d2);
        Self {
            data: (d2_new << 2) | d1_new,
        }
        .to_normal_form()
    }
}
