#[cfg(test)]
use core::Point3;
use libterrain::{Biome, TerrainChunk, TerrainLoader};

#[test]
fn test_chunk_coord() {
    let chunk_width: i32 = 8;
    let chunk_height: i32 = 8;

    let chunk_wh = chunk_width / 2;
    let chunk_hh = chunk_height / 2;

    let terloader = TerrainLoader::new(chunk_width as u32, chunk_height as u32);

    // Origin point maps to origin chunk
    let coords = vec![(0, 0), (0, 1), (1, 0), (1, 1), (0, -1), (-1, 0), (-1, -1)];

    for coord in coords {
        let pt = Point3::new(coord.0 * chunk_wh, coord.1 * chunk_hh, 0);
        let calcd = terloader.to_chunk_coord(&pt);
        assert_eq!(coord, calcd);
    }
}

#[test]
fn test_is_walkable() {
    let mut terloader = TerrainLoader::new(3, 3);
    let mut chunk = TerrainChunk::new(3, 3);

    // Test a single block high wall, should be passable.
    chunk.set((0, 2, 0), Some(Biome::ROCK));
    chunk.set((1, 2, 0), Some(Biome::ROCK));
    chunk.set((2, 2, 0), Some(Biome::ROCK));
    terloader.chunks.insert((0, 0), chunk);

    let pt = Point3::new(1, 1, 0);
    assert_eq!(terloader.is_walkable(&pt), false);
}

#[test]
fn test_basic_neighbors() {
    let mut terloader = TerrainLoader::new(3, 3);
    let chunk = TerrainChunk::new(3, 3);
    terloader.chunks.insert((0, 0), chunk);

    let neighbors = terloader.neighbors(&Point3::new(0, 0, 0));
    // Since there are no blocks in this test chunk, we should only have 4
    // walkable neighbors at zlevel = 0. Everything else would be considered air.
    assert_eq!(neighbors.len(), 4);
}

#[test]
fn test_passable_neighbors() {
    let mut terloader = TerrainLoader::new(3, 3);
    let mut chunk = TerrainChunk::new(3, 3);

    // Test a single block high wall, should be passable.
    // NOTE: This are chunk coordinates which have been translated from the woorld
    // coordinates.
    chunk.set((0, 2, 0), Some(Biome::ROCK));
    chunk.set((1, 2, 0), Some(Biome::ROCK));
    chunk.set((2, 2, 0), Some(Biome::ROCK));
    terloader.chunks.insert((0, 0), chunk);

    let neighbors = terloader.neighbors(&Point3::new(1, 0, 0));
    assert_eq!(neighbors.len(), 3);
    assert_eq!(neighbors[0].0, Point3::new(1, -1, 0));
    assert_eq!(neighbors[1].0, Point3::new(0, 0, 0));
    // assert_eq!(neighbors[2].0, Point3::new(2, 0, 0));
    // On top of the wall, right in front.
    assert_eq!(neighbors[2].0, Point3::new(1, 1, 1));
}

#[test]
fn test_blocked_neighbors() {
    let mut terloader = TerrainLoader::new(3, 3);
    let mut chunk = TerrainChunk::new(3, 3);

    // Test a chunk with a two block high wall
    chunk.set((0, 2, 0), Some(Biome::ROCK));
    chunk.set((1, 2, 0), Some(Biome::ROCK));
    chunk.set((2, 2, 0), Some(Biome::ROCK));
    chunk.set((0, 2, 1), Some(Biome::ROCK));
    chunk.set((1, 2, 1), Some(Biome::ROCK));
    chunk.set((2, 2, 1), Some(Biome::ROCK));
    terloader.chunks.insert((0, 0), chunk);

    // Since there is a wall in the way, we should only get these two points
    let neighbors = terloader.neighbors(&Point3::new(1, 0, 0));
    assert_eq!(neighbors.len(), 2);
    assert_eq!(neighbors[0].0, Point3::new(1, -1, 0));
    assert_eq!(neighbors[1].0, Point3::new(0, 0, 0));
}
