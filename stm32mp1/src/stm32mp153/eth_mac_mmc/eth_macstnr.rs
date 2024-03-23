#[doc = "Register `ETH_MACSTNR` reader"]
pub type R = crate::R<ETH_MACSTNRrs>;
#[doc = "Field `TSSS` reader - TSSS"]
pub type TSSS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - TSSS"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
}
#[doc = "The System Time Nanoseconds register, along with System Time Seconds register, indicates the current value of the system time maintained by the MAC. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macstnr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACSTNRrs;
impl crate::RegisterSpec for ETH_MACSTNRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macstnr::R`](R) reader structure"]
impl crate::Readable for ETH_MACSTNRrs {}
#[doc = "`reset()` method sets ETH_MACSTNR to value 0"]
impl crate::Resettable for ETH_MACSTNRrs {
    const RESET_VALUE: u32 = 0;
}
