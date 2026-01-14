///Register `AHBRSTR` reader
pub type R = crate::R<AHBRSTRrs>;
///Register `AHBRSTR` writer
pub type W = crate::W<AHBRSTRrs>;
///Field `DMA1RST` reader - DMA1 and DMAMUX reset Set and cleared by software.
pub type DMA1RST_R = crate::BitReader;
///Field `DMA1RST` writer - DMA1 and DMAMUX reset Set and cleared by software.
pub type DMA1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2RST` reader - DMA2 and DMAMUX reset Set and cleared by software.
pub type DMA2RST_R = crate::BitReader;
///Field `DMA2RST` writer - DMA2 and DMAMUX reset Set and cleared by software.
pub type DMA2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHRST` reader - Flash memory interface reset Set and cleared by software. This bit can only be set when the flash memory is in power down mode.
pub type FLASHRST_R = crate::BitReader;
///Field `FLASHRST` writer - Flash memory interface reset Set and cleared by software. This bit can only be set when the flash memory is in power down mode.
pub type FLASHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCRST` reader - CRC reset Set and cleared by software.
pub type CRCRST_R = crate::BitReader;
///Field `CRCRST` writer - CRC reset Set and cleared by software.
pub type CRCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESRST` reader - AES hardware accelerator reset Set and cleared by software.
pub type AESRST_R = crate::BitReader;
///Field `AESRST` writer - AES hardware accelerator reset Set and cleared by software.
pub type AESRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGRST` reader - Random number generator reset Set and cleared by software.
pub type RNGRST_R = crate::BitReader;
///Field `RNGRST` writer - Random number generator reset Set and cleared by software.
pub type RNGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCRST` reader - Touch sensing controller reset Set and cleared by software.
pub type TSCRST_R = crate::BitReader;
///Field `TSCRST` writer - Touch sensing controller reset Set and cleared by software.
pub type TSCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DMA1 and DMAMUX reset Set and cleared by software.
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 and DMAMUX reset Set and cleared by software.
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Flash memory interface reset Set and cleared by software. This bit can only be set when the flash memory is in power down mode.
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC reset Set and cleared by software.
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - AES hardware accelerator reset Set and cleared by software.
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Random number generator reset Set and cleared by software.
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - Touch sensing controller reset Set and cleared by software.
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRSTR")
            .field("dma1rst", &self.dma1rst())
            .field("dma2rst", &self.dma2rst())
            .field("flashrst", &self.flashrst())
            .field("crcrst", &self.crcrst())
            .field("aesrst", &self.aesrst())
            .field("rngrst", &self.rngrst())
            .field("tscrst", &self.tscrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 and DMAMUX reset Set and cleared by software.
    #[inline(always)]
    pub fn dma1rst(&mut self) -> DMA1RST_W<'_, AHBRSTRrs> {
        DMA1RST_W::new(self, 0)
    }
    ///Bit 1 - DMA2 and DMAMUX reset Set and cleared by software.
    #[inline(always)]
    pub fn dma2rst(&mut self) -> DMA2RST_W<'_, AHBRSTRrs> {
        DMA2RST_W::new(self, 1)
    }
    ///Bit 8 - Flash memory interface reset Set and cleared by software. This bit can only be set when the flash memory is in power down mode.
    #[inline(always)]
    pub fn flashrst(&mut self) -> FLASHRST_W<'_, AHBRSTRrs> {
        FLASHRST_W::new(self, 8)
    }
    ///Bit 12 - CRC reset Set and cleared by software.
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W<'_, AHBRSTRrs> {
        CRCRST_W::new(self, 12)
    }
    ///Bit 16 - AES hardware accelerator reset Set and cleared by software.
    #[inline(always)]
    pub fn aesrst(&mut self) -> AESRST_W<'_, AHBRSTRrs> {
        AESRST_W::new(self, 16)
    }
    ///Bit 18 - Random number generator reset Set and cleared by software.
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W<'_, AHBRSTRrs> {
        RNGRST_W::new(self, 18)
    }
    ///Bit 24 - Touch sensing controller reset Set and cleared by software.
    #[inline(always)]
    pub fn tscrst(&mut self) -> TSCRST_W<'_, AHBRSTRrs> {
        TSCRST_W::new(self, 24)
    }
}
/**AHB peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:AHBRSTR)*/
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
