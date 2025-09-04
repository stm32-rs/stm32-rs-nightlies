///Register `AHB1RSTR` reader
pub type R = crate::R<AHB1RSTRrs>;
///Register `AHB1RSTR` writer
pub type W = crate::W<AHB1RSTRrs>;
/**DMA1 reset Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1RST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<DMA1RST> for bool {
    #[inline(always)]
    fn from(variant: DMA1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1RST` reader - DMA1 reset Set and cleared by software.
pub type DMA1RST_R = crate::BitReader<DMA1RST>;
impl DMA1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DMA1RST> {
        match self.bits {
            true => Some(DMA1RST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMA1RST::Reset
    }
}
///Field `DMA1RST` writer - DMA1 reset Set and cleared by software.
pub type DMA1RST_W<'a, REG> = crate::BitWriter<'a, REG, DMA1RST>;
impl<'a, REG> DMA1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RST::Reset)
    }
}
///Field `DMA2RST` reader - DMA2 reset Set and cleared by software.
pub use DMA1RST_R as DMA2RST_R;
///Field `DMAMUX1RST` reader - Set and cleared by software.
pub use DMA1RST_R as DMAMUX1RST_R;
///Field `CORDICRST` reader - Set and cleared by software
pub use DMA1RST_R as CORDICRST_R;
///Field `FMACRST` reader - Set and cleared by software
pub use DMA1RST_R as FMACRST_R;
///Field `FLASHRST` reader - Flash memory interface reset Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode.
pub use DMA1RST_R as FLASHRST_R;
///Field `CRCRST` reader - CRC reset Set and cleared by software.
pub use DMA1RST_R as CRCRST_R;
///Field `DMA2RST` writer - DMA2 reset Set and cleared by software.
pub use DMA1RST_W as DMA2RST_W;
///Field `DMAMUX1RST` writer - Set and cleared by software.
pub use DMA1RST_W as DMAMUX1RST_W;
///Field `CORDICRST` writer - Set and cleared by software
pub use DMA1RST_W as CORDICRST_W;
///Field `FMACRST` writer - Set and cleared by software
pub use DMA1RST_W as FMACRST_W;
///Field `FLASHRST` writer - Flash memory interface reset Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode.
pub use DMA1RST_W as FLASHRST_W;
///Field `CRCRST` writer - CRC reset Set and cleared by software.
pub use DMA1RST_W as CRCRST_W;
impl R {
    ///Bit 0 - DMA1 reset Set and cleared by software.
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 reset Set and cleared by software.
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Set and cleared by software.
    #[inline(always)]
    pub fn dmamux1rst(&self) -> DMAMUX1RST_R {
        DMAMUX1RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Set and cleared by software
    #[inline(always)]
    pub fn cordicrst(&self) -> CORDICRST_R {
        CORDICRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Set and cleared by software
    #[inline(always)]
    pub fn fmacrst(&self) -> FMACRST_R {
        FMACRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Flash memory interface reset Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode.
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC reset Set and cleared by software.
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1RSTR")
            .field("dma1rst", &self.dma1rst())
            .field("dma2rst", &self.dma2rst())
            .field("dmamux1rst", &self.dmamux1rst())
            .field("cordicrst", &self.cordicrst())
            .field("fmacrst", &self.fmacrst())
            .field("flashrst", &self.flashrst())
            .field("crcrst", &self.crcrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 reset Set and cleared by software.
    #[inline(always)]
    pub fn dma1rst(&mut self) -> DMA1RST_W<AHB1RSTRrs> {
        DMA1RST_W::new(self, 0)
    }
    ///Bit 1 - DMA2 reset Set and cleared by software.
    #[inline(always)]
    pub fn dma2rst(&mut self) -> DMA2RST_W<AHB1RSTRrs> {
        DMA2RST_W::new(self, 1)
    }
    ///Bit 2 - Set and cleared by software.
    #[inline(always)]
    pub fn dmamux1rst(&mut self) -> DMAMUX1RST_W<AHB1RSTRrs> {
        DMAMUX1RST_W::new(self, 2)
    }
    ///Bit 3 - Set and cleared by software
    #[inline(always)]
    pub fn cordicrst(&mut self) -> CORDICRST_W<AHB1RSTRrs> {
        CORDICRST_W::new(self, 3)
    }
    ///Bit 4 - Set and cleared by software
    #[inline(always)]
    pub fn fmacrst(&mut self) -> FMACRST_W<AHB1RSTRrs> {
        FMACRST_W::new(self, 4)
    }
    ///Bit 8 - Flash memory interface reset Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode.
    #[inline(always)]
    pub fn flashrst(&mut self) -> FLASHRST_W<AHB1RSTRrs> {
        FLASHRST_W::new(self, 8)
    }
    ///Bit 12 - CRC reset Set and cleared by software.
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W<AHB1RSTRrs> {
        CRCRST_W::new(self, 12)
    }
}
/**AHB1 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G4A1.html#RCC:AHB1RSTR)*/
pub struct AHB1RSTRrs;
impl crate::RegisterSpec for AHB1RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1rstr::R`](R) reader structure
impl crate::Readable for AHB1RSTRrs {}
///`write(|w| ..)` method takes [`ahb1rstr::W`](W) writer structure
impl crate::Writable for AHB1RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1RSTR to value 0
impl crate::Resettable for AHB1RSTRrs {}
