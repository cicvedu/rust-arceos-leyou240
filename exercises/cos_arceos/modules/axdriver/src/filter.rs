use driver_common::{BaseDriverOps, DeviceType};
#[cfg(feature = "net")]
use driver_net::NetDriverOps;
// use driver_virtio::{Transport, VirtIoNetDev};
// use crate::virtio::VirtIoHalImpl;

pub struct NetFilter<T: BaseDriverOps> {
    pub inner: T,
}
#[cfg(feature = "net")]
impl<T: BaseDriverOps + NetDriverOps> BaseDriverOps for NetFilter<T> {
    fn device_name(&self) -> &str {
        self.inner.device_name()
    }

    fn device_type(&self) -> DeviceType {
        self.inner.device_type()
    }
}

#[cfg(feature = "net")]
impl<T: BaseDriverOps + NetDriverOps> NetDriverOps for NetFilter<T> {
    fn mac_address(&self) -> driver_net::EthernetAddress {
        self.inner.mac_address()
    }

    fn can_transmit(&self) -> bool {
        self.inner.can_transmit()
    }

    fn can_receive(&self) -> bool {
        self.inner.can_receive()
    }

    fn rx_queue_size(&self) -> usize {
        self.inner.rx_queue_size()
    }

    fn tx_queue_size(&self) -> usize {
        self.inner.tx_queue_size()
    }

    fn recycle_rx_buffer(&mut self, rx_buf: driver_net::NetBufPtr) -> driver_common::DevResult {
        self.inner.recycle_rx_buffer(rx_buf)
    }

    fn recycle_tx_buffers(&mut self) -> driver_common::DevResult {
        self.inner.recycle_tx_buffers()
    }

    fn transmit(&mut self, tx_buf: driver_net::NetBufPtr) -> driver_common::DevResult {
        self.inner.transmit(tx_buf)
    }

    fn receive(&mut self) -> driver_common::DevResult<driver_net::NetBufPtr> {
        let ret = self.inner.receive()?;
        Ok(ret)
    }

    fn alloc_tx_buffer(&mut self, size: usize) -> driver_common::DevResult<driver_net::NetBufPtr> {
        self.inner.alloc_tx_buffer(size)
    }
}