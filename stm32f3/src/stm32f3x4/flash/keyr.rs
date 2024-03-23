#[doc = "Register `KEYR` writer"]
pub type W = crate::W<KEYRrs>;
#[doc = "Field `FKEYR` writer - Flash Key"]
pub type FKEYR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Flash Key"]
    #[inline(always)]
    #[must_use]
    pub fn fkeyr(&mut self) -> FKEYR_W<KEYRrs> {
        FKEYR_W::new(self, 0)
    }
}
#[doc = "Flash key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYRrs;
impl crate::RegisterSpec for KEYRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keyr::W`](W) writer structure"]
impl crate::Writable for KEYRrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR to value 0"]
impl crate::Resettable for KEYRrs {
    const RESET_VALUE: u32 = 0;
}
