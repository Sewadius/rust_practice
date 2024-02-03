// Trait as a parameter for function
trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
#[allow(dead_code)]
struct Post {
    title: String,
    author: String,
    content: String
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}

fn main() {
    let post: Post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string()
    };
    let weibo: Weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweer".to_string()
    };

    summary(&post);
    summary(&weibo);

    println!("{:?}", post);
    println!("{:?}", weibo);
}

fn summary(a: &impl Summary) {
    let output: String = a.summarize();
    println!("{}", output);
}
