#[doc = "Register `DMACCARXBR` reader"]
pub type R = crate::R<DMACCARXBRrs>;
#[doc = "Field `CURRBUFAPTR` reader - Application Receive Buffer Address Pointer The DMA updates this pointer during Rx operation. This pointer is cleared on reset."]
pub type CURRBUFAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Application Receive Buffer Address Pointer The DMA updates this pointer during Rx operation. This pointer is cleared on reset."]
    #[inline(always)]
    pub fn currbufaptr(&self) -> CURRBUFAPTR_R {
        CURRBUFAPTR_R::new(self.bits)
    }
}
#[doc = "Channel current application receive buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccarxbr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACCARXBRrs;
impl crate::RegisterSpec for DMACCARXBRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccarxbr::R`](R) reader structure"]
impl crate::Readable for DMACCARXBRrs {}
#[doc = "`reset()` method sets DMACCARXBR to value 0"]
impl crate::Resettable for DMACCARXBRrs {
    const RESET_VALUE: u32 = 0;
}
