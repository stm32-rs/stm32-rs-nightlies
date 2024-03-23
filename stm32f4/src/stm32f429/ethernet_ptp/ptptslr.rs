#[doc = "Register `PTPTSLR` reader"]
pub type R = crate::R<PTPTSLRrs>;
#[doc = "Field `STSS` reader - STSS"]
pub type STSS_R = crate::FieldReader<u32>;
#[doc = "Field `STPNS` reader - STPNS"]
pub type STPNS_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - STSS"]
    #[inline(always)]
    pub fn stss(&self) -> STSS_R {
        STSS_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - STPNS"]
    #[inline(always)]
    pub fn stpns(&self) -> STPNS_R {
        STPNS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Ethernet PTP time stamp low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptslr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSLRrs;
impl crate::RegisterSpec for PTPTSLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptslr::R`](R) reader structure"]
impl crate::Readable for PTPTSLRrs {}
#[doc = "`reset()` method sets PTPTSLR to value 0"]
impl crate::Resettable for PTPTSLRrs {
    const RESET_VALUE: u32 = 0;
}
