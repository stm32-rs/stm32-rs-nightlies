///Register `AHBENR` reader
pub type R = crate::R<AHBENRrs>;
///Register `AHBENR` writer
pub type W = crate::W<AHBENRrs>;
///Field `DMAEN` reader - DMA and DMAMUX enable Set and enable by software.
pub type DMAEN_R = crate::BitReader;
///Field `DMAEN` writer - DMA and DMAMUX enable Set and enable by software.
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOAEN` reader - GPIOA enable. It must be enabled by default
pub type GPIOAEN_R = crate::BitReader;
///Field `GPIOAEN` writer - GPIOA enable. It must be enabled by default
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBEN` reader - GPIOB enable. It must be enabled by default
pub type GPIOBEN_R = crate::BitReader;
///Field `GPIOBEN` writer - GPIOB enable. It must be enabled by default
pub type GPIOBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEN` reader - CRC enable Set and enable by software.
pub type CRCEN_R = crate::BitReader;
///Field `CRCEN` writer - CRC enable Set and enable by software.
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKAEN` reader - PKA clock enable Set and enable by software.
pub type PKAEN_R = crate::BitReader;
///Field `PKAEN` writer - PKA clock enable Set and enable by software.
pub type PKAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGEN` reader - RNG clock enable Set and enable by software.
pub type RNGEN_R = crate::BitReader;
///Field `RNGEN` writer - RNG clock enable Set and enable by software.
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DMA and DMAMUX enable Set and enable by software.
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - GPIOA enable. It must be enabled by default
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIOB enable. It must be enabled by default
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 12 - CRC enable Set and enable by software.
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - PKA clock enable Set and enable by software.
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - RNG clock enable Set and enable by software.
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBENR")
            .field("dmaen", &self.dmaen())
            .field("gpioaen", &self.gpioaen())
            .field("gpioben", &self.gpioben())
            .field("crcen", &self.crcen())
            .field("pkaen", &self.pkaen())
            .field("rngen", &self.rngen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA and DMAMUX enable Set and enable by software.
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, AHBENRrs> {
        DMAEN_W::new(self, 0)
    }
    ///Bit 2 - GPIOA enable. It must be enabled by default
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<'_, AHBENRrs> {
        GPIOAEN_W::new(self, 2)
    }
    ///Bit 3 - GPIOB enable. It must be enabled by default
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<'_, AHBENRrs> {
        GPIOBEN_W::new(self, 3)
    }
    ///Bit 12 - CRC enable Set and enable by software.
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, AHBENRrs> {
        CRCEN_W::new(self, 12)
    }
    ///Bit 16 - PKA clock enable Set and enable by software.
    #[inline(always)]
    pub fn pkaen(&mut self) -> PKAEN_W<'_, AHBENRrs> {
        PKAEN_W::new(self, 16)
    }
    ///Bit 18 - RNG clock enable Set and enable by software.
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<'_, AHBENRrs> {
        RNGEN_W::new(self, 18)
    }
}
/**AHBENR register

You can [`read`](crate::Reg::read) this register and get [`ahbenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RCC:AHBENR)*/
pub struct AHBENRrs;
impl crate::RegisterSpec for AHBENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbenr::R`](R) reader structure
impl crate::Readable for AHBENRrs {}
///`write(|w| ..)` method takes [`ahbenr::W`](W) writer structure
impl crate::Writable for AHBENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHBENR to value 0x0c
impl crate::Resettable for AHBENRrs {
    const RESET_VALUE: u32 = 0x0c;
}
