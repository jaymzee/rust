use std::cell::RefCell;

pub enum Poll<T> {
    Ready(T),
    Pending
}

pub trait Future {
    type Output;

    fn poll(&mut self, ctx: &Context) -> Poll<Self::Output>;
}

thread_local!(static NOTIFY: RefCell<bool> = RefCell::new(false));

pub fn run<F: Future>(mut f: F) -> F::Output {
    NOTIFY.with(|n| {
        *n.borrow_mut() = true;
        loop {
            if *n.borrow() {
                *n.borrow_mut() = false;
                let ctx = Context::from_waker(&Waker);
                if let Poll::Ready(val) = f.poll(&ctx) {
                    return val;
                }
            }
        }
    })
}

struct Waker;

impl Waker {
    fn wake(&self) {
        NOTIFY.with(|f| *f.borrow_mut() = true)
    }
}

pub struct Context<'a> {
    waker: &'a Waker
}

impl<'a> Context<'a> {
    fn from_waker(waker: &'a Waker) -> Self {
        Context { waker }
    }

    fn waker(&self) -> &'a Waker {
        self.waker
    }
}

#[derive(Default)]
pub struct MyFuture {
    count: u32
}

impl Future for MyFuture {
    type Output = i32;

    fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
        match self.count {
            3 => Poll::Ready(3),
            _ => {
                self.count += 1;
                ctx.waker().wake();
                Poll::Pending
            }
        }
    }
}

pub struct AddOneFuture<T>(pub T);

impl<T> Future for AddOneFuture<T>
where
    T: Future,
    T::Output: std::ops::Add<i32, Output=i32>
{
    type Output = i32;

    fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
        match self.0.poll(ctx) {
            Poll::Ready(count) => Poll::Ready(count + 1),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct Foo(pub i32);
