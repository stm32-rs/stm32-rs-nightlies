#[doc = "Register `M5ECCKEYR` reader"]
pub type R = crate::R<M5ECCKEYRrs>;
#[doc = "Register `M5ECCKEYR` writer"]
pub type W = crate::W<M5ECCKEYRrs>;
#[doc = "Field `ECCKEY` reader - ECCKEY"]
pub type ECCKEY_R = crate::FieldReader;
#[doc = "Field `ECCKEY` writer - ECCKEY"]
pub type ECCKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ECCKEY"]
    #[inline(always)]
    pub fn ecckey(&self) -> ECCKEY_R {
        ECCKEY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ECCKEY"]
    #[inline(always)]
    #[must_use]
    pub fn ecckey(&mut self) -> ECCKEY_W<M5ECCKEYRrs> {
        ECCKEY_W::new(self, 0)
    }
}
#[doc = "RAMCFG RAM x interrupt clear register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5ecckeyr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5ecckeyr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M5ECCKEYRrs;
impl crate::RegisterSpec for M5ECCKEYRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m5ecckeyr::R`](R) reader structure"]
impl crate::Readable for M5ECCKEYRrs {}
#[doc = "`write(|w| ..)` method takes [`m5ecckeyr::W`](W) writer structure"]
impl crate::Writable for M5ECCKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M5ECCKEYR to value 0"]
impl crate::Resettable for M5ECCKEYRrs {
    const RESET_VALUE: u32 = 0;
}
