#[doc = "Register `RAM3ECCKEYR` writer"]
pub type W = crate::W<RAM3ECCKEYRrs>;
#[doc = "Field `ECCKEY` writer - ECCKEY"]
pub type ECCKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - ECCKEY"]
    #[inline(always)]
    #[must_use]
    pub fn ecckey(&mut self) -> ECCKEY_W<RAM3ECCKEYRrs> {
        ECCKEY_W::new(self, 0)
    }
}
#[doc = "RAMCFG SRAM x ECC key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram3ecckeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAM3ECCKEYRrs;
impl crate::RegisterSpec for RAM3ECCKEYRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ram3ecckeyr::W`](W) writer structure"]
impl crate::Writable for RAM3ECCKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAM3ECCKEYR to value 0"]
impl crate::Resettable for RAM3ECCKEYRrs {
    const RESET_VALUE: u32 = 0;
}
