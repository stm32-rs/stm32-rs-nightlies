#[doc = "Register `RX_UNICAST_PACKETS_GOOD` reader"]
pub type R = crate::R<RX_UNICAST_PACKETS_GOODrs>;
#[doc = "Field `RXUCASTG` reader - RXUCASTG"]
pub type RXUCASTG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RXUCASTG"]
    #[inline(always)]
    pub fn rxucastg(&self) -> RXUCASTG_R {
        RXUCASTG_R::new(self.bits)
    }
}
#[doc = "This register provides the number of good unicast packets received by Ethernet peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_unicast_packets_good::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_UNICAST_PACKETS_GOODrs;
impl crate::RegisterSpec for RX_UNICAST_PACKETS_GOODrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_unicast_packets_good::R`](R) reader structure"]
impl crate::Readable for RX_UNICAST_PACKETS_GOODrs {}
#[doc = "`reset()` method sets RX_UNICAST_PACKETS_GOOD to value 0"]
impl crate::Resettable for RX_UNICAST_PACKETS_GOODrs {
    const RESET_VALUE: u32 = 0;
}
