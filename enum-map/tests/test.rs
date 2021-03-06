#[macro_use]
extern crate enum_map;

use enum_map::{EnumMap, Internal, IntoIter};

use std::cell::RefCell;
use std::collections::HashSet;
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug, EnumMap, PartialEq)]
enum Example {
    A,
    B,
    C,
}

#[test]
fn test_bool() {
    let mut map = enum_map! { false => 24, true => 42 };
    assert_eq!(map[false], 24);
    assert_eq!(map[true], 42);
    map[false] += 1;
    assert_eq!(map[false], 25);
    for (key, item) in &mut map {
        if !key {
            *item += 1;
        }
    }
    assert_eq!(map[false], 26);
    assert_eq!(map[true], 42);
}

#[test]
fn test_option_bool() {
    let mut map = enum_map! { None => 1, Some(false) => 2, Some(true) => 3};
    assert_eq!(map[None], 1);
    assert_eq!(map[Some(false)], 2);
    assert_eq!(map[Some(true)], 3);
    map[None] = 4;
    map[Some(false)] = 5;
    map[Some(true)] = 6;
    assert_eq!(map[None], 4);
    assert_eq!(map[Some(false)], 5);
    assert_eq!(map[Some(true)], 6);
    assert_eq!(map.as_slice(), [4, 5, 6]);
    let expected = [(None, 4), (Some(false), 5), (Some(true), 6)];
    assert_eq!(map.into_iter().collect::<Vec<_>>(), expected);
}

#[test]
fn test_clone() {
    let map = enum_map! { false => 3, true => 5 };
    assert_eq!(map.clone(), map);
}

#[test]
fn test_debug() {
    let map = enum_map! { false => 3, true => 5 };
    assert_eq!(format!("{:?}", map), "{false: 3, true: 5}");
}

#[test]
fn test_hash() {
    let map = enum_map! { false => 3, true => 5 };
    let mut set = HashSet::new();
    set.insert(map);
    assert!(set.contains(&map));
}

#[test]
fn discriminants() {
    #[derive(Debug, EnumMap, PartialEq)]
    enum Discriminants {
        A = 2000,
        B = 3000,
        C = 1000,
    }
    let mut map = EnumMap::new();
    map[Discriminants::A] = 3;
    map[Discriminants::B] = 2;
    map[Discriminants::C] = 1;
    let mut pairs = map.iter();
    assert_eq!(pairs.next(), Some((Discriminants::A, &3)));
    assert_eq!(pairs.next(), Some((Discriminants::B, &2)));
    assert_eq!(pairs.next(), Some((Discriminants::C, &1)));
    assert_eq!(pairs.next(), None);
}

#[test]
fn extend() {
    let mut map = enum_map! { _ => 0 };
    map.extend(vec![(Example::A, 3)]);
    map.extend(vec![(&Example::B, &4)]);
    assert_eq!(
        map,
        enum_map! { Example:: A => 3, Example:: B => 4, Example::C => 0 }
    );
}

#[test]
fn huge_enum() {
    #[derive(EnumMap)]
    enum Example {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
        Aa,
        Bb,
        Cc,
        Dd,
        Ee,
        Ff,
        Gg,
        Hh,
        Ii,
        Jj,
        Kk,
        Ll,
        Mm,
        Nn,
        Oo,
        Pp,
        Qq,
        Rr,
        Ss,
        Tt,
        Uu,
        Vv,
        Ww,
        Xx,
        Yy,
        Zz,
    }

    let map = enum_map! { _ => 2 };
    assert_eq!(map[Example::Xx], 2);
}

#[test]
fn iterator_len() {
    assert_eq!(
        enum_map! { Example::A | Example::B | Example::C => 0 }
            .iter()
            .len(),
        3
    );
}

#[test]
fn iter_mut_len() {
    assert_eq!(
        enum_map! { Example::A | Example::B | Example::C => 0 }
            .iter_mut()
            .len(),
        3
    );
}

#[test]
fn into_iter_len() {
    assert_eq!(enum_map! { Example::A | _ => 0 }.into_iter().len(), 3);
}

#[test]
fn iterator_next_back() {
    assert_eq!(
        enum_map! { Example::A => 1, Example::B => 2, Example::C => 3 }
            .iter()
            .next_back(),
        Some((Example::C, &3))
    );
}

#[test]
fn iter_mut_next_back() {
    assert_eq!(
        enum_map! { Example::A => 1, Example::B => 2, Example::C => 3 }
            .iter_mut()
            .next_back(),
        Some((Example::C, &mut 3))
    );
}

#[test]
fn into_iter() {
    let mut iter = enum_map! { true => 5, false => 7 }.into_iter();
    assert_eq!(iter.next(), Some((false, 7)));
    assert_eq!(iter.next(), Some((true, 5)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn into_iter_u8() {
    assert_eq!(
        EnumMap::from(|i: u8| i).into_iter().collect::<Vec<_>>(),
        (0..256).map(|x| (x as u8, x as u8)).collect::<Vec<_>>()
    );
}

struct DropReporter<'a> {
    into: &'a RefCell<Vec<usize>>,
    value: usize,
}

impl<'a> Drop for DropReporter<'a> {
    fn drop(&mut self) {
        self.into.borrow_mut().push(self.value);
    }
}

#[test]
fn into_iter_drop() {
    let dropped = RefCell::new(Vec::new());
    let mut a: IntoIter<Example, _> = enum_map! {
        k => DropReporter {
            into: &dropped,
            value: k as usize,
        },
    }.into_iter();
    assert_eq!(a.next().unwrap().0, Example::A);
    assert_eq!(*dropped.borrow(), &[0]);
    drop(a);
    assert_eq!(*dropped.borrow(), &[0, 1, 2]);
}

#[test]
fn test_u8() {
    let mut map = enum_map! { b'a' => 4, _ => 0 };
    map[b'c'] = 3;
    assert_eq!(map[b'a'], 4);
    assert_eq!(map[b'b'], 0);
    assert_eq!(map[b'c'], 3);
    assert_eq!(map.iter().next(), Some((0, &0)));
}

#[derive(EnumMap)]
enum Void {}

#[test]
fn empty_map() {
    let void: EnumMap<Void, Void> = enum_map!{};
    assert!(void.is_empty());
}

#[test]
#[should_panic]
fn empty_value() {
    let _void: EnumMap<bool, Void> = enum_map! { _ => unreachable!() };
}

#[derive(Clone, Copy)]
enum X {
    A(PhantomData<*const ()>),
}

impl<V> Internal<V> for X {
    type Array = [V; 1];

    fn slice(array: &[V; 1]) -> &[V] {
        array
    }

    fn slice_mut(array: &mut [V; 1]) -> &mut [V] {
        array
    }

    fn from_usize(arg: usize) -> X {
        assert_eq!(arg, 0);
        X::A(PhantomData)
    }

    fn to_usize(self) -> usize {
        0
    }

    fn from_function<F: FnMut(Self) -> V>(mut f: F) -> [V; 1] {
        [f(X::A(PhantomData))]
    }
}

fn assert_sync_send<T: Sync + Send>(_: T) {}

#[test]
fn assert_enum_map_does_not_copy_sync_send_dependency_of_keys() {
    let mut map = enum_map! { X::A(PhantomData) => true };
    assert_sync_send(map);
    assert_sync_send(&map);
    assert_sync_send(&mut map);
    assert_sync_send(map.iter());
    assert_sync_send(map.iter_mut());
    assert_sync_send(map.into_iter());
    assert_eq!(map[X::A(PhantomData)], true);
}
