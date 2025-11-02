macro_rules! problems {
    ($($problem:ident), *) => {
        $(
            mod $problem;
        )*

        pub fn solve(problem: &str) {
            match problem {
                $(
                    stringify!($problem) => println!("{}", $problem::solve()),
                )*
                other => panic!("no solution for problem {other}"),
            }
        }
    }
}

problems!(p0001, p0002, p0003, p0029, p0031);
