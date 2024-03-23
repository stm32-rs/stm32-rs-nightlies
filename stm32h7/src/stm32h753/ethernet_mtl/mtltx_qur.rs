#[doc = "Register `MTLTxQUR` reader"]
pub type R = crate::R<MTLTX_QURrs>;
#[doc = "Field `UFFRMCNT` reader - Underflow Packet Counter"]
pub type UFFRMCNT_R = crate::FieldReader<u16>;
#[doc = "Field `UFCNTOVF` reader - Overflow Bit for Underflow Packet Counter"]
pub type UFCNTOVF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - Underflow Packet Counter"]
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UFFRMCNT_R {
        UFFRMCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Overflow Bit for Underflow Packet Counter"]
    #[inline(always)]
    pub fn ufcntovf(&self) -> UFCNTOVF_R {
        UFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Tx queue underflow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtltx_qur::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLTX_QURrs;
impl crate::RegisterSpec for MTLTX_QURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtltx_qur::R`](R) reader structure"]
impl crate::Readable for MTLTX_QURrs {}
#[doc = "`reset()` method sets MTLTxQUR to value 0"]
impl crate::Resettable for MTLTX_QURrs {
    const RESET_VALUE: u32 = 0;
}
