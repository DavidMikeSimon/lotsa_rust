use std::{
  collections::{hash_map::DefaultHasher, HashSet},
  fmt,
  hash::{Hash, Hasher},
  marker::PhantomData,
};

use crate::{
  block::{BlockType, UNKNOWN},
  chunk::Chunk,
  chunk_pos::ChunkPos,
  loaded_chunk::LoadedChunk,
  query::{BlockInfo, Cacheability, Context, Query},
  relative_pos::RelativePos,
};

pub struct Simulator {
  updaters: Vec<(BlockType, Box<Updater>)>,
  cacheabilities: HashSet<Cacheability>,
}

pub struct Updater {
  // TODO: Use a builder pattern so that updater_fn doesn't need to be wrapped in Option
  updater_fn: Option<Box<dyn Fn(&UpdaterHandle) -> Option<BlockType>>>,
  cacheability: Cacheability,
}

impl Updater {
  fn new() -> Updater {
    Updater {
      updater_fn: None,
      cacheability: Cacheability::Forever,
    }
  }

  fn run(&self, chunk: &Chunk, pos: ChunkPos) -> Option<BlockType> {
    let handle = UpdaterHandle {
      context: UpdaterContext {
        chunk,
        chunk_pos: pos,
      },
    };
    self.updater_fn.as_ref().unwrap()(&handle)
  }

  pub fn prepare_query<'a, Q, T>(&mut self, query: &Q) -> PreparedQuery<'a, Q, T>
  where
    Q: Query<'a, T>,
  {
    self.cacheability = Cacheability::merge(&self.cacheability, &query.cacheability());
    PreparedQuery::new(query)
  }

  pub fn implement(&mut self, updater_fn: impl Fn(&UpdaterHandle) -> Option<BlockType> + 'static) {
    self.updater_fn = Some(Box::new(updater_fn))
  }
}

pub struct PreparedQuery<'a, Q, T: 'a>
where
  Q: Query<'a, T>,
{
  query: Q,
  hashcode: u64,
  _phantom: PhantomData<&'a T>,
}

impl<'a, Q, T> PreparedQuery<'a, Q, T>
where
  Q: Query<'a, T>,
{
  fn new(query: &Q) -> PreparedQuery<'a, Q, T> {
    let mut hasher = DefaultHasher::new();
    query.unique_descrip().hash(&mut hasher);

    PreparedQuery {
      query: query.clone(),
      hashcode: hasher.finish(),
      _phantom: PhantomData,
    }
  }
}

impl<'a, Q, T> fmt::Debug for PreparedQuery<'a, Q, T>
where
  Q: Query<'a, T>,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "PreparedQuery:{}", self.query.unique_descrip())
  }
}

impl<'a, Q, T> Hash for PreparedQuery<'a, Q, T>
where
  Q: Query<'a, T>,
{
  fn hash<H: Hasher>(&self, state: &mut H) { self.hashcode.hash(state); }
}

pub struct UpdaterHandle<'a> {
  context: UpdaterContext<'a>,
}

impl<'a> UpdaterHandle<'a> {
  pub fn query<Q, T: 'a>(&'a self, linked_query: &'a PreparedQuery<'a, Q, T>) -> T
  where
    Q: Query<'a, T>,
  {
    linked_query.query.eval(&self.context, RelativePos::here())
  }
}

struct UpdaterContext<'a> {
  chunk: &'a Chunk,
  chunk_pos: ChunkPos,
}

impl<'a> Context for UpdaterContext<'a> {
  fn get_block(&self, rel_pos: RelativePos) -> BlockInfo {
    match self.chunk_pos.offset(rel_pos) {
      Some(pos) => self.chunk.get_block(pos),
      None => BlockInfo {
        block_type: UNKNOWN,
      },
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct BlockTypeUpdate {
  pos: ChunkPos,
  block_type: BlockType,
}

impl Simulator {
  pub fn new() -> Simulator {
    Simulator {
      updaters: Vec::new(),
      cacheabilities: HashSet::new(),
    }
  }

  pub fn add_updater(&mut self, target: BlockType, setup_fn: fn(&mut Updater)) {
    let mut updater = Box::new(Updater::new());
    setup_fn(&mut updater);
    self.cacheabilities.insert(updater.cacheability.clone());
    self.updaters.push((target, updater));
  }

  pub fn step(&self, loaded_chunk: &mut LoadedChunk) {
    let mut updates: Vec<BlockTypeUpdate> = Vec::new();

    for (target_block_type, updater) in self.updaters.iter() {
      for (pos, block) in loaded_chunk.considerable_blocks_iter(&updater.cacheability) {
        if target_block_type == &block.block_type {
          if let Some(new_block_type) = updater.run(loaded_chunk.get(), pos) {
            updates.push(BlockTypeUpdate {
              pos,
              block_type: new_block_type,
            });
          }
        }
      }
    }

    loaded_chunk.reset_cache_busters(self.cacheabilities.iter());

    for update in updates {
      loaded_chunk.set_block_type(update.pos, update.block_type);
    }
  }
}
