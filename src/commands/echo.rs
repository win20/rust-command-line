pub fn echo(text: Vec<String>) {
    println!("{}", text.join(" "));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_outputs_input_strings() {}
}
