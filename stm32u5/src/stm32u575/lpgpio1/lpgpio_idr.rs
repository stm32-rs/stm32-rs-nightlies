#[doc = "Register `LPGPIO_IDR` reader"]
pub type R = crate::R<LPGPIO_IDRrs>;
#[doc = "Field `IDy` reader - IDy"]
pub type IDY_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IDy"]
    #[inline(always)]
    pub fn idy(&self) -> IDY_R {
        IDY_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "LPGPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpgpio_idr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPGPIO_IDRrs;
impl crate::RegisterSpec for LPGPIO_IDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpgpio_idr::R`](R) reader structure"]
impl crate::Readable for LPGPIO_IDRrs {}
#[doc = "`reset()` method sets LPGPIO_IDR to value 0"]
impl crate::Resettable for LPGPIO_IDRrs {
    const RESET_VALUE: u32 = 0;
}
