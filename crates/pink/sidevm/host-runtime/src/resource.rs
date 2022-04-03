use std::pin::Pin;
use tokio::time::Sleep;

pub enum Resource {
    Sleep(Pin<Box<Sleep>>),
}

impl Resource {
    pub(crate) fn poll(&mut self, task_id: i32) -> bool {
        use crate::async_context::poll_in_task_cx;
        use std::task::Poll;
        use Resource::*;

        match self {
            Sleep(handle) => match poll_in_task_cx(handle.as_mut(), task_id) {
                Poll::Ready(_) => true,
                Poll::Pending => false,
            },
        }
    }
}

#[derive(Default)]
pub struct ResourceKeeper {
    resources: Vec<Option<Resource>>,
}

const RESOURCE_ID_MAX: usize = 8192;

impl ResourceKeeper {
    pub fn get_mut(&mut self, id: i32) -> Option<&mut Resource> {
        self.resources.get_mut(id as usize)?.as_mut()
    }

    pub fn push(&mut self, resource: Resource) -> Option<i32> {
        for (i, res) in self.resources.iter_mut().enumerate() {
            if res.is_none() {
                let id = i.try_into().ok()?;
                *res = Some(resource);
                return Some(id);
            }
        }
        if self.resources.len() >= RESOURCE_ID_MAX.min(i32::MAX as _) {
            return None;
        }
        let id = self.resources.len().try_into().ok()?;
        self.resources.push(Some(resource));
        Some(id)
    }

    pub fn take(&mut self, resource_id: i32) -> Option<Resource> {
        let resource_id = resource_id as u32 as usize;
        if resource_id >= self.resources.len() {
            return None;
        }
        self.resources[resource_id].take()
    }
}