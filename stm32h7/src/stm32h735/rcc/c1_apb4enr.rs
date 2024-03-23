#[doc = "Register `C1_APB4ENR` reader"]
pub type R = crate::R<C1_APB4ENRrs>;
#[doc = "Register `C1_APB4ENR` writer"]
pub type W = crate::W<C1_APB4ENRrs>;
#[doc = "SYSCFG peripheral clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGEN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<SYSCFGEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGEN` reader - SYSCFG peripheral clock enable"]
pub type SYSCFGEN_R = crate::BitReader<SYSCFGEN>;
impl SYSCFGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGEN {
        match self.bits {
            false => SYSCFGEN::Disabled,
            true => SYSCFGEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGEN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGEN::Enabled
    }
}
#[doc = "Field `SYSCFGEN` writer - SYSCFG peripheral clock enable"]
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGEN>;
impl<'a, REG> SYSCFGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::Enabled)
    }
}
#[doc = "Field `LPUART1EN` reader - LPUART1 Peripheral Clocks Enable"]
pub use SYSCFGEN_R as LPUART1EN_R;
#[doc = "Field `SPI6EN` reader - SPI6 Peripheral Clocks Enable"]
pub use SYSCFGEN_R as SPI6EN_R;
#[doc = "Field `I2C4EN` reader - I2C4 Peripheral Clocks Enable"]
pub use SYSCFGEN_R as I2C4EN_R;
#[doc = "Field `LPTIM2EN` reader - LPTIM2 Peripheral Clocks Enable"]
pub use SYSCFGEN_R as LPTIM2EN_R;
#[doc = "Field `LPTIM3EN` reader - LPTIM3 Peripheral Clocks Enable"]
pub use SYSCFGEN_R as LPTIM3EN_R;
#[doc = "Field `LPTIM4EN` reader - LPTIM4 Peripheral Clocks Enable"]
pub use SYSCFGEN_R as LPTIM4EN_R;
#[doc = "Field `LPTIM5EN` reader - LPTIM5 Peripheral Clocks Enable"]
pub use SYSCFGEN_R as LPTIM5EN_R;
#[doc = "Field `COMP12EN` reader - COMP1/2 peripheral clock enable"]
pub use SYSCFGEN_R as COMP12EN_R;
#[doc = "Field `VREFEN` reader - VREF peripheral clock enable"]
pub use SYSCFGEN_R as VREFEN_R;
#[doc = "Field `RTCAPBEN` reader - RTC APB Clock Enable"]
pub use SYSCFGEN_R as RTCAPBEN_R;
#[doc = "Field `SAI4EN` reader - SAI4 Peripheral Clocks Enable"]
pub use SYSCFGEN_R as SAI4EN_R;
#[doc = "Field `LPUART1EN` writer - LPUART1 Peripheral Clocks Enable"]
pub use SYSCFGEN_W as LPUART1EN_W;
#[doc = "Field `SPI6EN` writer - SPI6 Peripheral Clocks Enable"]
pub use SYSCFGEN_W as SPI6EN_W;
#[doc = "Field `I2C4EN` writer - I2C4 Peripheral Clocks Enable"]
pub use SYSCFGEN_W as I2C4EN_W;
#[doc = "Field `LPTIM2EN` writer - LPTIM2 Peripheral Clocks Enable"]
pub use SYSCFGEN_W as LPTIM2EN_W;
#[doc = "Field `LPTIM3EN` writer - LPTIM3 Peripheral Clocks Enable"]
pub use SYSCFGEN_W as LPTIM3EN_W;
#[doc = "Field `LPTIM4EN` writer - LPTIM4 Peripheral Clocks Enable"]
pub use SYSCFGEN_W as LPTIM4EN_W;
#[doc = "Field `LPTIM5EN` writer - LPTIM5 Peripheral Clocks Enable"]
pub use SYSCFGEN_W as LPTIM5EN_W;
#[doc = "Field `COMP12EN` writer - COMP1/2 peripheral clock enable"]
pub use SYSCFGEN_W as COMP12EN_W;
#[doc = "Field `VREFEN` writer - VREF peripheral clock enable"]
pub use SYSCFGEN_W as VREFEN_W;
#[doc = "Field `RTCAPBEN` writer - RTC APB Clock Enable"]
pub use SYSCFGEN_W as RTCAPBEN_W;
#[doc = "Field `SAI4EN` writer - SAI4 Peripheral Clocks Enable"]
pub use SYSCFGEN_W as SAI4EN_W;
impl R {
    #[doc = "Bit 1 - SYSCFG peripheral clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - LPUART1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI6 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi6en(&self) -> SPI6EN_R {
        SPI6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim4en(&self) -> LPTIM4EN_R {
        LPTIM4EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim5en(&self) -> LPTIM5EN_R {
        LPTIM5EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - COMP1/2 peripheral clock enable"]
    #[inline(always)]
    pub fn comp12en(&self) -> COMP12EN_R {
        COMP12EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VREF peripheral clock enable"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC APB Clock Enable"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn sai4en(&self) -> SAI4EN_R {
        SAI4EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SYSCFG peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<C1_APB4ENRrs> {
        SYSCFGEN_W::new(self, 1)
    }
    #[doc = "Bit 3 - LPUART1 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<C1_APB4ENRrs> {
        LPUART1EN_W::new(self, 3)
    }
    #[doc = "Bit 5 - SPI6 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi6en(&mut self) -> SPI6EN_W<C1_APB4ENRrs> {
        SPI6EN_W::new(self, 5)
    }
    #[doc = "Bit 7 - I2C4 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4en(&mut self) -> I2C4EN_W<C1_APB4ENRrs> {
        I2C4EN_W::new(self, 7)
    }
    #[doc = "Bit 9 - LPTIM2 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<C1_APB4ENRrs> {
        LPTIM2EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - LPTIM3 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<C1_APB4ENRrs> {
        LPTIM3EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - LPTIM4 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4en(&mut self) -> LPTIM4EN_W<C1_APB4ENRrs> {
        LPTIM4EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - LPTIM5 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim5en(&mut self) -> LPTIM5EN_W<C1_APB4ENRrs> {
        LPTIM5EN_W::new(self, 12)
    }
    #[doc = "Bit 14 - COMP1/2 peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp12en(&mut self) -> COMP12EN_W<C1_APB4ENRrs> {
        COMP12EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - VREF peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<C1_APB4ENRrs> {
        VREFEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - RTC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<C1_APB4ENRrs> {
        RTCAPBEN_W::new(self, 16)
    }
    #[doc = "Bit 21 - SAI4 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sai4en(&mut self) -> SAI4EN_W<C1_APB4ENRrs> {
        SAI4EN_W::new(self, 21)
    }
}
#[doc = "RCC APB4 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb4enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb4enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1_APB4ENRrs;
impl crate::RegisterSpec for C1_APB4ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_apb4enr::R`](R) reader structure"]
impl crate::Readable for C1_APB4ENRrs {}
#[doc = "`write(|w| ..)` method takes [`c1_apb4enr::W`](W) writer structure"]
impl crate::Writable for C1_APB4ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1_APB4ENR to value 0x0001_0000"]
impl crate::Resettable for C1_APB4ENRrs {
    const RESET_VALUE: u32 = 0x0001_0000;
}
