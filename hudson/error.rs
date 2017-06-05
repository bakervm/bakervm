error_chain!{
    foreign_links {
        ParseInt(::std::num::ParseIntError);
    }
}
