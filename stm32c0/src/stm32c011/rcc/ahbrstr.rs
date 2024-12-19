///Register `AHBRSTR` reader
pub type R = crate::R<AHBRSTRrs>;
///Register `AHBRSTR` writer
pub type W = crate::W<AHBRSTRrs>;
///Field `DMA1RST` reader - DMA1 and DMAMUX reset Set and cleared by software.
pub type DMA1RST_R = crate::BitReader;
///Field `DMA1RST` writer - DMA1 and DMAMUX reset Set and cleared by software.
pub type DMA1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHRST` reader - Flash memory interface reset Set and cleared by software. This bit can only be set when the Flash memory is in power down mode.
pub type FLASHRST_R = crate::BitReader;
///Field `FLASHRST` writer - Flash memory interface reset Set and cleared by software. This bit can only be set when the Flash memory is in power down mode.
pub type FLASHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCRST` reader - CRC reset Set and cleared by software.
pub type CRCRST_R = crate::BitReader;
///Field `CRCRST` writer - CRC reset Set and cleared by software.
pub type CRCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DMA1 and DMAMUX reset Set and cleared by software.
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Flash memory interface reset Set and cleared by software. This bit can only be set when the Flash memory is in power down mode.
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
        f.debug_struct("AHBRSTR")
            .field("dma1rst", &self.dma1rst())
            .field("flashrst", &self.flashrst())
            .field("crcrst", &self.crcrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 and DMAMUX reset Set and cleared by software.
    #[inline(always)]
    pub fn dma1rst(&mut self) -> DMA1RST_W<AHBRSTRrs> {
        DMA1RST_W::new(self, 0)
    }
    ///Bit 8 - Flash memory interface reset Set and cleared by software. This bit can only be set when the Flash memory is in power down mode.
    #[inline(always)]
    pub fn flashrst(&mut self) -> FLASHRST_W<AHBRSTRrs> {
        FLASHRST_W::new(self, 8)
    }
    ///Bit 12 - CRC reset Set and cleared by software.
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W<AHBRSTRrs> {
        CRCRST_W::new(self, 12)
    }
}
/**RCC AHB peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:AHBRSTR)*/
pub struct AHBRSTRrs;
impl crate::RegisterSpec for AHBRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbrstr::R`](R) reader structure
impl crate::Readable for AHBRSTRrs {}
///`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure
impl crate::Writable for AHBRSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AHBRSTR to value 0
impl crate::Resettable for AHBRSTRrs {
    const RESET_VALUE: u32 = 0;
}
