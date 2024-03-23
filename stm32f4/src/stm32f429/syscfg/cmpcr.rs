#[doc = "Register `CMPCR` reader"]
pub type R = crate::R<CMPCRrs>;
#[doc = "Field `CMP_PD` reader - Compensation cell power-down"]
pub type CMP_PD_R = crate::BitReader;
#[doc = "Field `READY` reader - READY"]
pub type READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Compensation cell power-down"]
    #[inline(always)]
    pub fn cmp_pd(&self) -> CMP_PD_R {
        CMP_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - READY"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Compensation cell control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPCRrs;
impl crate::RegisterSpec for CMPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpcr::R`](R) reader structure"]
impl crate::Readable for CMPCRrs {}
#[doc = "`reset()` method sets CMPCR to value 0"]
impl crate::Resettable for CMPCRrs {
    const RESET_VALUE: u32 = 0;
}
