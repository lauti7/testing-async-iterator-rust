use async_stream::stream;
use futures_core::Stream;
use std::time::Duration;
use tokio::time::sleep;
use tokio_stream::{self as stream, StreamExt};

pub struct QueryBooksRequest {
    pub author_prefix: String,
}

#[warn(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct Book {
    author: String,
    title: String,
}

pub struct MyContext {
    hardcoded_database: Vec<Book>,
}

fn generate_books(amount: usize, author: &str) -> Vec<Book> {
    let mut v = vec![];
    for i in 0..amount {
        v.push(Book {
            author: author.to_string(),
            title: format!("Rust advanced: {}", i),
        });
    }

    v
}

impl MyContext {
    pub fn new() -> Self {
        let mut vec1 = generate_books(5, "steve");
        vec1.append(&mut generate_books(5, "jobs"));

        Self {
            hardcoded_database: vec1,
        }
    }
}

pub async fn get_books_by_author(db: &[Book], author: &str) -> impl Stream<Item = Book> {
    let author = String::from(author);
    let db = Vec::from(db);
    stream! {
        for b in db {
            sleep(Duration::from_millis(10)).await;
            if b.author.contains(author.as_str()) {
                yield b;
            }
            sleep(Duration::from_millis(200)).await;
        }
    }
}

async fn filter_book(book: Book, filter: String) -> Book {
    sleep(Duration::from_secs(1)).await;
    if book.author.contains(&filter) {
        return book;
    } else {
        Book::default()
    }
}

pub async fn query_books_1(request: QueryBooksRequest, ctx: MyContext) -> impl Stream<Item = Book> {
    let author_prefix = request.author_prefix.clone();
    let stream = stream::iter(ctx.hardcoded_database)
        .then(move |book| filter_book(book, author_prefix.clone()));

    return stream;
}

pub async fn query_books_2(request: QueryBooksRequest, ctx: MyContext) -> impl Stream<Item = Book> {
    get_books_by_author(&ctx.hardcoded_database, &request.author_prefix).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_books_1_function() {
        let ctx = MyContext::new();
        let query = QueryBooksRequest {
            author_prefix: "steve".to_string(),
        };
        let query_books_gen = query_books_1(query, ctx).await;
        tokio::pin!(query_books_gen);

        while let Some(book) = query_books_gen.next().await {
            println!("My book {:?}", book);
        }
    }

    #[tokio::test]
    async fn test_query_books_2_function() {
        let ctx = MyContext::new();
        let query = QueryBooksRequest {
            author_prefix: "jobs".to_string(),
        };
        let query_books_gen = query_books_2(query, ctx).await;
        tokio::pin!(query_books_gen);

        while let Some(book) = query_books_gen.next().await {
            assert_eq!(book.author, "jobs")
        }
    }
}
