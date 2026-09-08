
use crate::prelude::*;
use crate::mapping::TypeKey;

struct A(i32); impl TypeKey for A { type Type = i32; }
struct B(&'static str); impl TypeKey for B { type Type = i32; }
struct C; impl TypeKey for C { type Type = &'static str; }

#[test]
fn test_type_map() {
    let mut map = TypeMap::new();
    
    assert!(!map.has::<A>());
    assert!(!map.has::<B>());
    assert!(!map.has::<C>());
    
    assert!(map.get::<A>().is_none());
    assert!(map.add::<A>(0));
    assert!(map.has::<A>());
    assert_eq!(map.get::<A>(), Some(&0));
    
    assert!(!map.add::<A>(1));
    assert!(map.has::<A>());
    assert_eq!(map.get::<A>(), Some(&0));
    
    assert!(map.add::<B>(2));
    assert_eq!(map.get::<A>(), Some(&0));
    assert_eq!(map.get::<B>(), Some(&2));
    
    assert!(map.add::<C>(2));
    assert_eq!(map.get::<A>(), Some(&0));
    assert_eq!(map.get::<B>(), Some(&2));
    assert_eq!(map.get::<C>(), Some(&2));
}

#[test]
fn test_type_map_iter() {
    let mut map = TypeMap::new();
    
    map.add::<A>(0);
    map.add::<B>(1);
    map.add::<C>(2);
    
    let [mut a, mut b, mut c] = [false; 3];
    
    for entry in &map {
        if let Some(&0) = entry.match_key::<A>() { a = true; }
        if let Some(&1) = entry.match_key::<B>() { b = true; }
        if let Some(&2) = entry.match_key::<C>() { c = true; }
    }
    
    assert!(a && b && c);
    
    [a, b, c] = [false; 3];
    
    for mut entry in &mut map {
        if let Some(t) = entry.match_key::<A>() {
            a = true;
            assert_eq!(*t, 0);
            *t += 1;
        }
        if let Some(t) = entry.match_key::<B>() {
            b = true;
            assert_eq!(*t, 1);
            *t += 1;
        }
        if let Some(t) = entry.match_key::<C>() {
            c = true;
            assert_eq!(*t, 2);
            *t += 1;
        }
    }
    
    assert!(a && b && c);
    assert_eq!(map.get::<A>(), Some(&1));
    assert_eq!(map.get::<B>(), Some(&2));
    assert_eq!(map.get::<C>(), Some(&3));
    
    [a, b, c] = [false; 3];
    
    for entry in map {
        if entry.is_key::<A>() {
            a = true;
            assert_eq!(entry.match_key::<A>(), Some(1));
        }
        else if entry.is_key::<B>() {
            b = true;
            assert_eq!(entry.match_key::<B>(), Some(2));
        }
        else if entry.is_key::<C>() {
            c = true;
            assert_eq!(entry.match_key::<C>(), Some(3));
        }
    }
    
    assert!(a && b && c);
}

#[test]
fn test_type_set() {
    let mut set = TypeSet::new();
    
    assert!(!set.has::<A>());
    assert!(!set.has::<B>());
    assert!(!set.has::<C>());
    
    assert!(set.add::<A>());
    assert!(set.has::<A>());
    
    assert!(!set.add::<A>());
    assert!(set.has::<A>());
    
    assert!(set.add::<B>());
    assert!(set.has::<A>());
    assert!(set.has::<B>());
    
    assert!(set.add::<C>());
    assert!(set.has::<A>());
    assert!(set.has::<B>());
    assert!(set.has::<C>());
}

#[test]
fn test_type_set_iter() {
    let mut set = TypeSet::new();
    
    set.add::<A>();
    set.add::<B>();
    set.add::<C>();
    
    let [mut a, mut b, mut c] = [false; 3];
    
    for entry in &set {
        if entry.is_key::<A>() { a = true; }
        if entry.is_key::<B>() { b = true; }
        if entry.is_key::<C>() { c = true; }
    }
    
    assert!(a && b && c);
    
    [a, b, c] = [false; 3];
    
    for entry in &mut set {
        if entry.is_key::<A>() { a = true; }
        if entry.is_key::<B>() { b = true; }
        if entry.is_key::<C>() { c = true; }
    }
    
    assert!(a && b && c);
    assert!(set.has::<A>());
    assert!(set.has::<B>());
    assert!(set.has::<C>());
    
    [a, b, c] = [false; 3];
    
    for entry in set {
        if entry.is_key::<A>() { a = true; }
        if entry.is_key::<B>() { b = true; }
        if entry.is_key::<C>() { c = true; }
    }
    
    assert!(a && b && c);
}

#[test]
fn test_type_dictionary() {
    let mut dict: TypeDictionary = TypeDictionary::new();
    
    assert!(!dict.has::<A>());
    assert!(!dict.has::<B>());
    assert!(!dict.has::<C>());
    
    assert!(dict.get::<A>().is_none());
    assert!(dict.add::<A>(0));
    assert!(dict.has::<A>());
    assert_eq!(dict.get::<A>(), Some(&0));
    
    assert!(!dict.add::<A>(1));
    assert!(dict.has::<A>());
    assert_eq!(dict.get::<A>(), Some(&0));
    
    assert!(dict.add::<B>(2));
    assert_eq!(dict.get::<A>(), Some(&0));
    assert_eq!(dict.get::<B>(), Some(&2));
    
    assert!(dict.add::<C>("test"));
    assert_eq!(dict.get::<A>(), Some(&0));
    assert_eq!(dict.get::<B>(), Some(&2));
    assert_eq!(dict.get::<C>(), Some(&"test"));
}

#[test]
fn test_type_dictionary_iter() {
    let mut dict: TypeDictionary = TypeDictionary::new();
    
    dict.add::<A>(0);
    dict.add::<B>(1);
    dict.add::<C>("test");
    
    let [mut a, mut b, mut c] = [false; 3];
    
    for entry in &dict {
        if let Some(&0) = entry.match_key::<A>() { a = true; }
        if let Some(&1) = entry.match_key::<B>() { b = true; }
        if let Some(&"test") = entry.match_key::<C>() { c = true; }
    }
    
    assert!(a && b && c);
    
    [a, b, c] = [false; 3];
    
    for mut entry in &mut dict {
        if let Some(t) = entry.match_key::<A>() {
            a = true;
            assert_eq!(*t, 0);
            *t += 1;
        }
        if let Some(t) = entry.match_key::<B>() {
            b = true;
            assert_eq!(*t, 1);
            *t += 1;
        }
        if let Some(t) = entry.match_key::<C>() {
            c = true;
            assert_eq!(*t, "test");
            *t = "test2";
        }
    }
    
    assert!(a && b && c);
    assert_eq!(dict.get::<A>(), Some(&1));
    assert_eq!(dict.get::<B>(), Some(&2));
    assert_eq!(dict.get::<C>(), Some(&"test2"));
    
    [a, b, c] = [false; 3];
    
    for entry in dict {
        if entry.is_key::<A>() {
            a = true;
            assert_eq!(entry.match_key::<A>(), Some(1));
        }
        else if entry.is_key::<B>() {
            b = true;
            assert_eq!(entry.match_key::<B>(), Some(2));
        }
        else if entry.is_key::<C>() {
            c = true;
            assert_eq!(entry.match_key::<C>(), Some("test2"));
        }
    }
    
    assert!(a && b && c);
}

#[test]
fn test_any_set() {
    let mut set = AnySet::new();
    
    assert!(!set.has::<A>());
    assert!(!set.has::<B>());
    assert!(!set.has::<C>());
    
    assert!(set.get::<A>().is_none());
    assert!(set.add(A(0)));
    assert!(set.has::<A>());
    assert!(matches!(set.get(), Some(&A(0))));
    
    assert!(!set.add(A(1)));
    assert!(set.has::<A>());
    assert!(matches!(set.get(), Some(&A(0))));
    
    assert!(set.add(B("test")));
    assert!(matches!(set.get(), Some(&A(0))));
    assert!(matches!(set.get(), Some(&B("test"))));
    
    assert!(set.add(C));
    assert!(matches!(set.get(), Some(&A(0))));
    assert!(matches!(set.get(), Some(&B("test"))));
    assert!(matches!(set.get(), Some(&C)));
}

#[test]
fn test_any_set_iter() {
    let mut set = AnySet::new();
    
    set.add(A(0));
    set.add(B("test"));
    set.add(C);
    
    let [mut a, mut b, mut c] = [false; 3];
    
    for entry in &set {
        if let Some(&A(0)) = entry.match_key() { a = true; }
        if let Some(&B("test")) = entry.match_key() { b = true; }
        if let Some(&C) = entry.match_key() { c = true; }
    }
    
    assert!(a && b && c);
    
    [a, b, c] = [false; 3];
    
    for mut entry in &mut set {
        if let Some(t) = entry.match_key() {
            a = true;
            assert!(matches!(*t, A(0)));
            *t = A(1);
        }
        if let Some(t) = entry.match_key() {
            b = true;
            assert!(matches!(*t, B("test")));
            *t = B("test2");
        }
        if let Some(t) = entry.match_key() {
            c = true;
            assert!(matches!(*t, C));
        }
    }
    
    assert!(a && b && c);
    assert!(matches!(set.get(), Some(&A(1))));
    assert!(matches!(set.get(), Some(&B("test2"))));
    assert!(matches!(set.get(), Some(&C)));
    
    [a, b, c] = [false; 3];
    
    for entry in set {
        if entry.is_key::<A>() {
            a = true;
            assert!(matches!(entry.match_key(), Some(A(1))));
        }
        else if entry.is_key::<B>() {
            b = true;
            assert!(matches!(entry.match_key(), Some(B("test2"))));
        }
        else if entry.is_key::<C>() {
            c = true;
            assert!(matches!(entry.match_key(), Some(C)));
        }
    }
    
    assert!(a && b && c);
}
