#[doc = "Register `OPTKEYR_` writer"]
pub type W = crate::W<OPTKEYR_rs>;
#[doc = "Field `OPTKEYR` writer - FLASH option bytes control access unlock key"]
pub type OPTKEYR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - FLASH option bytes control access unlock key"]
    #[inline(always)]
    #[must_use]
    pub fn optkeyr(&mut self) -> OPTKEYR_W<OPTKEYR_rs> {
        OPTKEYR_W::new(self, 0)
    }
}
#[doc = "FLASH option key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optkeyr_::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTKEYR_rs;
impl crate::RegisterSpec for OPTKEYR_rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`optkeyr_::W`](W) writer structure"]
impl crate::Writable for OPTKEYR_rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTKEYR_ to value 0"]
impl crate::Resettable for OPTKEYR_rs {
    const RESET_VALUE: u32 = 0;
}
