use bevy::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{Cursor, Position};
use crate::Map;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
    pub recalced: bool,
}

/// Update viewsheds
pub fn update_viewsheds(map: Res<Map>, mut viewsheds: Query<(&Position, &mut Viewshed)>) {
    for (p, mut v) in viewsheds.iter_mut() {
        if v.dirty {
            v.visible_tiles.clear(); // is this necessary?
            v.visible_tiles = field_of_view(Point::new(p.x, p.y), v.range, &*map);
            v.dirty = false;
            v.recalced = true;
        }
    }
}

/// Update revealed terrain on the map based on cursor viewshed
pub fn reveal_map(mut map: ResMut<Map>, mut viewsheds: Query<&mut Viewshed, With<Cursor>>) {
    for mut v in viewsheds.iter_mut() {
        if v.recalced {
            for t in map.terrain_visible.iter_mut() {
                *t = false
            }
            for p in &v.visible_tiles {
                let idx = map.point2d_to_index(*p);
                map.terrain_known[idx] = true;
                map.terrain_visible[idx] = true;
            }
            v.recalced = false;
        }
    }
}
