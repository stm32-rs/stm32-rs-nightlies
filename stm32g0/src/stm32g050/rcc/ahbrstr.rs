///Register `AHBRSTR` reader
pub type R = crate::R<AHBRSTRrs>;
///Register `AHBRSTR` writer
pub type W = crate::W<AHBRSTRrs>;
/**DMA1 reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1RST {
    ///1: Reset peripheral
    Reset = 1,
}
impl From<DMA1RST> for bool {
    #[inline(always)]
    fn from(variant: DMA1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1RST` reader - DMA1 reset
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
    ///Reset peripheral
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMA1RST::Reset
    }
}
///Field `DMA1RST` writer - DMA1 reset
pub type DMA1RST_W<'a, REG> = crate::BitWriter<'a, REG, DMA1RST>;
impl<'a, REG> DMA1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RST::Reset)
    }
}
///Field `FLASHRST` reader - FLITF reset
pub use DMA1RST_R as FLASHRST_R;
///Field `CRCRST` reader - CRC reset
pub use DMA1RST_R as CRCRST_R;
///Field `FLASHRST` writer - FLITF reset
pub use DMA1RST_W as FLASHRST_W;
///Field `CRCRST` writer - CRC reset
pub use DMA1RST_W as CRCRST_W;
impl R {
    ///Bit 0 - DMA1 reset
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - FLITF reset
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC reset
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRSTR")
            .field("dma1rst", &self.dma1rst())
            .field("flashrst", &self.flashrst())
            .field("crcrst", &self.crcrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 reset
    #[inline(always)]
    pub fn dma1rst(&mut self) -> DMA1RST_W<'_, AHBRSTRrs> {
        DMA1RST_W::new(self, 0)
    }
    ///Bit 8 - FLITF reset
    #[inline(always)]
    pub fn flashrst(&mut self) -> FLASHRST_W<'_, AHBRSTRrs> {
        FLASHRST_W::new(self, 8)
    }
    ///Bit 12 - CRC reset
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W<'_, AHBRSTRrs> {
        CRCRST_W::new(self, 12)
    }
}
/**AHB peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G050.html#RCC:AHBRSTR)*/
pub struct AHBRSTRrs;
impl crate::RegisterSpec for AHBRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbrstr::R`](R) reader structure
impl crate::Readable for AHBRSTRrs {}
///`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure
impl crate::Writable for AHBRSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHBRSTR to value 0
impl crate::Resettable for AHBRSTRrs {}
