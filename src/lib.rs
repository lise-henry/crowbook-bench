#![feature(test)]
extern crate test;
extern crate crowbook;
use std::io;
use std::io::Write;
use std::env;

use test::Bencher;
use crowbook::Book;

fn book_en() -> Book {
    // 0.1
    Book::new_from_file("en.book", false).unwrap()
}

fn book_fr() -> Book {
    // 0.1
    Book::new_from_file("fr.book", false).unwrap()
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
    let book = book_en();
    b.iter(|| {
        book.render_all().unwrap()
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
    let book = book_fr();
    b.iter(|| {
        book.render_all().unwrap()
    });
}

