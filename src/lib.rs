#![feature(test)]
extern crate test;
extern crate crowbook;
use std::io;
use std::io::Write;

use test::Bencher;
use crowbook::Book;
// >= 0.4.0
use crowbook::InfoLevel;

fn book_en() -> Book {
    // < 0.4.0
    // Book::new_from_file("en.book", false).unwrap()
    // = 0.4.0
    // Book::new_from_file("en.book", InfoLevel::Error).unwrap()
    // >= 0.5.0
    Book::new_from_file("en.book", InfoLevel::Error, &[]).unwrap()
}

fn book_fr() -> Book {
    // < 0.4.0
    // Book::new_from_file("fr.book", false).unwrap()
    // = 0.4.0
    // Book::new_from_file("fr.book", InfoLevel::Error).unwrap()
    // >= 0.5.0
    Book::new_from_file("fr.book", InfoLevel::Error, &[]).unwrap()
}

#[bench]
fn bench_parse_book(b: &mut Bencher) {
    b.iter(|| {
        book_en();
    });
}


#[bench]
fn bench_en_html(b: &mut Bencher) {
    let book = book_en();
    b.iter(|| {
        book.render_html(&mut io::sink()).unwrap()
    });
}

#[bench]
fn bench_en_epub(b: &mut Bencher) {
    let book = book_en();
    b.iter(|| {
        book.render_epub().unwrap()
    });
}

#[bench]
fn bench_en_tex(b: &mut Bencher) {
    let book = book_en();
    b.iter(|| {
        book.render_tex(&mut io::sink()).unwrap()
    });
}

#[bench]
fn bench_en_all(b: &mut Bencher) {
    b.iter(|| {
        let book = book_en();
        // < 0.4.0
        //        book.render_all().unwrap()
        book.render_all()
    });
}


#[bench]
fn bench_fr_html(b: &mut Bencher) {
    let book = book_fr();
    b.iter(|| {
        book.render_html(&mut io::sink()).unwrap()
    });
}

#[bench]
fn bench_fr_epub(b: &mut Bencher) {
    let book = book_fr();
    b.iter(|| {
        book.render_epub().unwrap()
    });
}


#[bench]
fn bench_fr_tex(b: &mut Bencher) {
    let book = book_fr();
    b.iter(|| {
        book.render_tex(&mut io::sink()).unwrap()
    });
}


#[bench]
fn bench_fr_all(b: &mut Bencher) {
    b.iter(|| {
        let book = book_fr();
        // < 0.4.0
        // book.render_all().unwrap()
        book.render_all()
    });
}

