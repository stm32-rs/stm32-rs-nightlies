///Register `AHB5ENR` reader
pub type R = crate::R<AHB5ENRrs>;
///Register `AHB5ENR` writer
pub type W = crate::W<AHB5ENRrs>;
/**HPDMA1 peripheral clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPDMA1EN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<HPDMA1EN> for bool {
    #[inline(always)]
    fn from(variant: HPDMA1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `HPDMA1EN` reader - HPDMA1 peripheral clock enable
pub type HPDMA1EN_R = crate::BitReader<HPDMA1EN>;
impl HPDMA1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HPDMA1EN {
        match self.bits {
            false => HPDMA1EN::Disabled,
            true => HPDMA1EN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HPDMA1EN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HPDMA1EN::Enabled
    }
}
///Field `HPDMA1EN` writer - HPDMA1 peripheral clock enable
pub type HPDMA1EN_W<'a, REG> = crate::BitWriter<'a, REG, HPDMA1EN>;
impl<'a, REG> HPDMA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HPDMA1EN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HPDMA1EN::Enabled)
    }
}
///Field `DMA2DEN` reader - DMA2D peripheral clock enable
pub use HPDMA1EN_R as DMA2DEN_R;
///Field `JPEGEN` reader - JPEG peripheral clock enable
pub use HPDMA1EN_R as JPEGEN_R;
///Field `FMCEN` reader - FMC and MCE3 peripheral clocks enable
pub use HPDMA1EN_R as FMCEN_R;
///Field `XSPI1EN` reader - XSPI1 and MCE1 peripheral clocks enable
pub use HPDMA1EN_R as XSPI1EN_R;
///Field `SDMMC1EN` reader - SDMMC1 and DB_SDMMC1 peripheral clocks enable
pub use HPDMA1EN_R as SDMMC1EN_R;
///Field `XSPI2EN` reader - XSPI2 and MCE2 peripheral clocks enable
pub use HPDMA1EN_R as XSPI2EN_R;
///Field `XSPIMEN` reader - XSPIM peripheral clock enable
pub use HPDMA1EN_R as XSPIMEN_R;
///Field `GFXMMUEN` reader - GFXMMU peripheral clock enable
pub use HPDMA1EN_R as GFXMMUEN_R;
///Field `GPU2DEN` reader - GPU2D peripheral clock enable
pub use HPDMA1EN_R as GPU2DEN_R;
///Field `DMA2DEN` writer - DMA2D peripheral clock enable
pub use HPDMA1EN_W as DMA2DEN_W;
///Field `JPEGEN` writer - JPEG peripheral clock enable
pub use HPDMA1EN_W as JPEGEN_W;
///Field `FMCEN` writer - FMC and MCE3 peripheral clocks enable
pub use HPDMA1EN_W as FMCEN_W;
///Field `XSPI1EN` writer - XSPI1 and MCE1 peripheral clocks enable
pub use HPDMA1EN_W as XSPI1EN_W;
///Field `SDMMC1EN` writer - SDMMC1 and DB_SDMMC1 peripheral clocks enable
pub use HPDMA1EN_W as SDMMC1EN_W;
///Field `XSPI2EN` writer - XSPI2 and MCE2 peripheral clocks enable
pub use HPDMA1EN_W as XSPI2EN_W;
///Field `XSPIMEN` writer - XSPIM peripheral clock enable
pub use HPDMA1EN_W as XSPIMEN_W;
///Field `GFXMMUEN` writer - GFXMMU peripheral clock enable
pub use HPDMA1EN_W as GFXMMUEN_W;
///Field `GPU2DEN` writer - GPU2D peripheral clock enable
pub use HPDMA1EN_W as GPU2DEN_W;
impl R {
    ///Bit 0 - HPDMA1 peripheral clock enable
    #[inline(always)]
    pub fn hpdma1en(&self) -> HPDMA1EN_R {
        HPDMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2D peripheral clock enable
    #[inline(always)]
    pub fn dma2den(&self) -> DMA2DEN_R {
        DMA2DEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - JPEG peripheral clock enable
    #[inline(always)]
    pub fn jpegen(&self) -> JPEGEN_R {
        JPEGEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FMC and MCE3 peripheral clocks enable
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - XSPI1 and MCE1 peripheral clocks enable
    #[inline(always)]
    pub fn xspi1en(&self) -> XSPI1EN_R {
        XSPI1EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - SDMMC1 and DB_SDMMC1 peripheral clocks enable
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - XSPI2 and MCE2 peripheral clocks enable
    #[inline(always)]
    pub fn xspi2en(&self) -> XSPI2EN_R {
        XSPI2EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - XSPIM peripheral clock enable
    #[inline(always)]
    pub fn xspimen(&self) -> XSPIMEN_R {
        XSPIMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 19 - GFXMMU peripheral clock enable
    #[inline(always)]
    pub fn gfxmmuen(&self) -> GFXMMUEN_R {
        GFXMMUEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - GPU2D peripheral clock enable
    #[inline(always)]
    pub fn gpu2den(&self) -> GPU2DEN_R {
        GPU2DEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB5ENR")
            .field("hpdma1en", &self.hpdma1en())
            .field("dma2den", &self.dma2den())
            .field("jpegen", &self.jpegen())
            .field("fmcen", &self.fmcen())
            .field("xspi1en", &self.xspi1en())
            .field("sdmmc1en", &self.sdmmc1en())
            .field("xspi2en", &self.xspi2en())
            .field("xspimen", &self.xspimen())
            .field("gfxmmuen", &self.gfxmmuen())
            .field("gpu2den", &self.gpu2den())
            .finish()
    }
}
impl W {
    ///Bit 0 - HPDMA1 peripheral clock enable
    #[inline(always)]
    pub fn hpdma1en(&mut self) -> HPDMA1EN_W<'_, AHB5ENRrs> {
        HPDMA1EN_W::new(self, 0)
    }
    ///Bit 1 - DMA2D peripheral clock enable
    #[inline(always)]
    pub fn dma2den(&mut self) -> DMA2DEN_W<'_, AHB5ENRrs> {
        DMA2DEN_W::new(self, 1)
    }
    ///Bit 3 - JPEG peripheral clock enable
    #[inline(always)]
    pub fn jpegen(&mut self) -> JPEGEN_W<'_, AHB5ENRrs> {
        JPEGEN_W::new(self, 3)
    }
    ///Bit 4 - FMC and MCE3 peripheral clocks enable
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W<'_, AHB5ENRrs> {
        FMCEN_W::new(self, 4)
    }
    ///Bit 5 - XSPI1 and MCE1 peripheral clocks enable
    #[inline(always)]
    pub fn xspi1en(&mut self) -> XSPI1EN_W<'_, AHB5ENRrs> {
        XSPI1EN_W::new(self, 5)
    }
    ///Bit 8 - SDMMC1 and DB_SDMMC1 peripheral clocks enable
    #[inline(always)]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<'_, AHB5ENRrs> {
        SDMMC1EN_W::new(self, 8)
    }
    ///Bit 12 - XSPI2 and MCE2 peripheral clocks enable
    #[inline(always)]
    pub fn xspi2en(&mut self) -> XSPI2EN_W<'_, AHB5ENRrs> {
        XSPI2EN_W::new(self, 12)
    }
    ///Bit 14 - XSPIM peripheral clock enable
    #[inline(always)]
    pub fn xspimen(&mut self) -> XSPIMEN_W<'_, AHB5ENRrs> {
        XSPIMEN_W::new(self, 14)
    }
    ///Bit 19 - GFXMMU peripheral clock enable
    #[inline(always)]
    pub fn gfxmmuen(&mut self) -> GFXMMUEN_W<'_, AHB5ENRrs> {
        GFXMMUEN_W::new(self, 19)
    }
    ///Bit 20 - GPU2D peripheral clock enable
    #[inline(always)]
    pub fn gpu2den(&mut self) -> GPU2DEN_W<'_, AHB5ENRrs> {
        GPU2DEN_W::new(self, 20)
    }
}
/**RCC AHB5 clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb5enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB5ENR)*/
pub struct AHB5ENRrs;
impl crate::RegisterSpec for AHB5ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb5enr::R`](R) reader structure
impl crate::Readable for AHB5ENRrs {}
///`write(|w| ..)` method takes [`ahb5enr::W`](W) writer structure
impl crate::Writable for AHB5ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB5ENR to value 0
impl crate::Resettable for AHB5ENRrs {}
