#[doc = "Register `RCC_MP_TZAHB6ENSETR` reader"]
pub type R = crate::R<RCC_MP_TZAHB6ENSETRrs>;
#[doc = "Register `RCC_MP_TZAHB6ENSETR` writer"]
pub type W = crate::W<RCC_MP_TZAHB6ENSETRrs>;
#[doc = "Field `MDMAEN` reader - MDMAEN"]
pub type MDMAEN_R = crate::BitReader;
#[doc = "Field `MDMAEN` writer - MDMAEN"]
pub type MDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MDMAEN"]
    #[inline(always)]
    pub fn mdmaen(&self) -> MDMAEN_R {
        MDMAEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn mdmaen(&mut self) -> MDMAEN_W<RCC_MP_TZAHB6ENSETRrs> {
        MDMAEN_W::new(self, 0)
    }
}
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_tzahb6ensetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_tzahb6ensetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_TZAHB6ENSETRrs;
impl crate::RegisterSpec for RCC_MP_TZAHB6ENSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_tzahb6ensetr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_TZAHB6ENSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_tzahb6ensetr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_TZAHB6ENSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_TZAHB6ENSETR to value 0"]
impl crate::Resettable for RCC_MP_TZAHB6ENSETRrs {
    const RESET_VALUE: u32 = 0;
}
