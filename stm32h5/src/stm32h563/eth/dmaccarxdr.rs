#[doc = "Register `DMACCARXDR` reader"]
pub type R = crate::R<DMACCARXDRrs>;
#[doc = "Field `CURRDESAPTR` reader - Application Receive Descriptor Address Pointer The DMA updates this pointer during Rx operation. This pointer is cleared on reset."]
pub type CURRDESAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Application Receive Descriptor Address Pointer The DMA updates this pointer during Rx operation. This pointer is cleared on reset."]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new(self.bits)
    }
}
#[doc = "Channel current application receive descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccarxdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACCARXDRrs;
impl crate::RegisterSpec for DMACCARXDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccarxdr::R`](R) reader structure"]
impl crate::Readable for DMACCARXDRrs {}
#[doc = "`reset()` method sets DMACCARXDR to value 0"]
impl crate::Resettable for DMACCARXDRrs {
    const RESET_VALUE: u32 = 0;
}
