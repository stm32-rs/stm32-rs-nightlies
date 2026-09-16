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
///Field `JPGDECEN` reader - JPGDEC Peripheral Clock Enable
pub use MDMAEN_R as JPGDECEN_R;
///Field `FMCEN` reader - FMC Peripheral Clocks Enable
pub use MDMAEN_R as FMCEN_R;
///Field `QSPIEN` reader - QUADSPI and QUADSPI Delay Clock Enable
pub use MDMAEN_R as QSPIEN_R;
///Field `SDMMC1EN` reader - SDMMC1 and SDMMC1 Delay Clock Enable
pub use MDMAEN_R as SDMMC1EN_R;
///Field `DTCM1EN` reader - D1 DTCM1 block enable
pub use MDMAEN_R as DTCM1EN_R;
///Field `DTCM2EN` reader - D1 DTCM2 block enable
pub use MDMAEN_R as DTCM2EN_R;
///Field `ITCM1EN` reader - D1 DTCM2 block enable
pub use MDMAEN_R as ITCM1EN_R;
///Field `AXISRAMEN` reader - AXISRAM block enable
pub use MDMAEN_R as AXISRAMEN_R;
///Field `DMA2DEN` writer - DMA2D Peripheral Clock Enable
pub use MDMAEN_W as DMA2DEN_W;
///Field `JPGDECEN` writer - JPGDEC Peripheral Clock Enable
pub use MDMAEN_W as JPGDECEN_W;
///Field `FMCEN` writer - FMC Peripheral Clocks Enable
pub use MDMAEN_W as FMCEN_W;
///Field `QSPIEN` writer - QUADSPI and QUADSPI Delay Clock Enable
pub use MDMAEN_W as QSPIEN_W;
///Field `SDMMC1EN` writer - SDMMC1 and SDMMC1 Delay Clock Enable
pub use MDMAEN_W as SDMMC1EN_W;
///Field `DTCM1EN` writer - D1 DTCM1 block enable
pub use MDMAEN_W as DTCM1EN_W;
///Field `DTCM2EN` writer - D1 DTCM2 block enable
pub use MDMAEN_W as DTCM2EN_W;
///Field `ITCM1EN` writer - D1 DTCM2 block enable
pub use MDMAEN_W as ITCM1EN_W;
///Field `AXISRAMEN` writer - AXISRAM block enable
pub use MDMAEN_W as AXISRAMEN_W;
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
    ///Bit 5 - JPGDEC Peripheral Clock Enable
    #[inline(always)]
    pub fn jpgdecen(&self) -> JPGDECEN_R {
        JPGDECEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 12 - FMC Peripheral Clocks Enable
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - QUADSPI and QUADSPI Delay Clock Enable
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 28 - D1 DTCM1 block enable
    #[inline(always)]
    pub fn dtcm1en(&self) -> DTCM1EN_R {
        DTCM1EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - D1 DTCM2 block enable
    #[inline(always)]
    pub fn dtcm2en(&self) -> DTCM2EN_R {
        DTCM2EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - D1 DTCM2 block enable
    #[inline(always)]
    pub fn itcm1en(&self) -> ITCM1EN_R {
        ITCM1EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - AXISRAM block enable
    #[inline(always)]
    pub fn axisramen(&self) -> AXISRAMEN_R {
        AXISRAMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1_AHB3ENR")
            .field("mdmaen", &self.mdmaen())
            .field("dma2den", &self.dma2den())
            .field("jpgdecen", &self.jpgdecen())
            .field("fmcen", &self.fmcen())
            .field("qspien", &self.qspien())
            .field("sdmmc1en", &self.sdmmc1en())
            .field("dtcm1en", &self.dtcm1en())
            .field("dtcm2en", &self.dtcm2en())
            .field("itcm1en", &self.itcm1en())
            .field("axisramen", &self.axisramen())
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
    ///Bit 5 - JPGDEC Peripheral Clock Enable
    #[inline(always)]
    pub fn jpgdecen(&mut self) -> JPGDECEN_W<'_, C1_AHB3ENRrs> {
        JPGDECEN_W::new(self, 5)
    }
    ///Bit 12 - FMC Peripheral Clocks Enable
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W<'_, C1_AHB3ENRrs> {
        FMCEN_W::new(self, 12)
    }
    ///Bit 14 - QUADSPI and QUADSPI Delay Clock Enable
    #[inline(always)]
    pub fn qspien(&mut self) -> QSPIEN_W<'_, C1_AHB3ENRrs> {
        QSPIEN_W::new(self, 14)
    }
    ///Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable
    #[inline(always)]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<'_, C1_AHB3ENRrs> {
        SDMMC1EN_W::new(self, 16)
    }
    ///Bit 28 - D1 DTCM1 block enable
    #[inline(always)]
    pub fn dtcm1en(&mut self) -> DTCM1EN_W<'_, C1_AHB3ENRrs> {
        DTCM1EN_W::new(self, 28)
    }
    ///Bit 29 - D1 DTCM2 block enable
    #[inline(always)]
    pub fn dtcm2en(&mut self) -> DTCM2EN_W<'_, C1_AHB3ENRrs> {
        DTCM2EN_W::new(self, 29)
    }
    ///Bit 30 - D1 DTCM2 block enable
    #[inline(always)]
    pub fn itcm1en(&mut self) -> ITCM1EN_W<'_, C1_AHB3ENRrs> {
        ITCM1EN_W::new(self, 30)
    }
    ///Bit 31 - AXISRAM block enable
    #[inline(always)]
    pub fn axisramen(&mut self) -> AXISRAMEN_W<'_, C1_AHB3ENRrs> {
        AXISRAMEN_W::new(self, 31)
    }
}
/**RCC AHB3 Clock Register

You can [`read`](crate::Reg::read) this register and get [`c1_ahb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_ahb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#RCC:C1_AHB3ENR)*/
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
