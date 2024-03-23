#[doc = "Register `AHB3ENR` reader"]
pub type R = crate::R<AHB3ENRrs>;
#[doc = "Register `AHB3ENR` writer"]
pub type W = crate::W<AHB3ENRrs>;
#[doc = "MDMA Peripheral Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDMAEN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<MDMAEN> for bool {
    #[inline(always)]
    fn from(variant: MDMAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMAEN` reader - MDMA Peripheral Clock Enable"]
pub type MDMAEN_R = crate::BitReader<MDMAEN>;
impl MDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MDMAEN {
        match self.bits {
            false => MDMAEN::Disabled,
            true => MDMAEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MDMAEN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MDMAEN::Enabled
    }
}
#[doc = "Field `MDMAEN` writer - MDMA Peripheral Clock Enable"]
pub type MDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, MDMAEN>;
impl<'a, REG> MDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MDMAEN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MDMAEN::Enabled)
    }
}
#[doc = "Field `DMA2DEN` reader - DMA2D Peripheral Clock Enable"]
pub use MDMAEN_R as DMA2DEN_R;
#[doc = "Field `FMCEN` reader - FMC Peripheral Clocks Enable"]
pub use MDMAEN_R as FMCEN_R;
#[doc = "Field `OCTOSPI1EN` reader - OCTOSPI1 and OCTOSPI1 delay block enable"]
pub use MDMAEN_R as OCTOSPI1EN_R;
#[doc = "Field `SDMMC1EN` reader - SDMMC1 and SDMMC1 Delay Clock Enable"]
pub use MDMAEN_R as SDMMC1EN_R;
#[doc = "Field `OCTOSPI2EN` reader - OCTOSPI2 and OCTOSPI2 delay block enable"]
pub use MDMAEN_R as OCTOSPI2EN_R;
#[doc = "Field `IOMNGREN` reader - OCTOSPI IO manager enable"]
pub use MDMAEN_R as IOMNGREN_R;
#[doc = "Field `OTFD1EN` reader - OTFDEC1 enable"]
pub use MDMAEN_R as OTFD1EN_R;
#[doc = "Field `OTFD2EN` reader - OTFDEC2 enable"]
pub use MDMAEN_R as OTFD2EN_R;
#[doc = "Field `DMA2DEN` writer - DMA2D Peripheral Clock Enable"]
pub use MDMAEN_W as DMA2DEN_W;
#[doc = "Field `FMCEN` writer - FMC Peripheral Clocks Enable"]
pub use MDMAEN_W as FMCEN_W;
#[doc = "Field `OCTOSPI1EN` writer - OCTOSPI1 and OCTOSPI1 delay block enable"]
pub use MDMAEN_W as OCTOSPI1EN_W;
#[doc = "Field `SDMMC1EN` writer - SDMMC1 and SDMMC1 Delay Clock Enable"]
pub use MDMAEN_W as SDMMC1EN_W;
#[doc = "Field `OCTOSPI2EN` writer - OCTOSPI2 and OCTOSPI2 delay block enable"]
pub use MDMAEN_W as OCTOSPI2EN_W;
#[doc = "Field `IOMNGREN` writer - OCTOSPI IO manager enable"]
pub use MDMAEN_W as IOMNGREN_W;
#[doc = "Field `OTFD1EN` writer - OTFDEC1 enable"]
pub use MDMAEN_W as OTFD1EN_W;
#[doc = "Field `OTFD2EN` writer - OTFDEC2 enable"]
pub use MDMAEN_W as OTFD2EN_W;
impl R {
    #[doc = "Bit 0 - MDMA Peripheral Clock Enable"]
    #[inline(always)]
    pub fn mdmaen(&self) -> MDMAEN_R {
        MDMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DMA2D Peripheral Clock Enable"]
    #[inline(always)]
    pub fn dma2den(&self) -> DMA2DEN_R {
        DMA2DEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - OCTOSPI1 and OCTOSPI1 delay block enable"]
    #[inline(always)]
    pub fn octospi1en(&self) -> OCTOSPI1EN_R {
        OCTOSPI1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable"]
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - OCTOSPI2 and OCTOSPI2 delay block enable"]
    #[inline(always)]
    pub fn octospi2en(&self) -> OCTOSPI2EN_R {
        OCTOSPI2EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - OCTOSPI IO manager enable"]
    #[inline(always)]
    pub fn iomngren(&self) -> IOMNGREN_R {
        IOMNGREN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OTFDEC1 enable"]
    #[inline(always)]
    pub fn otfd1en(&self) -> OTFD1EN_R {
        OTFD1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OTFDEC2 enable"]
    #[inline(always)]
    pub fn otfd2en(&self) -> OTFD2EN_R {
        OTFD2EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMA Peripheral Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mdmaen(&mut self) -> MDMAEN_W<AHB3ENRrs> {
        MDMAEN_W::new(self, 0)
    }
    #[doc = "Bit 4 - DMA2D Peripheral Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2den(&mut self) -> DMA2DEN_W<AHB3ENRrs> {
        DMA2DEN_W::new(self, 4)
    }
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FMCEN_W<AHB3ENRrs> {
        FMCEN_W::new(self, 12)
    }
    #[doc = "Bit 14 - OCTOSPI1 and OCTOSPI1 delay block enable"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1en(&mut self) -> OCTOSPI1EN_W<AHB3ENRrs> {
        OCTOSPI1EN_W::new(self, 14)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<AHB3ENRrs> {
        SDMMC1EN_W::new(self, 16)
    }
    #[doc = "Bit 19 - OCTOSPI2 and OCTOSPI2 delay block enable"]
    #[inline(always)]
    #[must_use]
    pub fn octospi2en(&mut self) -> OCTOSPI2EN_W<AHB3ENRrs> {
        OCTOSPI2EN_W::new(self, 19)
    }
    #[doc = "Bit 21 - OCTOSPI IO manager enable"]
    #[inline(always)]
    #[must_use]
    pub fn iomngren(&mut self) -> IOMNGREN_W<AHB3ENRrs> {
        IOMNGREN_W::new(self, 21)
    }
    #[doc = "Bit 22 - OTFDEC1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn otfd1en(&mut self) -> OTFD1EN_W<AHB3ENRrs> {
        OTFD1EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - OTFDEC2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn otfd2en(&mut self) -> OTFD2EN_W<AHB3ENRrs> {
        OTFD2EN_W::new(self, 23)
    }
}
#[doc = "RCC AHB3 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB3ENRrs;
impl crate::RegisterSpec for AHB3ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3enr::R`](R) reader structure"]
impl crate::Readable for AHB3ENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb3enr::W`](W) writer structure"]
impl crate::Writable for AHB3ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB3ENR to value 0"]
impl crate::Resettable for AHB3ENRrs {
    const RESET_VALUE: u32 = 0;
}
