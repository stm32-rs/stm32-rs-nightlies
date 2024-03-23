#[doc = "Register `APB2ENR` reader"]
pub type R = crate::R<APB2ENRrs>;
#[doc = "Register `APB2ENR` writer"]
pub type W = crate::W<APB2ENRrs>;
#[doc = "TIM1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1EN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<TIM1EN> for bool {
    #[inline(always)]
    fn from(variant: TIM1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1EN` reader - TIM1 clock enable"]
pub type TIM1EN_R = crate::BitReader<TIM1EN>;
impl TIM1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM1EN {
        match self.bits {
            false => TIM1EN::Disabled,
            true => TIM1EN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM1EN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM1EN::Enabled
    }
}
#[doc = "Field `TIM1EN` writer - TIM1 clock enable"]
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM1EN>;
impl<'a, REG> TIM1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1EN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1EN::Enabled)
    }
}
#[doc = "Field `TIM8EN` reader - TIM8 clock enable"]
pub use TIM1EN_R as TIM8EN_R;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub use TIM1EN_R as USART1EN_R;
#[doc = "Field `USART6EN` reader - USART6 clock enable"]
pub use TIM1EN_R as USART6EN_R;
#[doc = "Field `SDMMC2EN` reader - SDMMC2 clock enable"]
pub use TIM1EN_R as SDMMC2EN_R;
#[doc = "Field `ADC1EN` reader - ADC1 clock enable"]
pub use TIM1EN_R as ADC1EN_R;
#[doc = "Field `ADC2EN` reader - ADC2 clock enable"]
pub use TIM1EN_R as ADC2EN_R;
#[doc = "Field `ADC3EN` reader - ADC3 clock enable"]
pub use TIM1EN_R as ADC3EN_R;
#[doc = "Field `SDMMC1EN` reader - SDMMC1 clock enable"]
pub use TIM1EN_R as SDMMC1EN_R;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub use TIM1EN_R as SPI1EN_R;
#[doc = "Field `SPI4EN` reader - SPI4 clock enable"]
pub use TIM1EN_R as SPI4EN_R;
#[doc = "Field `SYSCFGEN` reader - System configuration controller clock enable"]
pub use TIM1EN_R as SYSCFGEN_R;
#[doc = "Field `TIM9EN` reader - TIM9 clock enable"]
pub use TIM1EN_R as TIM9EN_R;
#[doc = "Field `TIM10EN` reader - TIM10 clock enable"]
pub use TIM1EN_R as TIM10EN_R;
#[doc = "Field `TIM11EN` reader - TIM11 clock enable"]
pub use TIM1EN_R as TIM11EN_R;
#[doc = "Field `SPI5EN` reader - SPI5 clock enable"]
pub use TIM1EN_R as SPI5EN_R;
#[doc = "Field `SPI6EN` reader - SPI6 clock enable"]
pub use TIM1EN_R as SPI6EN_R;
#[doc = "Field `SAI1EN` reader - SAI1 clock enable"]
pub use TIM1EN_R as SAI1EN_R;
#[doc = "Field `SAI2EN` reader - SAI2 clock enable"]
pub use TIM1EN_R as SAI2EN_R;
#[doc = "Field `LTDCEN` reader - LTDC clock enable"]
pub use TIM1EN_R as LTDCEN_R;
#[doc = "Field `DSIEN` reader - DSI clock enable"]
pub use TIM1EN_R as DSIEN_R;
#[doc = "Field `DFSDM1EN` reader - DFSDM1 clock enable"]
pub use TIM1EN_R as DFSDM1EN_R;
#[doc = "Field `MDIOEN` reader - MDIO clock enable"]
pub use TIM1EN_R as MDIOEN_R;
#[doc = "Field `TIM8EN` writer - TIM8 clock enable"]
pub use TIM1EN_W as TIM8EN_W;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub use TIM1EN_W as USART1EN_W;
#[doc = "Field `USART6EN` writer - USART6 clock enable"]
pub use TIM1EN_W as USART6EN_W;
#[doc = "Field `SDMMC2EN` writer - SDMMC2 clock enable"]
pub use TIM1EN_W as SDMMC2EN_W;
#[doc = "Field `ADC1EN` writer - ADC1 clock enable"]
pub use TIM1EN_W as ADC1EN_W;
#[doc = "Field `ADC2EN` writer - ADC2 clock enable"]
pub use TIM1EN_W as ADC2EN_W;
#[doc = "Field `ADC3EN` writer - ADC3 clock enable"]
pub use TIM1EN_W as ADC3EN_W;
#[doc = "Field `SDMMC1EN` writer - SDMMC1 clock enable"]
pub use TIM1EN_W as SDMMC1EN_W;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub use TIM1EN_W as SPI1EN_W;
#[doc = "Field `SPI4EN` writer - SPI4 clock enable"]
pub use TIM1EN_W as SPI4EN_W;
#[doc = "Field `SYSCFGEN` writer - System configuration controller clock enable"]
pub use TIM1EN_W as SYSCFGEN_W;
#[doc = "Field `TIM9EN` writer - TIM9 clock enable"]
pub use TIM1EN_W as TIM9EN_W;
#[doc = "Field `TIM10EN` writer - TIM10 clock enable"]
pub use TIM1EN_W as TIM10EN_W;
#[doc = "Field `TIM11EN` writer - TIM11 clock enable"]
pub use TIM1EN_W as TIM11EN_W;
#[doc = "Field `SPI5EN` writer - SPI5 clock enable"]
pub use TIM1EN_W as SPI5EN_W;
#[doc = "Field `SPI6EN` writer - SPI6 clock enable"]
pub use TIM1EN_W as SPI6EN_W;
#[doc = "Field `SAI1EN` writer - SAI1 clock enable"]
pub use TIM1EN_W as SAI1EN_W;
#[doc = "Field `SAI2EN` writer - SAI2 clock enable"]
pub use TIM1EN_W as SAI2EN_W;
#[doc = "Field `LTDCEN` writer - LTDC clock enable"]
pub use TIM1EN_W as LTDCEN_W;
#[doc = "Field `DSIEN` writer - DSI clock enable"]
pub use TIM1EN_W as DSIEN_W;
#[doc = "Field `DFSDM1EN` writer - DFSDM1 clock enable"]
pub use TIM1EN_W as DFSDM1EN_W;
#[doc = "Field `MDIOEN` writer - MDIO clock enable"]
pub use TIM1EN_W as MDIOEN_W;
impl R {
    #[doc = "Bit 0 - TIM1 clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 clock enable"]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - SDMMC2 clock enable"]
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC2 clock enable"]
    #[inline(always)]
    pub fn adc2en(&self) -> ADC2EN_R {
        ADC2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC3 clock enable"]
    #[inline(always)]
    pub fn adc3en(&self) -> ADC3EN_R {
        ADC3EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SDMMC1 clock enable"]
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 clock enable"]
    #[inline(always)]
    pub fn spi4en(&self) -> SPI4EN_R {
        SPI4EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - System configuration controller clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM9 clock enable"]
    #[inline(always)]
    pub fn tim9en(&self) -> TIM9EN_R {
        TIM9EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM10 clock enable"]
    #[inline(always)]
    pub fn tim10en(&self) -> TIM10EN_R {
        TIM10EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM11 clock enable"]
    #[inline(always)]
    pub fn tim11en(&self) -> TIM11EN_R {
        TIM11EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI5 clock enable"]
    #[inline(always)]
    pub fn spi5en(&self) -> SPI5EN_R {
        SPI5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SPI6 clock enable"]
    #[inline(always)]
    pub fn spi6en(&self) -> SPI6EN_R {
        SPI6EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI1 clock enable"]
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SAI2 clock enable"]
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - LTDC clock enable"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DSI clock enable"]
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - DFSDM1 clock enable"]
    #[inline(always)]
    pub fn dfsdm1en(&self) -> DFSDM1EN_R {
        DFSDM1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MDIO clock enable"]
    #[inline(always)]
    pub fn mdioen(&self) -> MDIOEN_R {
        MDIOEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<APB2ENRrs> {
        TIM1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM8 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim8en(&mut self) -> TIM8EN_W<APB2ENRrs> {
        TIM8EN_W::new(self, 1)
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<APB2ENRrs> {
        USART1EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart6en(&mut self) -> USART6EN_W<APB2ENRrs> {
        USART6EN_W::new(self, 5)
    }
    #[doc = "Bit 7 - SDMMC2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<APB2ENRrs> {
        SDMMC2EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1en(&mut self) -> ADC1EN_W<APB2ENRrs> {
        ADC1EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - ADC2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc2en(&mut self) -> ADC2EN_W<APB2ENRrs> {
        ADC2EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - ADC3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc3en(&mut self) -> ADC3EN_W<APB2ENRrs> {
        ADC3EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - SDMMC1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<APB2ENRrs> {
        SDMMC1EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - SPI4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi4en(&mut self) -> SPI4EN_W<APB2ENRrs> {
        SPI4EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - System configuration controller clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<APB2ENRrs> {
        SYSCFGEN_W::new(self, 14)
    }
    #[doc = "Bit 16 - TIM9 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim9en(&mut self) -> TIM9EN_W<APB2ENRrs> {
        TIM9EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM10 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim10en(&mut self) -> TIM10EN_W<APB2ENRrs> {
        TIM10EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM11 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim11en(&mut self) -> TIM11EN_W<APB2ENRrs> {
        TIM11EN_W::new(self, 18)
    }
    #[doc = "Bit 20 - SPI5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi5en(&mut self) -> SPI5EN_W<APB2ENRrs> {
        SPI5EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - SPI6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi6en(&mut self) -> SPI6EN_W<APB2ENRrs> {
        SPI6EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - SAI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sai1en(&mut self) -> SAI1EN_W<APB2ENRrs> {
        SAI1EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - SAI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sai2en(&mut self) -> SAI2EN_W<APB2ENRrs> {
        SAI2EN_W::new(self, 23)
    }
    #[doc = "Bit 26 - LTDC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<APB2ENRrs> {
        LTDCEN_W::new(self, 26)
    }
    #[doc = "Bit 27 - DSI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsien(&mut self) -> DSIEN_W<APB2ENRrs> {
        DSIEN_W::new(self, 27)
    }
    #[doc = "Bit 29 - DFSDM1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1en(&mut self) -> DFSDM1EN_W<APB2ENRrs> {
        DFSDM1EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - MDIO clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn mdioen(&mut self) -> MDIOEN_W<APB2ENRrs> {
        MDIOEN_W::new(self, 30)
    }
}
#[doc = "APB2 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2ENRrs;
impl crate::RegisterSpec for APB2ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2enr::R`](R) reader structure"]
impl crate::Readable for APB2ENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb2enr::W`](W) writer structure"]
impl crate::Writable for APB2ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2ENR to value 0"]
impl crate::Resettable for APB2ENRrs {
    const RESET_VALUE: u32 = 0;
}
