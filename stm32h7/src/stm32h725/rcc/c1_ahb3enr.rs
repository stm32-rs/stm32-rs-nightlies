///Register `C1_AHB3ENR` reader
pub type R = crate::R<C1_AHB3ENRrs>;
///Register `C1_AHB3ENR` writer
pub type W = crate::W<C1_AHB3ENRrs>;
/**MDMA Peripheral Clock Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDMAEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<MDMAEN> for bool {
    #[inline(always)]
    fn from(variant: MDMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `MDMAEN` reader - MDMA Peripheral Clock Enable
pub type MDMAEN_R = crate::BitReader<MDMAEN>;
impl MDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MDMAEN {
        match self.bits {
            false => MDMAEN::Disabled,
            true => MDMAEN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MDMAEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MDMAEN::Enabled
    }
}
///Field `MDMAEN` writer - MDMA Peripheral Clock Enable
pub type MDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, MDMAEN>;
impl<'a, REG> MDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MDMAEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MDMAEN::Enabled)
    }
}
///Field `DMA2DEN` reader - DMA2D Peripheral Clock Enable
pub use MDMAEN_R as DMA2DEN_R;
///Field `FMCEN` reader - FMC Peripheral Clocks Enable
pub use MDMAEN_R as FMCEN_R;
///Field `OCTOSPI1EN` reader - OCTOSPI1 and OCTOSPI1 delay clock enable
pub use MDMAEN_R as OCTOSPI1EN_R;
///Field `SDMMC1EN` reader - SDMMC1 and SDMMC1 Delay Clock Enable
pub use MDMAEN_R as SDMMC1EN_R;
///Field `OCTOSPI2EN` reader - OCTOSPI2 clock enable Set and reset by software.
pub use MDMAEN_R as OCTOSPI2EN_R;
///Field `IOMNGREN` reader - OCTOSPIM clock enable Set and reset by software.
pub use MDMAEN_R as IOMNGREN_R;
///Field `OTFD1EN` reader - OTFD1 clock enable Set and reset by software.
pub use MDMAEN_R as OTFD1EN_R;
///Field `OTFD2EN` reader - OTFD2 clock enable Set and reset by software.
pub use MDMAEN_R as OTFD2EN_R;
///Field `DMA2DEN` writer - DMA2D Peripheral Clock Enable
pub use MDMAEN_W as DMA2DEN_W;
///Field `FMCEN` writer - FMC Peripheral Clocks Enable
pub use MDMAEN_W as FMCEN_W;
///Field `OCTOSPI1EN` writer - OCTOSPI1 and OCTOSPI1 delay clock enable
pub use MDMAEN_W as OCTOSPI1EN_W;
///Field `SDMMC1EN` writer - SDMMC1 and SDMMC1 Delay Clock Enable
pub use MDMAEN_W as SDMMC1EN_W;
///Field `OCTOSPI2EN` writer - OCTOSPI2 clock enable Set and reset by software.
pub use MDMAEN_W as OCTOSPI2EN_W;
///Field `IOMNGREN` writer - OCTOSPIM clock enable Set and reset by software.
pub use MDMAEN_W as IOMNGREN_W;
///Field `OTFD1EN` writer - OTFD1 clock enable Set and reset by software.
pub use MDMAEN_W as OTFD1EN_W;
///Field `OTFD2EN` writer - OTFD2 clock enable Set and reset by software.
pub use MDMAEN_W as OTFD2EN_W;
impl R {
    ///Bit 0 - MDMA Peripheral Clock Enable
    #[inline(always)]
    pub fn mdmaen(&self) -> MDMAEN_R {
        MDMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - DMA2D Peripheral Clock Enable
    #[inline(always)]
    pub fn dma2den(&self) -> DMA2DEN_R {
        DMA2DEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 12 - FMC Peripheral Clocks Enable
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - OCTOSPI1 and OCTOSPI1 delay clock enable
    #[inline(always)]
    pub fn octospi1en(&self) -> OCTOSPI1EN_R {
        OCTOSPI1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 19 - OCTOSPI2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn octospi2en(&self) -> OCTOSPI2EN_R {
        OCTOSPI2EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - OCTOSPIM clock enable Set and reset by software.
    #[inline(always)]
    pub fn iomngren(&self) -> IOMNGREN_R {
        IOMNGREN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - OTFD1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn otfd1en(&self) -> OTFD1EN_R {
        OTFD1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - OTFD2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn otfd2en(&self) -> OTFD2EN_R {
        OTFD2EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1_AHB3ENR")
            .field("mdmaen", &self.mdmaen())
            .field("dma2den", &self.dma2den())
            .field("fmcen", &self.fmcen())
            .field("octospi1en", &self.octospi1en())
            .field("sdmmc1en", &self.sdmmc1en())
            .field("octospi2en", &self.octospi2en())
            .field("iomngren", &self.iomngren())
            .field("otfd1en", &self.otfd1en())
            .field("otfd2en", &self.otfd2en())
            .finish()
    }
}
impl W {
    ///Bit 0 - MDMA Peripheral Clock Enable
    #[inline(always)]
    pub fn mdmaen(&mut self) -> MDMAEN_W<'_, C1_AHB3ENRrs> {
        MDMAEN_W::new(self, 0)
    }
    ///Bit 4 - DMA2D Peripheral Clock Enable
    #[inline(always)]
    pub fn dma2den(&mut self) -> DMA2DEN_W<'_, C1_AHB3ENRrs> {
        DMA2DEN_W::new(self, 4)
    }
    ///Bit 12 - FMC Peripheral Clocks Enable
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W<'_, C1_AHB3ENRrs> {
        FMCEN_W::new(self, 12)
    }
    ///Bit 14 - OCTOSPI1 and OCTOSPI1 delay clock enable
    #[inline(always)]
    pub fn octospi1en(&mut self) -> OCTOSPI1EN_W<'_, C1_AHB3ENRrs> {
        OCTOSPI1EN_W::new(self, 14)
    }
    ///Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable
    #[inline(always)]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<'_, C1_AHB3ENRrs> {
        SDMMC1EN_W::new(self, 16)
    }
    ///Bit 19 - OCTOSPI2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn octospi2en(&mut self) -> OCTOSPI2EN_W<'_, C1_AHB3ENRrs> {
        OCTOSPI2EN_W::new(self, 19)
    }
    ///Bit 21 - OCTOSPIM clock enable Set and reset by software.
    #[inline(always)]
    pub fn iomngren(&mut self) -> IOMNGREN_W<'_, C1_AHB3ENRrs> {
        IOMNGREN_W::new(self, 21)
    }
    ///Bit 22 - OTFD1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn otfd1en(&mut self) -> OTFD1EN_W<'_, C1_AHB3ENRrs> {
        OTFD1EN_W::new(self, 22)
    }
    ///Bit 23 - OTFD2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn otfd2en(&mut self) -> OTFD2EN_W<'_, C1_AHB3ENRrs> {
        OTFD2EN_W::new(self, 23)
    }
}
/**RCC AHB3 Clock Register

You can [`read`](crate::Reg::read) this register and get [`c1_ahb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_ahb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#RCC:C1_AHB3ENR)*/
pub struct C1_AHB3ENRrs;
impl crate::RegisterSpec for C1_AHB3ENRrs {
    type Ux = u32;
}
///`read()` method returns [`c1_ahb3enr::R`](R) reader structure
impl crate::Readable for C1_AHB3ENRrs {}
///`write(|w| ..)` method takes [`c1_ahb3enr::W`](W) writer structure
impl crate::Writable for C1_AHB3ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C1_AHB3ENR to value 0
impl crate::Resettable for C1_AHB3ENRrs {}
