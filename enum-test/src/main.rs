fn main() {
    #[derive(Debug)]
    enum MusicPlayer {
        Spotify,
        YTM,
    }
    let x = MusicPlayer::YTM;
    println!("{:?}", x);
}
