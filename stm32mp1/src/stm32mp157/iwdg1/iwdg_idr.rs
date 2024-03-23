#[doc = "Register `IWDG_IDR` reader"]
pub type R = crate::R<IWDG_IDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "IWDG identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_idr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IWDG_IDRrs;
impl crate::RegisterSpec for IWDG_IDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_idr::R`](R) reader structure"]
impl crate::Readable for IWDG_IDRrs {}
#[doc = "`reset()` method sets IWDG_IDR to value 0x0012_0041"]
impl crate::Resettable for IWDG_IDRrs {
    const RESET_VALUE: u32 = 0x0012_0041;
}
