#[doc = "Register `PDKEYR` writer"]
pub type W = crate::W<PDKEYRrs>;
#[doc = "Field `PDKEYR` writer - RUN_PD in FLASH_ACR key"]
pub type PDKEYR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - RUN_PD in FLASH_ACR key"]
    #[inline(always)]
    #[must_use]
    pub fn pdkeyr(&mut self) -> PDKEYR_W<PDKEYRrs> {
        PDKEYR_W::new(self, 0)
    }
}
#[doc = "Power down key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDKEYRrs;
impl crate::RegisterSpec for PDKEYRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pdkeyr::W`](W) writer structure"]
impl crate::Writable for PDKEYRrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDKEYR to value 0"]
impl crate::Resettable for PDKEYRrs {
    const RESET_VALUE: u32 = 0;
}
