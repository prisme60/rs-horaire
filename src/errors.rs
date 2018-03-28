error_chain! {
    foreign_links {
        Req(::reqwest::Error);
        Io(::std::io::Error);
    }

    errors { InvalidAnswerError }
}
