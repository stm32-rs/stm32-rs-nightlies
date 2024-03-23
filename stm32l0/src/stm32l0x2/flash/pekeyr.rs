#[doc = "Register `PEKEYR` writer"]
pub type W = crate::W<PEKEYRrs>;
#[doc = "Field `PEKEYR` writer - FLASH_PEC and data EEPROM key"]
pub type PEKEYR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - FLASH_PEC and data EEPROM key"]
    #[inline(always)]
    #[must_use]
    pub fn pekeyr(&mut self) -> PEKEYR_W<PEKEYRrs> {
        PEKEYR_W::new(self, 0)
    }
}
#[doc = "Program/erase key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pekeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PEKEYRrs;
impl crate::RegisterSpec for PEKEYRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pekeyr::W`](W) writer structure"]
impl crate::Writable for PEKEYRrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PEKEYR to value 0"]
impl crate::Resettable for PEKEYRrs {
    const RESET_VALUE: u32 = 0;
}
