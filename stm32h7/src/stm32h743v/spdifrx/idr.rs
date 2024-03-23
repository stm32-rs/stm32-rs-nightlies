#[doc = "Register `IDR` reader"]
pub type R = crate::R<IDRrs>;
#[doc = "Field `ID` reader - SPDIFRX identifier"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SPDIFRX identifier"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "SPDIFRX identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr::R`](R) reader structure"]
impl crate::Readable for IDRrs {}
#[doc = "`reset()` method sets IDR to value 0x0013_0041"]
impl crate::Resettable for IDRrs {
    const RESET_VALUE: u32 = 0x0013_0041;
}
