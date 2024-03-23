#[doc = "Register `DMACCATXDR` reader"]
pub type R = crate::R<DMACCATXDRrs>;
#[doc = "Field `CURTDESAPTR` reader - Application Transmit Descriptor Address Pointer The DMA updates this pointer during Tx operation. This pointer is cleared on reset."]
pub type CURTDESAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Descriptor Address Pointer The DMA updates this pointer during Tx operation. This pointer is cleared on reset."]
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CURTDESAPTR_R {
        CURTDESAPTR_R::new(self.bits)
    }
}
#[doc = "Channel current application transmit descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccatxdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACCATXDRrs;
impl crate::RegisterSpec for DMACCATXDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccatxdr::R`](R) reader structure"]
impl crate::Readable for DMACCATXDRrs {}
#[doc = "`reset()` method sets DMACCATXDR to value 0"]
impl crate::Resettable for DMACCATXDRrs {
    const RESET_VALUE: u32 = 0;
}
