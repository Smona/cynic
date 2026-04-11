use async_graphql::{ServerError, dynamic::ResolverContext};
use futures_lite::stream;

pub trait Resolver: Send + Sync {
    fn resolve(&mut self, context: ResolverContext<'_>) -> Option<serde_json::Value>;
}

pub trait SubscriptionResolver: Send + Sync {
    fn resolve(&mut self, context: ResolverContext<'_>) -> stream::Boxed<serde_json::Value>;
}

impl<F> Resolver for F
where
    for<'a> F: FnMut(ResolverContext<'a>) -> Option<serde_json::Value> + Send + Sync,
{
    fn resolve(&mut self, context: ResolverContext<'_>) -> Option<serde_json::Value> {
        self(context)
    }
}

impl Resolver for serde_json::Value {
    fn resolve(&mut self, _context: ResolverContext<'_>) -> Option<serde_json::Value> {
        Some(self.clone())
    }
}

impl Resolver for ServerError {
    fn resolve(&mut self, context: ResolverContext<'_>) -> Option<serde_json::Value> {
        context.add_error(self.clone());
        None
    }
}

impl Resolver for Option<serde_json::Value> {
    fn resolve(&mut self, _context: ResolverContext<'_>) -> Option<serde_json::Value> {
        self.clone()
    }
}

impl<F> SubscriptionResolver for F
where
    for<'a> F: FnMut(ResolverContext<'a>) -> stream::Boxed<serde_json::Value> + Send + Sync,
{
    fn resolve(&mut self, context: ResolverContext<'_>) -> stream::Boxed<serde_json::Value> {
        self(context)
    }
}

impl SubscriptionResolver for Vec<serde_json::Value> {
    fn resolve(&mut self, _context: ResolverContext<'_>) -> stream::Boxed<serde_json::Value> {
        Box::pin(stream::iter(self.clone()))
    }
}
