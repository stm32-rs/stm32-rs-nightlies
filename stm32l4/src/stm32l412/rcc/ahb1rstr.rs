///Register `AHB1RSTR` reader
pub type R = crate::R<AHB1RSTRrs>;
///Register `AHB1RSTR` writer
pub type W = crate::W<AHB1RSTRrs>;
///Field `DMA1RST` reader - DMA1 reset
pub type DMA1RST_R = crate::BitReader;
///Field `DMA1RST` writer - DMA1 reset
pub type DMA1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2RST` reader - DMA2 reset
pub type DMA2RST_R = crate::BitReader;
///Field `DMA2RST` writer - DMA2 reset
pub type DMA2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHRST` reader - Flash memory interface reset
pub type FLASHRST_R = crate::BitReader;
///Field `FLASHRST` writer - Flash memory interface reset
pub type FLASHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCRST` reader - CRC reset
pub type CRCRST_R = crate::BitReader;
///Field `CRCRST` writer - CRC reset
pub type CRCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCRST` reader - Touch Sensing Controller reset
pub type TSCRST_R = crate::BitReader;
///Field `TSCRST` writer - Touch Sensing Controller reset
pub type TSCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DMA1 reset
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 reset
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Flash memory interface reset
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC reset
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Touch Sensing Controller reset
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1RSTR")
            .field("tscrst", &self.tscrst())
            .field("crcrst", &self.crcrst())
            .field("flashrst", &self.flashrst())
            .field("dma2rst", &self.dma2rst())
            .field("dma1rst", &self.dma1rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 reset
    #[inline(always)]
    pub fn dma1rst(&mut self) -> DMA1RST_W<'_, AHB1RSTRrs> {
        DMA1RST_W::new(self, 0)
    }
    ///Bit 1 - DMA2 reset
    #[inline(always)]
    pub fn dma2rst(&mut self) -> DMA2RST_W<'_, AHB1RSTRrs> {
        DMA2RST_W::new(self, 1)
    }
    ///Bit 8 - Flash memory interface reset
    #[inline(always)]
    pub fn flashrst(&mut self) -> FLASHRST_W<'_, AHB1RSTRrs> {
        FLASHRST_W::new(self, 8)
    }
    ///Bit 12 - CRC reset
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W<'_, AHB1RSTRrs> {
        CRCRST_W::new(self, 12)
    }
    ///Bit 16 - Touch Sensing Controller reset
    #[inline(always)]
    pub fn tscrst(&mut self) -> TSCRST_W<'_, AHB1RSTRrs> {
        TSCRST_W::new(self, 16)
    }
}
/**AHB1 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#RCC:AHB1RSTR)*/
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
