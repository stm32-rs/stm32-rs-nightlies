#[doc = "Register `DMACCARxDR` reader"]
pub type R = crate::R<DMACCARX_DRrs>;
#[doc = "Field `CURRDESAPTR` reader - Application Receive Descriptor Address Pointer"]
pub type CURRDESAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Application Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new(self.bits)
    }
}
#[doc = "Channel current application receive descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccarx_dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACCARX_DRrs;
impl crate::RegisterSpec for DMACCARX_DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccarx_dr::R`](R) reader structure"]
impl crate::Readable for DMACCARX_DRrs {}
#[doc = "`reset()` method sets DMACCARxDR to value 0"]
impl crate::Resettable for DMACCARX_DRrs {
    const RESET_VALUE: u32 = 0;
}
