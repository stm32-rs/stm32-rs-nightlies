#[doc = "Register `RCC_TZAHB6RSTSETR` reader"]
pub type R = crate::R<RCC_TZAHB6RSTSETRrs>;
#[doc = "Register `RCC_TZAHB6RSTSETR` writer"]
pub type W = crate::W<RCC_TZAHB6RSTSETRrs>;
#[doc = "Field `MDMARST` reader - MDMARST"]
pub type MDMARST_R = crate::BitReader;
#[doc = "Field `MDMARST` writer - MDMARST"]
pub type MDMARST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MDMARST"]
    #[inline(always)]
    pub fn mdmarst(&self) -> MDMARST_R {
        MDMARST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMARST"]
    #[inline(always)]
    #[must_use]
    pub fn mdmarst(&mut self) -> MDMARST_W<RCC_TZAHB6RSTSETRrs> {
        MDMARST_W::new(self, 0)
    }
}
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_tzahb6rstsetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_tzahb6rstsetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_TZAHB6RSTSETRrs;
impl crate::RegisterSpec for RCC_TZAHB6RSTSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_tzahb6rstsetr::R`](R) reader structure"]
impl crate::Readable for RCC_TZAHB6RSTSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_tzahb6rstsetr::W`](W) writer structure"]
impl crate::Writable for RCC_TZAHB6RSTSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_TZAHB6RSTSETR to value 0"]
impl crate::Resettable for RCC_TZAHB6RSTSETRrs {
    const RESET_VALUE: u32 = 0;
}
