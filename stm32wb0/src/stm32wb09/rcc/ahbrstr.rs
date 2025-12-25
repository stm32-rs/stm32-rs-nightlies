///Register `AHBRSTR` reader
pub type R = crate::R<AHBRSTRrs>;
///Register `AHBRSTR` writer
pub type W = crate::W<AHBRSTRrs>;
///Field `DMARST` reader - DMA and DMAMUX reset Set and reset by software.
pub type DMARST_R = crate::BitReader;
///Field `DMARST` writer - DMA and DMAMUX reset Set and reset by software.
pub type DMARST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOARST` reader - GPIOA reset Set and reset by software.
pub type GPIOARST_R = crate::BitReader;
///Field `GPIOARST` writer - GPIOA reset Set and reset by software.
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBRST` reader - GPIOB reset Set and reset by software.
pub type GPIOBRST_R = crate::BitReader;
///Field `GPIOBRST` writer - GPIOB reset Set and reset by software.
pub type GPIOBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCRST` reader - CRC reset Set and reset by software.
pub type CRCRST_R = crate::BitReader;
///Field `CRCRST` writer - CRC reset Set and reset by software.
pub type CRCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKARST` reader - PKA reset Set and reset by software.
pub type PKARST_R = crate::BitReader;
///Field `PKARST` writer - PKA reset Set and reset by software.
pub type PKARST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGRST` reader - RNG reset Set and reset by software.
pub type RNGRST_R = crate::BitReader;
///Field `RNGRST` writer - RNG reset Set and reset by software.
pub type RNGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DMA and DMAMUX reset Set and reset by software.
    #[inline(always)]
    pub fn dmarst(&self) -> DMARST_R {
        DMARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - GPIOA reset Set and reset by software.
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIOB reset Set and reset by software.
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 12 - CRC reset Set and reset by software.
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - PKA reset Set and reset by software.
    #[inline(always)]
    pub fn pkarst(&self) -> PKARST_R {
        PKARST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - RNG reset Set and reset by software.
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRSTR")
            .field("dmarst", &self.dmarst())
            .field("gpioarst", &self.gpioarst())
            .field("gpiobrst", &self.gpiobrst())
            .field("crcrst", &self.crcrst())
            .field("pkarst", &self.pkarst())
            .field("rngrst", &self.rngrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA and DMAMUX reset Set and reset by software.
    #[inline(always)]
    pub fn dmarst(&mut self) -> DMARST_W<'_, AHBRSTRrs> {
        DMARST_W::new(self, 0)
    }
    ///Bit 2 - GPIOA reset Set and reset by software.
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W<'_, AHBRSTRrs> {
        GPIOARST_W::new(self, 2)
    }
    ///Bit 3 - GPIOB reset Set and reset by software.
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<'_, AHBRSTRrs> {
        GPIOBRST_W::new(self, 3)
    }
    ///Bit 12 - CRC reset Set and reset by software.
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W<'_, AHBRSTRrs> {
        CRCRST_W::new(self, 12)
    }
    ///Bit 16 - PKA reset Set and reset by software.
    #[inline(always)]
    pub fn pkarst(&mut self) -> PKARST_W<'_, AHBRSTRrs> {
        PKARST_W::new(self, 16)
    }
    ///Bit 18 - RNG reset Set and reset by software.
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W<'_, AHBRSTRrs> {
        RNGRST_W::new(self, 18)
    }
}
/**AHBRSTR register

You can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RCC:AHBRSTR)*/
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
