#[doc = "Register `RCC_IDR` reader"]
pub type R = crate::R<RCC_IDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "This register gives the unique identifier of the RCC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_idr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_IDRrs;
impl crate::RegisterSpec for RCC_IDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_idr::R`](R) reader structure"]
impl crate::Readable for RCC_IDRrs {}
#[doc = "`reset()` method sets RCC_IDR to value 0x01"]
impl crate::Resettable for RCC_IDRrs {
    const RESET_VALUE: u32 = 0x01;
}
