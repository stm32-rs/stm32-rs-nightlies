///Register `AHB5RSTR` reader
pub type R = crate::R<AHB5RSTRrs>;
///Register `AHB5RSTR` writer
pub type W = crate::W<AHB5RSTRrs>;
/**HPDMA1 block reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPDMA1RST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<HPDMA1RST> for bool {
    #[inline(always)]
    fn from(variant: HPDMA1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `HPDMA1RST` reader - HPDMA1 block reset
pub type HPDMA1RST_R = crate::BitReader<HPDMA1RST>;
impl HPDMA1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<HPDMA1RST> {
        match self.bits {
            true => Some(HPDMA1RST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == HPDMA1RST::Reset
    }
}
///Field `HPDMA1RST` writer - HPDMA1 block reset
pub type HPDMA1RST_W<'a, REG> = crate::BitWriter<'a, REG, HPDMA1RST>;
impl<'a, REG> HPDMA1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(HPDMA1RST::Reset)
    }
}
///Field `DMA2DRST` reader - DMA2D block reset
pub use HPDMA1RST_R as DMA2DRST_R;
///Field `JPEGRST` reader - JPEG block reset
pub use HPDMA1RST_R as JPEGRST_R;
///Field `FMCRST` reader - FMC and MCE3 blocks reset
pub use HPDMA1RST_R as FMCRST_R;
///Field `XSPI1RST` reader - XSPI1 and MCE1 blocks reset
pub use HPDMA1RST_R as XSPI1RST_R;
///Field `SDMMC1RST` reader - SDMMC1 and DB_SDMMC1 blocks reset
pub use HPDMA1RST_R as SDMMC1RST_R;
///Field `XSPI2RST` reader - XSPI2 and MCE2 blocks reset
pub use HPDMA1RST_R as XSPI2RST_R;
///Field `XSPIMRST` reader - XSPIM reset
pub use HPDMA1RST_R as XSPIMRST_R;
///Field `GFXMMURST` reader - GFXMMU block reset
pub use HPDMA1RST_R as GFXMMURST_R;
///Field `GPU2DRST` reader - GPU2D block reset
pub use HPDMA1RST_R as GPU2DRST_R;
///Field `DMA2DRST` writer - DMA2D block reset
pub use HPDMA1RST_W as DMA2DRST_W;
///Field `JPEGRST` writer - JPEG block reset
pub use HPDMA1RST_W as JPEGRST_W;
///Field `FMCRST` writer - FMC and MCE3 blocks reset
pub use HPDMA1RST_W as FMCRST_W;
///Field `XSPI1RST` writer - XSPI1 and MCE1 blocks reset
pub use HPDMA1RST_W as XSPI1RST_W;
///Field `SDMMC1RST` writer - SDMMC1 and DB_SDMMC1 blocks reset
pub use HPDMA1RST_W as SDMMC1RST_W;
///Field `XSPI2RST` writer - XSPI2 and MCE2 blocks reset
pub use HPDMA1RST_W as XSPI2RST_W;
///Field `XSPIMRST` writer - XSPIM reset
pub use HPDMA1RST_W as XSPIMRST_W;
///Field `GFXMMURST` writer - GFXMMU block reset
pub use HPDMA1RST_W as GFXMMURST_W;
///Field `GPU2DRST` writer - GPU2D block reset
pub use HPDMA1RST_W as GPU2DRST_W;
impl R {
    ///Bit 0 - HPDMA1 block reset
    #[inline(always)]
    pub fn hpdma1rst(&self) -> HPDMA1RST_R {
        HPDMA1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2D block reset
    #[inline(always)]
    pub fn dma2drst(&self) -> DMA2DRST_R {
        DMA2DRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - JPEG block reset
    #[inline(always)]
    pub fn jpegrst(&self) -> JPEGRST_R {
        JPEGRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FMC and MCE3 blocks reset
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - XSPI1 and MCE1 blocks reset
    #[inline(always)]
    pub fn xspi1rst(&self) -> XSPI1RST_R {
        XSPI1RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - SDMMC1 and DB_SDMMC1 blocks reset
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - XSPI2 and MCE2 blocks reset
    #[inline(always)]
    pub fn xspi2rst(&self) -> XSPI2RST_R {
        XSPI2RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - XSPIM reset
    #[inline(always)]
    pub fn xspimrst(&self) -> XSPIMRST_R {
        XSPIMRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 19 - GFXMMU block reset
    #[inline(always)]
    pub fn gfxmmurst(&self) -> GFXMMURST_R {
        GFXMMURST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - GPU2D block reset
    #[inline(always)]
    pub fn gpu2drst(&self) -> GPU2DRST_R {
        GPU2DRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB5RSTR")
            .field("hpdma1rst", &self.hpdma1rst())
            .field("dma2drst", &self.dma2drst())
            .field("jpegrst", &self.jpegrst())
            .field("fmcrst", &self.fmcrst())
            .field("xspi1rst", &self.xspi1rst())
            .field("sdmmc1rst", &self.sdmmc1rst())
            .field("xspi2rst", &self.xspi2rst())
            .field("xspimrst", &self.xspimrst())
            .field("gfxmmurst", &self.gfxmmurst())
            .field("gpu2drst", &self.gpu2drst())
            .finish()
    }
}
impl W {
    ///Bit 0 - HPDMA1 block reset
    #[inline(always)]
    pub fn hpdma1rst(&mut self) -> HPDMA1RST_W<'_, AHB5RSTRrs> {
        HPDMA1RST_W::new(self, 0)
    }
    ///Bit 1 - DMA2D block reset
    #[inline(always)]
    pub fn dma2drst(&mut self) -> DMA2DRST_W<'_, AHB5RSTRrs> {
        DMA2DRST_W::new(self, 1)
    }
    ///Bit 3 - JPEG block reset
    #[inline(always)]
    pub fn jpegrst(&mut self) -> JPEGRST_W<'_, AHB5RSTRrs> {
        JPEGRST_W::new(self, 3)
    }
    ///Bit 4 - FMC and MCE3 blocks reset
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FMCRST_W<'_, AHB5RSTRrs> {
        FMCRST_W::new(self, 4)
    }
    ///Bit 5 - XSPI1 and MCE1 blocks reset
    #[inline(always)]
    pub fn xspi1rst(&mut self) -> XSPI1RST_W<'_, AHB5RSTRrs> {
        XSPI1RST_W::new(self, 5)
    }
    ///Bit 8 - SDMMC1 and DB_SDMMC1 blocks reset
    #[inline(always)]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<'_, AHB5RSTRrs> {
        SDMMC1RST_W::new(self, 8)
    }
    ///Bit 12 - XSPI2 and MCE2 blocks reset
    #[inline(always)]
    pub fn xspi2rst(&mut self) -> XSPI2RST_W<'_, AHB5RSTRrs> {
        XSPI2RST_W::new(self, 12)
    }
    ///Bit 14 - XSPIM reset
    #[inline(always)]
    pub fn xspimrst(&mut self) -> XSPIMRST_W<'_, AHB5RSTRrs> {
        XSPIMRST_W::new(self, 14)
    }
    ///Bit 19 - GFXMMU block reset
    #[inline(always)]
    pub fn gfxmmurst(&mut self) -> GFXMMURST_W<'_, AHB5RSTRrs> {
        GFXMMURST_W::new(self, 19)
    }
    ///Bit 20 - GPU2D block reset
    #[inline(always)]
    pub fn gpu2drst(&mut self) -> GPU2DRST_W<'_, AHB5RSTRrs> {
        GPU2DRST_W::new(self, 20)
    }
}
/**RCC AHB5 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb5rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB5RSTR)*/
pub struct AHB5RSTRrs;
impl crate::RegisterSpec for AHB5RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb5rstr::R`](R) reader structure
impl crate::Readable for AHB5RSTRrs {}
///`write(|w| ..)` method takes [`ahb5rstr::W`](W) writer structure
impl crate::Writable for AHB5RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB5RSTR to value 0
impl crate::Resettable for AHB5RSTRrs {}
