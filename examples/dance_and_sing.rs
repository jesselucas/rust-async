// https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html

// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
use futures::executor::block_on;

type Song = String;

async fn learn_song(song_title: &str) -> Song {
    println!("Finished learning {}", song_title);
    song_title.to_string()
}

async fn sing_song(song: Song) {
    println!("Finished singing {}", song);
}

async fn dance() {
    println!("I'm dancing!");
}

async fn learn_and_sing(song_title: &str) {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song(song_title).await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing("'Let's Dance' by David Bowie");
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}

fn main() {
    // Sing then dance
    let song = block_on(learn_song("'Thirteen' by Big Star"));
    block_on(sing_song(song));
    block_on(dance());

    // Dance and sing at the same time
    block_on(async_main());
}
