#[doc = "Register `ETH_MTLTxQ0ESR` reader"]
pub type R = crate::R<ETH_MTLTX_Q0ESRrs>;
#[doc = "Field `ABS` reader - ABS"]
pub type ABS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - ABS"]
    #[inline(always)]
    pub fn abs(&self) -> ABS_R {
        ABS_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Tx queue x ETS status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q0esr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MTLTX_Q0ESRrs;
impl crate::RegisterSpec for ETH_MTLTX_Q0ESRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mtltx_q0esr::R`](R) reader structure"]
impl crate::Readable for ETH_MTLTX_Q0ESRrs {}
#[doc = "`reset()` method sets ETH_MTLTxQ0ESR to value 0"]
impl crate::Resettable for ETH_MTLTX_Q0ESRrs {
    const RESET_VALUE: u32 = 0;
}
