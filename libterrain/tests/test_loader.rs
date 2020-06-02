#[cfg(test)]
use core::{Point3, WorldPos};
use libterrain::{Biome, ChunkEntity, ChunkPos, TerrainChunk, TerrainLoader};

#[test]
fn test_chunk_coord() {
    let chunk_width: i32 = 8;
    let chunk_height: i32 = 8;

    let terloader = TerrainLoader::new(chunk_width as u32, chunk_height as u32);
    // Origin point maps to origin chunk
    let coords = vec![(0, 0), (0, 1), (1, 0), (1, 1), (0, -1), (-1, 0), (-1, -1)];

    for coord in coords {
        let pt = Point3::new(
            coord.0 * chunk_width,
            coord.1 * chunk_height,
            0,
        );
        let calcd = terloader.to_chunk_coord(&pt);
        assert_eq!(coord, calcd);
    }
}

#[test]
fn test_world_to_chunk() {
    let terloader = TerrainLoader::new(64, 64);

    assert_eq!(
        terloader.world_to_chunk(&WorldPos::new(0, 0, 0)),
        ChunkPos::new(0, 0, 0)
    );

    // On the border of the next chunk over.
    assert_eq!(
        terloader.world_to_chunk(&WorldPos::new(64, 64, 0)),
        ChunkPos::new(0, 0, 0)
    );

    // Chunks in the negative world pos.
    assert_eq!(
        terloader.world_to_chunk(&WorldPos::new(-1, -1, 0)),
        ChunkPos::new(63, 63, 0)
    );
}

#[test]
fn test_is_walkable() {
    let mut terloader = TerrainLoader::new(3, 3);
    let mut chunk = TerrainChunk::new(3, 3);

    // Test a single block high wall, should be passable.
    chunk.set(&ChunkPos::new(0, 2, 0), ChunkEntity::Terrain(Biome::ROCK));
    chunk.set(&ChunkPos::new(1, 2, 0), ChunkEntity::Terrain(Biome::ROCK));
    chunk.set(&ChunkPos::new(2, 2, 0), ChunkEntity::Terrain(Biome::ROCK));
    terloader.chunks.insert((0, 0), chunk);

    let pt = WorldPos::new(1, 2, 0);
    assert_eq!(terloader.is_walkable(&pt), false);
}

#[test]
fn test_basic_neighbors() {
    let mut terloader = TerrainLoader::new(3, 3);
    let chunk = TerrainChunk::new(3, 3);
    terloader.chunks.insert((0, 0), chunk);

    let neighbors = terloader.neighbors(&WorldPos::new(1, 1, 0));
    // Since there are no blocks in this test chunk, we should only have 4
    // walkable neighbors at zlevel = 0. Everything else would be considered air.
    assert_eq!(neighbors.len(), 4);
}

#[test]
fn test_passable_neighbors() {
    let mut terloader = TerrainLoader::new(3, 3);
    let mut chunk = TerrainChunk::new(3, 3);

    // Test a single block high wall, should be passable.
    chunk.set(&ChunkPos::new(0, 2, 0), ChunkEntity::Terrain(Biome::ROCK));
    chunk.set(&ChunkPos::new(1, 2, 0), ChunkEntity::Terrain(Biome::ROCK));
    chunk.set(&ChunkPos::new(2, 2, 0), ChunkEntity::Terrain(Biome::ROCK));
    terloader.chunks.insert((0, 0), chunk);

    let neighbors = terloader.neighbors(&WorldPos::new(1, 1, 0));
    assert_eq!(neighbors.len(), 4);
    assert_eq!(neighbors[0].0, WorldPos::new(1, 0, 0));
    assert_eq!(neighbors[1].0, WorldPos::new(0, 1, 0));
    assert_eq!(neighbors[2].0, WorldPos::new(2, 1, 0));
    // On top of the wall, right in front.
    assert_eq!(neighbors[3].0, WorldPos::new(1, 2, 1));
}

#[test]
fn test_blocked_neighbors() {
    let mut terloader = TerrainLoader::new(3, 3);
    let mut chunk = TerrainChunk::new(3, 3);

    // Test a chunk with a two block high wall
    chunk.set(&ChunkPos::new(0, 2, 0), ChunkEntity::Terrain(Biome::ROCK));
    chunk.set(&ChunkPos::new(1, 2, 0), ChunkEntity::Terrain(Biome::ROCK));
    chunk.set(&ChunkPos::new(2, 2, 0), ChunkEntity::Terrain(Biome::ROCK));
    chunk.set(&ChunkPos::new(0, 2, 1), ChunkEntity::Terrain(Biome::ROCK));
    chunk.set(&ChunkPos::new(1, 2, 1), ChunkEntity::Terrain(Biome::ROCK));
    chunk.set(&ChunkPos::new(2, 2, 1), ChunkEntity::Terrain(Biome::ROCK));
    terloader.chunks.insert((0, 0), chunk);

    // Since there is a wall in the way, we should only get 3 points
    let neighbors = terloader.neighbors(&Point3::new(1, 1, 0));
    assert_eq!(neighbors.len(), 3);
    assert_eq!(neighbors[0].0, Point3::new(1, 0, 0));
    assert_eq!(neighbors[1].0, Point3::new(0, 1, 0));
    assert_eq!(neighbors[2].0, Point3::new(2, 1, 0));
}
