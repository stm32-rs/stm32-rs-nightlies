#[doc = "Register `FLASH_PDKEY2R` writer"]
pub type W = crate::W<FLASH_PDKEY2Rrs>;
#[doc = "Field `PDKEY2` writer - Bank 2 power-down key"]
pub type PDKEY2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Bank 2 power-down key"]
    #[inline(always)]
    #[must_use]
    pub fn pdkey2(&mut self) -> PDKEY2_W<FLASH_PDKEY2Rrs> {
        PDKEY2_W::new(self, 0)
    }
}
#[doc = "FLASH bank 2 power-down key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_pdkey2r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_PDKEY2Rrs;
impl crate::RegisterSpec for FLASH_PDKEY2Rrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flash_pdkey2r::W`](W) writer structure"]
impl crate::Writable for FLASH_PDKEY2Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_PDKEY2R to value 0"]
impl crate::Resettable for FLASH_PDKEY2Rrs {
    const RESET_VALUE: u32 = 0;
}
