#[doc = "Register `SECKEYR` writer"]
pub type W = crate::W<SECKEYRrs>;
#[doc = "Field `SECKEYR` writer - SECKEYR"]
pub type SECKEYR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - SECKEYR"]
    #[inline(always)]
    #[must_use]
    pub fn seckeyr(&mut self) -> SECKEYR_W<SECKEYRrs> {
        SECKEYR_W::new(self, 0)
    }
}
#[doc = "Flash secure key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seckeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECKEYRrs;
impl crate::RegisterSpec for SECKEYRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`seckeyr::W`](W) writer structure"]
impl crate::Writable for SECKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECKEYR to value 0"]
impl crate::Resettable for SECKEYRrs {
    const RESET_VALUE: u32 = 0;
}
