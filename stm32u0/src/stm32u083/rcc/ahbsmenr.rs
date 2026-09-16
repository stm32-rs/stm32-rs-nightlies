///Register `AHBSMENR` reader
pub type R = crate::R<AHBSMENRrs>;
///Register `AHBSMENR` writer
pub type W = crate::W<AHBSMENRrs>;
///Field `DMA1SMEN` reader - DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.
pub type DMA1SMEN_R = crate::BitReader;
///Field `DMA1SMEN` writer - DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.
pub type DMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2SMEN` reader - DMA2 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.
pub type DMA2SMEN_R = crate::BitReader;
///Field `DMA2SMEN` writer - DMA2 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.
pub type DMA2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHSMEN` reader - Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the flash memory is in power down mode.
pub type FLASHSMEN_R = crate::BitReader;
///Field `FLASHSMEN` writer - Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the flash memory is in power down mode.
pub type FLASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAMSMEN` reader - SRAM clock enable during Sleep mode Set and cleared by software.
pub type SRAMSMEN_R = crate::BitReader;
///Field `SRAMSMEN` writer - SRAM clock enable during Sleep mode Set and cleared by software.
pub type SRAMSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCSMEN` reader - CRC clock enable during Sleep mode Set and cleared by software.
pub type CRCSMEN_R = crate::BitReader;
///Field `CRCSMEN` writer - CRC clock enable during Sleep mode Set and cleared by software.
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESSMEN` reader - AES hardware accelerator clock enable during Sleep mode Set and cleared by software.
pub type AESSMEN_R = crate::BitReader;
///Field `AESSMEN` writer - AES hardware accelerator clock enable during Sleep mode Set and cleared by software.
pub type AESSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGSMEN` reader - RNG clock enable during Sleep and Stop mode Set and cleared by software.
pub type RNGSMEN_R = crate::BitReader;
///Field `RNGSMEN` writer - RNG clock enable during Sleep and Stop mode Set and cleared by software.
pub type RNGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCSMEN` reader - TSC clock enable during Sleep and Stop mode Set and cleared by software.
pub type TSCSMEN_R = crate::BitReader;
///Field `TSCSMEN` writer - TSC clock enable during Sleep and Stop mode Set and cleared by software.
pub type TSCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the flash memory is in power down mode.
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn sramsmen(&self) -> SRAMSMEN_R {
        SRAMSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - AES hardware accelerator clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn aessmen(&self) -> AESSMEN_R {
        AESSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - RNG clock enable during Sleep and Stop mode Set and cleared by software.
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - TSC clock enable during Sleep and Stop mode Set and cleared by software.
    #[inline(always)]
    pub fn tscsmen(&self) -> TSCSMEN_R {
        TSCSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBSMENR")
            .field("dma1smen", &self.dma1smen())
            .field("dma2smen", &self.dma2smen())
            .field("flashsmen", &self.flashsmen())
            .field("sramsmen", &self.sramsmen())
            .field("crcsmen", &self.crcsmen())
            .field("aessmen", &self.aessmen())
            .field("rngsmen", &self.rngsmen())
            .field("tscsmen", &self.tscsmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.
    #[inline(always)]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<'_, AHBSMENRrs> {
        DMA1SMEN_W::new(self, 0)
    }
    ///Bit 1 - DMA2 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.
    #[inline(always)]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W<'_, AHBSMENRrs> {
        DMA2SMEN_W::new(self, 1)
    }
    ///Bit 8 - Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the flash memory is in power down mode.
    #[inline(always)]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<'_, AHBSMENRrs> {
        FLASHSMEN_W::new(self, 8)
    }
    ///Bit 9 - SRAM clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn sramsmen(&mut self) -> SRAMSMEN_W<'_, AHBSMENRrs> {
        SRAMSMEN_W::new(self, 9)
    }
    ///Bit 12 - CRC clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<'_, AHBSMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
    ///Bit 16 - AES hardware accelerator clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn aessmen(&mut self) -> AESSMEN_W<'_, AHBSMENRrs> {
        AESSMEN_W::new(self, 16)
    }
    ///Bit 18 - RNG clock enable during Sleep and Stop mode Set and cleared by software.
    #[inline(always)]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<'_, AHBSMENRrs> {
        RNGSMEN_W::new(self, 18)
    }
    ///Bit 24 - TSC clock enable during Sleep and Stop mode Set and cleared by software.
    #[inline(always)]
    pub fn tscsmen(&mut self) -> TSCSMEN_W<'_, AHBSMENRrs> {
        TSCSMEN_W::new(self, 24)
    }
}
/**AHB peripheral clock enable in Sleep/Stop mode register

You can [`read`](crate::Reg::read) this register and get [`ahbsmenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsmenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:AHBSMENR)*/
pub struct AHBSMENRrs;
impl crate::RegisterSpec for AHBSMENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbsmenr::R`](R) reader structure
impl crate::Readable for AHBSMENRrs {}
///`write(|w| ..)` method takes [`ahbsmenr::W`](W) writer structure
impl crate::Writable for AHBSMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHBSMENR to value 0x0105_1303
impl crate::Resettable for AHBSMENRrs {
    const RESET_VALUE: u32 = 0x0105_1303;
}
