#[macro_use]
extern crate stdweb;

use stdweb::web::{IEventTarget, event::KeyDownEvent};
use stdweb::traits::*;

mod renderer;
use renderer::Renderer;

mod game;
use game::Direction;

fn main() {
    stdweb::initialize();

    let renderer = Renderer::new("#mycanvas", 16, 16);
    let snake = std::rc::Rc::new(std::cell::RefCell::new(game::Snake::new(16, 16)));
    snake.borrow().draw(&renderer);
    stdweb::web::document().add_event_listener({
        let snake = snake.clone();
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                "ArrowLeft" => snake.borrow_mut().change_dir(Direction::Left),
                "ArrowRight" => snake.borrow_mut().change_dir(Direction::Right),
                "ArrowDown" => snake.borrow_mut().change_dir(Direction::Down),
                "ArrowUp" => snake.borrow_mut().change_dir(Direction::Up),
                _ => {}
            };
        }
    });

    fn looper(
        snake: std::rc::Rc<std::cell::RefCell<game::Snake>>,
        renderer: std::rc::Rc<Renderer>,
        timer: u32,
    ) {
        stdweb::web::set_timeout(
            move || {
                looper(snake.clone(), renderer.clone(), timer);
                snake.borrow_mut().update();
                snake.borrow().draw(&renderer);
            },
            timer,
        );
    }

    looper(snake, std::rc::Rc::new(renderer), 200);
    stdweb::event_loop();
}
