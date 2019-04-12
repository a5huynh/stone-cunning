use termion::{clear, style, cursor};
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{self, Write, Read};

use libdwarf::{
    objects::{ MapObject },
    tasks::Action,
    world::{ WorldSim }
};


struct AsciiRenderer<R, W: Write> {
    stdout: W,
    stdin: R,
    num_ticks: u16,
}

impl<R: Read, W: Write> AsciiRenderer<R, W> {
    pub fn new(stdin: R, stdout: W) -> AsciiRenderer<R, RawTerminal<W>> {
        AsciiRenderer {
            stdout: stdout.into_raw_mode().unwrap(),
            stdin: stdin,
            num_ticks: 0,
        }
    }

    pub fn render(&mut self, world: &WorldSim) {
        write!(self.stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
        write!(self.stdout, "num ticks: {}\n\r", self.num_ticks).unwrap();
        // Render world.
        write!(self.stdout, "{}", world).unwrap();

        // Render workers & worker status
        write!(self.stdout, "\n\rWorkers\n\r--------------\n\r").unwrap();
        for worker in world.workers.iter() {
            for action in worker.actions.iter() {
                write!(self.stdout, "- {:?}\n\r", action).unwrap();
            }
        }

        // Render objects & object queue
        write!(self.stdout, "\n\rObjects\n\r--------------\n\r").unwrap();
        for (pos, object) in world.objects.iter() {
            write!(self.stdout, "{:?} {}\n\r", pos, object.id).unwrap();
            for action in object.actions.iter() {
                write!(self.stdout, "- {:?}\n\r", action).unwrap();
            }
        }

        // Render command prompt
        write!(self.stdout, "\n\r").unwrap();
        self.stdout.flush().unwrap();
    }
}

impl<R, W: Write> Drop for AsciiRenderer<R, W> {
    fn drop(&mut self) {
        write!(self.stdout, "{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1)).unwrap();
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut renderer = AsciiRenderer::new(stdin.lock(), stdout.lock());

    let mut world = WorldSim::new(10, 10);
    // Add a tree to the world
    let tree = MapObject::new(1, 9, 9);
    world.add_object(tree.clone());
    // Add a worker to the world
    world.add_worker(0, 0, 0);
    // Add a job
    world.add_task(
        Action::HarvestResource(
            (9, 9),
            "wood".to_string(),
            tree.id
        ),
    );

    loop {
        // Render map
        renderer.render(&world);
        // Get input and handle action
        let mut b = [0];
        renderer.stdin.read(&mut b).unwrap();
        match b[0] {
            // quit
            b'q' => return,
            // Tick map
            b'.' => {
                renderer.num_ticks += 1;
                // Tick map
                world.tick();
            },
            _ => {}
        }

        renderer.stdout.flush().unwrap();
    }
}
