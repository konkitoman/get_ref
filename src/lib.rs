use std::alloc::System;
use std::any::Any;

pub trait TGetRef {
    const LEN: usize;

    fn get_ref<'a>(&'a self, index: usize) -> Option<&'a dyn Any>;
    fn get_ref_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut dyn Any>;
    fn get_ref_len(&self) -> usize {
        Self::LEN
    }
}

macro_rules! trupl_intrp {
    ($def:ident) => {
        $def!(
            {T0.0, T1.1}
            {T0.0, T1.1, T2.2}
            {T0.0, T1.1, T2.2, T3.3}
            {T0.0, T1.1, T2.2, T3.3, T4.4}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6, T7.7}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6, T7.7, T8.8}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6, T7.7, T8.8, T9.9}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6, T7.7, T8.8, T9.9, T10.10}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6, T7.7, T8.8, T9.9, T10.10, T11.11}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6, T7.7, T8.8, T9.9, T10.10, T11.11, T12.12}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6, T7.7, T8.8, T9.9, T10.10, T11.11, T12.12, T13.13}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6, T7.7, T8.8, T9.9, T10.10, T11.11, T12.12, T13.13, T14.14}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6, T7.7, T8.8, T9.9, T10.10, T11.11, T12.12, T13.13, T14.14, T15.15}
        );
    };
}

macro_rules! impl_truple {
    ($( { $($T:ident.$idx:tt),* } )*) => ($(
        impl<$($T: 'static,)*> TGetRef for ($($T),*){
            const LEN: usize = $($idx+)*0;

            fn get_ref<'a>(&'a self, index: usize) -> Option<&'a dyn Any> {
                match index{
                    $($idx => Some(&self.$idx as &dyn Any),)*
                    _ => None
                }
            }

            fn get_ref_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut dyn Any> {
                match index{
                    $($idx => Some(&mut self.$idx as &mut dyn Any),)*
                    _ => None
                }
            }
        }
    )*)
}

#[macro_export]
macro_rules! impl_get_ref {
    (<$($TT:ident: $TTP:ident),*> $T:ty) => {
        impl<$($TT:$TTP),*> TGetRef for $T {
            const LEN: usize = 1;

            fn get_ref<'a>(&'a self, index: usize) -> Option<&'a dyn Any> {
                match index{
                    0 => Some(self as &dyn Any),
                    _ => None
                }
            }

            fn get_ref_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut dyn Any> {
                match index{
                    0 => Some(self as &mut dyn Any),
                    _ => None
                }
            }
        }
    };

    (<$($TT:ident),*> $T:ty) => {
        impl<$($TT),*> TGetRef for $T where $T: 'static {
            const LEN: usize = 1;

            fn get_ref<'a>(&'a self, index: usize) -> Option<&'a dyn Any> {
                match index{
                    0 => Some(self as &dyn Any),
                    _ => None
                }
            }

            fn get_ref_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut dyn Any> {
                match index{
                    0 => Some(self as &mut dyn Any),
                    _ => None
                }
            }
        }
    };

    ($T:ty) => {
        impl TGetRef for $T where $T: 'static {
            const LEN: usize = 1;

            fn get_ref<'a>(&'a self, index: usize) -> Option<&'a dyn Any> {
                match index{
                    0 => Some(self as &dyn Any),
                    _ => None
                }
            }

            fn get_ref_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut dyn Any> {
                match index{
                    0 => Some(self as &mut dyn Any),
                    _ => None
                }
            }
        }
    };

    ($($T:ty),*) => ($(impl_get_ref!($T);)*);
}

use std::cell::{Cell, RefCell, UnsafeCell};
use std::io::{BufReader, Bytes};
use std::rc::{Rc, Weak};
use std::sync::atomic::{
    AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16, AtomicU32,
    AtomicU64, AtomicU8, AtomicUsize,
};
use std::sync::mpsc::{Sender, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread::{JoinHandle, Thread, ThreadId};
use std::time::{Duration, Instant, SystemTime};

trupl_intrp!(impl_truple);

impl_get_ref!(i8, i32, i64, i128, isize);
impl_get_ref!(u8, u32, u64, u128, usize);
impl_get_ref!(f32, f64);

impl_get_ref!(bool, AtomicBool);

impl_get_ref!(AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize);
impl_get_ref!(AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize);

impl_get_ref!(Instant, Duration, System, SystemTime, String);
impl_get_ref!(Thread, ThreadId);
// impl_get_ref!(Scope<'_, '_>);
impl_get_ref!(<T> JoinHandle<T>);
// impl_get_ref!(<T> ScopedJoinHandle<'_, T>);

impl_get_ref!(<T> Vec<T>);
// impl_get_ref!(<T> std::vec::Drain<'_, T>);

impl_get_ref!(<T> Option<T>);
impl_get_ref!(<T> Mutex<T>);
impl_get_ref!(<T> Arc<T>);
impl_get_ref!(<T> Rc<T>);
impl_get_ref!(<T> Sender<T>);
impl_get_ref!(<T> SyncSender<T>);
impl_get_ref!(<T> Weak<T>);
impl_get_ref!(<T> Cell<T>);
impl_get_ref!(<T> RefCell<T>);
impl_get_ref!(<T> UnsafeCell<T>);
impl_get_ref!(<T> Bytes<T>);
impl_get_ref!(<T> BufReader<T>);
