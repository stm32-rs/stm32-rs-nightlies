#[doc = "Register `DMACCATXBR` reader"]
pub type R = crate::R<DMACCATXBRrs>;
#[doc = "Field `CURTBUFAPTR` reader - Application Transmit Buffer Address Pointer The DMA updates this pointer during Tx operation. This pointer is cleared on reset."]
pub type CURTBUFAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Buffer Address Pointer The DMA updates this pointer during Tx operation. This pointer is cleared on reset."]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new(self.bits)
    }
}
#[doc = "Channel current application transmit buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccatxbr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACCATXBRrs;
impl crate::RegisterSpec for DMACCATXBRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccatxbr::R`](R) reader structure"]
impl crate::Readable for DMACCATXBRrs {}
#[doc = "`reset()` method sets DMACCATXBR to value 0"]
impl crate::Resettable for DMACCATXBRrs {
    const RESET_VALUE: u32 = 0;
}
