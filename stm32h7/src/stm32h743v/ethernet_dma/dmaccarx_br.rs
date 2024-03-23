#[doc = "Register `DMACCARxBR` reader"]
pub type R = crate::R<DMACCARX_BRrs>;
#[doc = "Field `CURRBUFAPTR` reader - Application Receive Buffer Address Pointer"]
pub type CURRBUFAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Application Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn currbufaptr(&self) -> CURRBUFAPTR_R {
        CURRBUFAPTR_R::new(self.bits)
    }
}
#[doc = "Channel current application receive buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccarx_br::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACCARX_BRrs;
impl crate::RegisterSpec for DMACCARX_BRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccarx_br::R`](R) reader structure"]
impl crate::Readable for DMACCARX_BRrs {}
#[doc = "`reset()` method sets DMACCARxBR to value 0"]
impl crate::Resettable for DMACCARX_BRrs {
    const RESET_VALUE: u32 = 0;
}
