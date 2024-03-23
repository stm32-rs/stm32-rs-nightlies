#[doc = "Register `IER2` reader"]
pub type R = crate::R<IER2rs>;
#[doc = "Register `IER2` writer"]
pub type W = crate::W<IER2rs>;
#[doc = "Field `SYSCFGIE` reader - illegal access interrupt enable for SYSCFG"]
pub type SYSCFGIE_R = crate::BitReader;
#[doc = "Field `SYSCFGIE` writer - illegal access interrupt enable for SYSCFG"]
pub type SYSCFGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCIE` reader - illegal access interrupt enable for RTC"]
pub type RTCIE_R = crate::BitReader;
#[doc = "Field `RTCIE` writer - illegal access interrupt enable for RTC"]
pub type RTCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPIE` reader - illegal access interrupt enable for TAMP"]
pub type TAMPIE_R = crate::BitReader;
#[doc = "Field `TAMPIE` writer - illegal access interrupt enable for TAMP"]
pub type TAMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRIE` reader - illegal access interrupt enable for PWR"]
pub type PWRIE_R = crate::BitReader;
#[doc = "Field `PWRIE` writer - illegal access interrupt enable for PWR"]
pub type PWRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCCIE` reader - illegal access interrupt enable for RCC"]
pub type RCCIE_R = crate::BitReader;
#[doc = "Field `RCCIE` writer - illegal access interrupt enable for RCC"]
pub type RCCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPDMA1IE` reader - illegal access interrupt enable for LPDMA"]
pub type LPDMA1IE_R = crate::BitReader;
#[doc = "Field `LPDMA1IE` writer - illegal access interrupt enable for LPDMA"]
pub type LPDMA1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIIE` reader - illegal access interrupt enable for EXTI"]
pub type EXTIIE_R = crate::BitReader;
#[doc = "Field `EXTIIE` writer - illegal access interrupt enable for EXTI"]
pub type EXTIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSC2IE` reader - illegal access interrupt enable for GTZC2 TZSC registers"]
pub type TZSC2IE_R = crate::BitReader;
#[doc = "Field `TZSC2IE` writer - illegal access interrupt enable for GTZC2 TZSC registers"]
pub type TZSC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZIC2IE` reader - illegal access interrupt enable for GTZC2 TZIC registers"]
pub type TZIC2IE_R = crate::BitReader;
#[doc = "Field `TZIC2IE` writer - illegal access interrupt enable for GTZC2 TZIC registers"]
pub type TZIC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM4IE` reader - illegal access interrupt enable for SRAM4"]
pub type SRAM4IE_R = crate::BitReader;
#[doc = "Field `SRAM4IE` writer - illegal access interrupt enable for SRAM4"]
pub type SRAM4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCBB4_REGIE` reader - illegal access interrupt enable for MPCBB4 registers"]
pub type MPCBB4_REGIE_R = crate::BitReader;
#[doc = "Field `MPCBB4_REGIE` writer - illegal access interrupt enable for MPCBB4 registers"]
pub type MPCBB4_REGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - illegal access interrupt enable for SYSCFG"]
    #[inline(always)]
    pub fn syscfgie(&self) -> SYSCFGIE_R {
        SYSCFGIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - illegal access interrupt enable for RTC"]
    #[inline(always)]
    pub fn rtcie(&self) -> RTCIE_R {
        RTCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - illegal access interrupt enable for TAMP"]
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - illegal access interrupt enable for PWR"]
    #[inline(always)]
    pub fn pwrie(&self) -> PWRIE_R {
        PWRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - illegal access interrupt enable for RCC"]
    #[inline(always)]
    pub fn rccie(&self) -> RCCIE_R {
        RCCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - illegal access interrupt enable for LPDMA"]
    #[inline(always)]
    pub fn lpdma1ie(&self) -> LPDMA1IE_R {
        LPDMA1IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - illegal access interrupt enable for EXTI"]
    #[inline(always)]
    pub fn extiie(&self) -> EXTIIE_R {
        EXTIIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - illegal access interrupt enable for GTZC2 TZSC registers"]
    #[inline(always)]
    pub fn tzsc2ie(&self) -> TZSC2IE_R {
        TZSC2IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - illegal access interrupt enable for GTZC2 TZIC registers"]
    #[inline(always)]
    pub fn tzic2ie(&self) -> TZIC2IE_R {
        TZIC2IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - illegal access interrupt enable for SRAM4"]
    #[inline(always)]
    pub fn sram4ie(&self) -> SRAM4IE_R {
        SRAM4IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - illegal access interrupt enable for MPCBB4 registers"]
    #[inline(always)]
    pub fn mpcbb4_regie(&self) -> MPCBB4_REGIE_R {
        MPCBB4_REGIE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - illegal access interrupt enable for SYSCFG"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgie(&mut self) -> SYSCFGIE_W<IER2rs> {
        SYSCFGIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - illegal access interrupt enable for RTC"]
    #[inline(always)]
    #[must_use]
    pub fn rtcie(&mut self) -> RTCIE_W<IER2rs> {
        RTCIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - illegal access interrupt enable for TAMP"]
    #[inline(always)]
    #[must_use]
    pub fn tampie(&mut self) -> TAMPIE_W<IER2rs> {
        TAMPIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - illegal access interrupt enable for PWR"]
    #[inline(always)]
    #[must_use]
    pub fn pwrie(&mut self) -> PWRIE_W<IER2rs> {
        PWRIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - illegal access interrupt enable for RCC"]
    #[inline(always)]
    #[must_use]
    pub fn rccie(&mut self) -> RCCIE_W<IER2rs> {
        RCCIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - illegal access interrupt enable for LPDMA"]
    #[inline(always)]
    #[must_use]
    pub fn lpdma1ie(&mut self) -> LPDMA1IE_W<IER2rs> {
        LPDMA1IE_W::new(self, 5)
    }
    #[doc = "Bit 6 - illegal access interrupt enable for EXTI"]
    #[inline(always)]
    #[must_use]
    pub fn extiie(&mut self) -> EXTIIE_W<IER2rs> {
        EXTIIE_W::new(self, 6)
    }
    #[doc = "Bit 14 - illegal access interrupt enable for GTZC2 TZSC registers"]
    #[inline(always)]
    #[must_use]
    pub fn tzsc2ie(&mut self) -> TZSC2IE_W<IER2rs> {
        TZSC2IE_W::new(self, 14)
    }
    #[doc = "Bit 15 - illegal access interrupt enable for GTZC2 TZIC registers"]
    #[inline(always)]
    #[must_use]
    pub fn tzic2ie(&mut self) -> TZIC2IE_W<IER2rs> {
        TZIC2IE_W::new(self, 15)
    }
    #[doc = "Bit 24 - illegal access interrupt enable for SRAM4"]
    #[inline(always)]
    #[must_use]
    pub fn sram4ie(&mut self) -> SRAM4IE_W<IER2rs> {
        SRAM4IE_W::new(self, 24)
    }
    #[doc = "Bit 25 - illegal access interrupt enable for MPCBB4 registers"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb4_regie(&mut self) -> MPCBB4_REGIE_W<IER2rs> {
        MPCBB4_REGIE_W::new(self, 25)
    }
}
#[doc = "TZIC interrupt enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER2rs;
impl crate::RegisterSpec for IER2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier2::R`](R) reader structure"]
impl crate::Readable for IER2rs {}
#[doc = "`write(|w| ..)` method takes [`ier2::W`](W) writer structure"]
impl crate::Writable for IER2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER2 to value 0"]
impl crate::Resettable for IER2rs {
    const RESET_VALUE: u32 = 0;
}
