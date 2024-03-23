#[doc = "Register `ETH_MTLTxQ0UR` reader"]
pub type R = crate::R<ETH_MTLTX_Q0URrs>;
#[doc = "Field `UFFRMCNT` reader - UFFRMCNT"]
pub type UFFRMCNT_R = crate::FieldReader<u16>;
#[doc = "Field `UFCNTOVF` reader - UFCNTOVF"]
pub type UFCNTOVF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - UFFRMCNT"]
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UFFRMCNT_R {
        UFFRMCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - UFCNTOVF"]
    #[inline(always)]
    pub fn ufcntovf(&self) -> UFCNTOVF_R {
        UFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Tx queue 0 underflow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q0ur::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MTLTX_Q0URrs;
impl crate::RegisterSpec for ETH_MTLTX_Q0URrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mtltx_q0ur::R`](R) reader structure"]
impl crate::Readable for ETH_MTLTX_Q0URrs {}
#[doc = "`reset()` method sets ETH_MTLTxQ0UR to value 0"]
impl crate::Resettable for ETH_MTLTX_Q0URrs {
    const RESET_VALUE: u32 = 0;
}
