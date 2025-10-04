use bracket_lib::prelude::*; //Game Engine... https://github.com/amethyst/bracket-lib

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are Dead!");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit!");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY); //let you specify the background color.
        self.frame_time += ctx.frame_time_ms; //The tick() function runs as fast as it can—often
                                              //60 or more times per sec-ond.
                                              //Your player doesn’t have superhuman reflexes,
                                              //so you need to slow the game down.
                                              //The context provides a variable named frame_time_ms
                                              //con-taining the time elapsed since the last time tick()
                                              //was called. Add this to your state’s frame_time.
                                              //If it exceeds the FRAME_DURATION constant, then it’s time
                                              //to run the physics simulation and reset your
                                              //frame_time to zero.

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            //SPACEBAR, call the flap() function
            self.player.flap();
        }

        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to Flap or Fap...");
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        //Main Menu... i think...
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            //extract the key’s value into key.

            match key {
                VirtualKeyCode::P => self.restart(), //calling the restart() function.
                VirtualKeyCode::Q => ctx.quitting = true, //bracket-lib terminate program.
                _ => {}
            }
        }
    }
}

impl GameState for State {
    //This is similar to implementing functions for a structure

    fn tick(&mut self, ctx: &mut BTerm) {
        //GameState requiresu implement a function named tick()

        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

enum GameMode {
    Menu, //Each possible game state is enumerated as an entry
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

struct Player {
    x: i32,        //The x position of the player.
    y: i32,        //The vertical position of the player in screen-space.
    velocity: f32, //The player’s vertical velocity.
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0, //Floating-point types (f32) must be fractional.
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, TOMATO, BLACK, to_cp437('@'));
        { //‘bracket-lib‘ function that sets a single
             //character on the screen.
             //The function to_cp437() converts a Unicode symbol
             //from your source code to the matching Codepage
             //437 character number.
        }
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            //Check for terminal velocity: only apply gravity
            //if the downward momentum is less than two.

            self.velocity += 0.2; //Adding the current velocity moves the player up or down.
        }

        self.y += self.velocity as i32; //Add the velocity to the player’s y position. Rust won’t
                                        //let you add floating-point and integer numbers together,
                                        //so you have to convert the velocity to an integer with an i32.
                                        //This number will always round down.

        self.x += 1;

        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0; //sets the player’s velocity to -2.0. It’s a negative number,
                              //so this will move the character upward
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50() //Start by requesting an 80x50 terminal.
        .with_title("Flappy Dragon") //Request that the window be titled “Flappy Dragon.”
        .build()?; //The ? operator because main() to return BError.

    main_loop(context, State::new())
}
