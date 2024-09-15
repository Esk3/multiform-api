use std::sync::{Arc, Mutex};

use crate::{
    into_response::IntoResponse,
    response::new_header,
    service::{self, Fut, Service},
};

use super::router_args::{Args, RouterArgs};

#[derive(Debug, Clone)]
pub struct BestillingsId<T: Clone> {
    pub inner: T,
    pub ids: Arc<Mutex<i32>>,
}

impl<T: Clone> BestillingsId<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            ids: Arc::new(Mutex::new(0)),
        }
    }
    fn next_id(&self) -> i32 {
        let mut lock = self.ids.lock().unwrap();
        *lock += 1;
        *lock
    }
}

impl<T> service::Service<RouterArgs> for BestillingsId<T>
where
    T: Service<Args, Response = Box<dyn IntoResponse>, Error = ()> + Clone + 'static,
{
    type Response = Box<dyn IntoResponse>;

    type Error = ();

    type Future = Fut<Self::Response, Self::Error>;

    fn call(
        &mut self,
        RouterArgs {
            url,
            method,
            bestillings_id,
            body,
            pool,
        }: RouterArgs,
    ) -> Self::Future {
        let id = if let Some(id) = bestillings_id {
            id
        } else {
            self.next_id()
        };
        let args = Args {
            url,
            method,
            bestillings_id: id,
            body,
            pool,
        };

        let mut this = self.clone();
        Box::pin(async move {
            this.inner
                .call(args)
                .await
                .map(|res| {
                    res.into_response()
                        .with_header(new_header("Set-Cookie", &format!("bestillings_id={id}")))
                })
                .map::<Box<dyn IntoResponse>, _>(|res| Box::new(res))
        })
    }
}
