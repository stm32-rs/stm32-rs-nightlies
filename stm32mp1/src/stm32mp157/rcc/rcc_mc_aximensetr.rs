#[doc = "Register `RCC_MC_AXIMENSETR` reader"]
pub type R = crate::R<RCC_MC_AXIMENSETRrs>;
#[doc = "Register `RCC_MC_AXIMENSETR` writer"]
pub type W = crate::W<RCC_MC_AXIMENSETRrs>;
#[doc = "Field `SYSRAMEN` reader - SYSRAMEN"]
pub type SYSRAMEN_R = crate::BitReader;
#[doc = "Field `SYSRAMEN` writer - SYSRAMEN"]
pub type SYSRAMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSRAMEN"]
    #[inline(always)]
    pub fn sysramen(&self) -> SYSRAMEN_R {
        SYSRAMEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSRAMEN"]
    #[inline(always)]
    #[must_use]
    pub fn sysramen(&mut self) -> SYSRAMEN_W<RCC_MC_AXIMENSETRrs> {
        SYSRAMEN_W::new(self, 0)
    }
}
#[doc = "This register is used to set the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_aximensetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_aximensetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MC_AXIMENSETRrs;
impl crate::RegisterSpec for RCC_MC_AXIMENSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mc_aximensetr::R`](R) reader structure"]
impl crate::Readable for RCC_MC_AXIMENSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mc_aximensetr::W`](W) writer structure"]
impl crate::Writable for RCC_MC_AXIMENSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MC_AXIMENSETR to value 0"]
impl crate::Resettable for RCC_MC_AXIMENSETRrs {
    const RESET_VALUE: u32 = 0;
}
