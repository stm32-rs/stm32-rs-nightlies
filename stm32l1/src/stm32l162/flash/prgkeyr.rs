#[doc = "Register `PRGKEYR` writer"]
pub type W = crate::W<PRGKEYRrs>;
#[doc = "Field `PRGKEYR` writer - Program memory key"]
pub type PRGKEYR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Program memory key"]
    #[inline(always)]
    #[must_use]
    pub fn prgkeyr(&mut self) -> PRGKEYR_W<PRGKEYRrs> {
        PRGKEYR_W::new(self, 0)
    }
}
#[doc = "Program memory key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prgkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRGKEYRrs;
impl crate::RegisterSpec for PRGKEYRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prgkeyr::W`](W) writer structure"]
impl crate::Writable for PRGKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRGKEYR to value 0"]
impl crate::Resettable for PRGKEYRrs {
    const RESET_VALUE: u32 = 0;
}
