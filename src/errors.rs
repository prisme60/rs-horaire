error_chain! {
    foreign_links {
        Req(::reqwest::Error);
        Io(::std::io::Error);
    }

    errors {
        InvalidAnswerError

        MissingField(t: String) {
            description("Missing field in the answer")
            display("name of missing field: '{}'", t)
        }
    }
}
