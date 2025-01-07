use std::sync::mpsc::Sender;

use super::{ MEM, MemoryRegion };

pub struct MemoryHook {
    operation: MemoryOperation,
    range: MemoryRegion,
    tx: Sender<MemoryEvent>,
}

impl MemoryHook {
    pub fn send(&self, address: usize, value: u8) {
        self.tx.send(
            MemoryEvent {
                operation: self.operation,
                address: address as u16,
                value,
            }
        ).unwrap()
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MemoryOperation {
    Read,
    Write,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct MemoryEvent {
    pub operation: MemoryOperation,
    pub address: u16,
    pub value: u8,
}

impl MEM {
    pub fn push_hook(&mut self, operation: MemoryOperation, range: MemoryRegion, tx: Sender<MemoryEvent>) {
        self.hooks.push(
            MemoryHook {
                operation,
                range,
                tx,
            }
        );
    }

    pub fn get_hooks(&self, operation: MemoryOperation, address: usize) -> Vec<&MemoryHook> {
        let mut valid_hooks = vec![];
        for hook in &self.hooks {
            if hook.operation == operation {
                if hook.range.inside_region(address) {
                    valid_hooks.push(hook);
                }
            }
        }
        return valid_hooks;
    }
}

#[cfg(test)]
mod memory_hook_tests {
    use std::sync::mpsc::{ channel, Receiver, TryRecvError, Sender };

    use super::*;
    use super::super::MEMORY_SIZE;

    #[test]
    fn test_get_hooks() {
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        let (tx1, _): (Sender<MemoryEvent>, Receiver<MemoryEvent>) = channel();
        let (tx2, _): (Sender<MemoryEvent>, Receiver<MemoryEvent>) = channel();

        test_memory.push_hook(
            MemoryOperation::Read,
            MemoryRegion {
                region_address: 0x0010,
                region_size: 0x0010,
            },
            tx1,
        );
        test_memory.push_hook(
            MemoryOperation::Write,
            MemoryRegion {
                region_address: 0x0010,
                region_size: 0x0010,
            },
            tx2,
        );

        assert_eq!(test_memory.get_hooks(MemoryOperation::Read,  0x000F).is_empty(), true);
        assert_eq!(test_memory.get_hooks(MemoryOperation::Write, 0x000F).is_empty(), true);
        assert_eq!(test_memory.get_hooks(MemoryOperation::Read,  0x0020).is_empty(), true);
        assert_eq!(test_memory.get_hooks(MemoryOperation::Write, 0x0020).is_empty(), true);

        assert_eq!(test_memory.get_hooks(MemoryOperation::Read,  0x0010).is_empty(), false);
        assert_eq!(test_memory.get_hooks(MemoryOperation::Write, 0x0010).is_empty(), false);
        assert_eq!(test_memory.get_hooks(MemoryOperation::Read,  0x001F).is_empty(), false);
        assert_eq!(test_memory.get_hooks(MemoryOperation::Write, 0x001F).is_empty(), false);
    }

    #[test]
    fn test_multiple_hooks() {
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        let (tx1, _): (Sender<MemoryEvent>, Receiver<MemoryEvent>) = channel();
        let (tx2, _): (Sender<MemoryEvent>, Receiver<MemoryEvent>) = channel();
        let (tx3, _): (Sender<MemoryEvent>, Receiver<MemoryEvent>) = channel();
        let (tx4, _): (Sender<MemoryEvent>, Receiver<MemoryEvent>) = channel();

        test_memory.push_hook(
            MemoryOperation::Read,
            MemoryRegion {
                region_address: 0x0010,
                region_size: 0x0020,
            },
            tx1,
        );
        test_memory.push_hook(
            MemoryOperation::Read,
            MemoryRegion {
                region_address: 0x0020,
                region_size: 0x0010,
            },
            tx2,
        );
        test_memory.push_hook(
            MemoryOperation::Write,
            MemoryRegion {
                region_address: 0x0010,
                region_size: 0x0020,
            },
            tx3,
        );
        test_memory.push_hook(
            MemoryOperation::Write,
            MemoryRegion {
                region_address: 0x0020,
                region_size: 0x0010,
            },
            tx4,
        );

        assert_eq!(test_memory.get_hooks(MemoryOperation::Read, 0x0007).len(), 0);
        assert_eq!(test_memory.get_hooks(MemoryOperation::Read, 0x0017).len(), 1);
        assert_eq!(test_memory.get_hooks(MemoryOperation::Read, 0x0027).len(), 2);
        assert_eq!(test_memory.get_hooks(MemoryOperation::Read, 0x0037).len(), 0);

        assert_eq!(test_memory.get_hooks(MemoryOperation::Write, 0x0007).len(), 0);
        assert_eq!(test_memory.get_hooks(MemoryOperation::Write, 0x0017).len(), 1);
        assert_eq!(test_memory.get_hooks(MemoryOperation::Write, 0x0027).len(), 2);
        assert_eq!(test_memory.get_hooks(MemoryOperation::Write, 0x0037).len(), 0);
    }

    #[test]
    fn test_multiple_hooks_send() {
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        let (tx1, rx1): (Sender<MemoryEvent>, Receiver<MemoryEvent>) = channel();
        let (tx2, rx2): (Sender<MemoryEvent>, Receiver<MemoryEvent>) = channel();
        let (tx3, rx3): (Sender<MemoryEvent>, Receiver<MemoryEvent>) = channel();
        let (tx4, rx4): (Sender<MemoryEvent>, Receiver<MemoryEvent>) = channel();

        test_memory.push_hook(
            MemoryOperation::Read,
            MemoryRegion {
                region_address: 0x0010,
                region_size: 0x0020,
            },
            tx1
        );
        test_memory.push_hook(
            MemoryOperation::Read,
            MemoryRegion {
                region_address: 0x0020,
                region_size: 0x0010,
            },
            tx2,
        );
        test_memory.push_hook(
            MemoryOperation::Write,
            MemoryRegion {
                region_address: 0x0010,
                region_size: 0x0020,
            },
            tx3
        );
        test_memory.push_hook(
            MemoryOperation::Write,
            MemoryRegion {
                region_address: 0x0020,
                region_size: 0x0010,
            },
            tx4,
        );

        for hook in test_memory.get_hooks(MemoryOperation::Read, 0x0007) {
            hook.send(0x0007, 0xDE);
        }
        for hook in test_memory.get_hooks(MemoryOperation::Read, 0x0017) {
            hook.send(0x0017, 0xAD);
        }
        for hook in test_memory.get_hooks(MemoryOperation::Read, 0x0027) {
            hook.send(0x0027, 0xBE);
        }
        for hook in test_memory.get_hooks(MemoryOperation::Read, 0x0037) {
            hook.send(0x0037, 0xEF);
        }

        for hook in test_memory.get_hooks(MemoryOperation::Write, 0x0007) {
            hook.send(0x0007, 0xDE);
        }
        for hook in test_memory.get_hooks(MemoryOperation::Write, 0x0017) {
            hook.send(0x0017, 0xAD);
        }
        for hook in test_memory.get_hooks(MemoryOperation::Write, 0x0027) {
            hook.send(0x0027, 0xBE);
        }
        for hook in test_memory.get_hooks(MemoryOperation::Write, 0x0037) {
            hook.send(0x0037, 0xEF);
        }

        assert_eq!(rx1.try_recv(), Ok(MemoryEvent{
            operation: MemoryOperation::Read,
            address: 0x0017,
            value: 0xAD,
        }));
        assert_eq!(rx1.try_recv(), Ok(MemoryEvent{
            operation: MemoryOperation::Read,
            address: 0x0027,
            value: 0xBE,
        }));
        assert_eq!(rx1.try_recv(), Err(TryRecvError::Empty));

        assert_eq!(rx2.try_recv(), Ok(MemoryEvent{
            operation: MemoryOperation::Read,
            address: 0x0027,
            value: 0xBE,
        }));
        assert_eq!(rx2.try_recv(), Err(TryRecvError::Empty));

        assert_eq!(rx3.try_recv(), Ok(MemoryEvent{
            operation: MemoryOperation::Write,
            address: 0x0017,
            value: 0xAD,
        }));
        assert_eq!(rx3.try_recv(), Ok(MemoryEvent{
            operation: MemoryOperation::Write,
            address: 0x0027,
            value: 0xBE,
        }));
        assert_eq!(rx3.try_recv(), Err(TryRecvError::Empty));

        assert_eq!(rx4.try_recv(), Ok(MemoryEvent{
            operation: MemoryOperation::Write,
            address: 0x0027,
            value: 0xBE,
        }));
        assert_eq!(rx4.try_recv(), Err(TryRecvError::Empty));
    }
}
