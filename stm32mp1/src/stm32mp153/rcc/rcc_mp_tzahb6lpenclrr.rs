#[doc = "Register `RCC_MP_TZAHB6LPENCLRR` reader"]
pub type R = crate::R<RCC_MP_TZAHB6LPENCLRRrs>;
#[doc = "Register `RCC_MP_TZAHB6LPENCLRR` writer"]
pub type W = crate::W<RCC_MP_TZAHB6LPENCLRRrs>;
#[doc = "Field `MDMALPEN` reader - MDMALPEN"]
pub type MDMALPEN_R = crate::BitReader;
#[doc = "Field `MDMALPEN` writer - MDMALPEN"]
pub type MDMALPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MDMALPEN"]
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMALPEN"]
    #[inline(always)]
    #[must_use]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W<RCC_MP_TZAHB6LPENCLRRrs> {
        MDMALPEN_W::new(self, 0)
    }
}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_tzahb6lpenclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_tzahb6lpenclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_TZAHB6LPENCLRRrs;
impl crate::RegisterSpec for RCC_MP_TZAHB6LPENCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_tzahb6lpenclrr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_TZAHB6LPENCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_tzahb6lpenclrr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_TZAHB6LPENCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_TZAHB6LPENCLRR to value 0x01"]
impl crate::Resettable for RCC_MP_TZAHB6LPENCLRRrs {
    const RESET_VALUE: u32 = 0x01;
}
