///Register `AHBSMENR` reader
pub type R = crate::R<AHBSMENRrs>;
///Register `AHBSMENR` writer
pub type W = crate::W<AHBSMENRrs>;
///Field `DMASMEN` reader - DMA clock enable during Sleep mode bit This bit is set and reset by software. - 0: DMA clock disabled in Sleep mode - 1: DMA clock enabled in Sleep mode (if enabled in DMAEN)
pub type DMASMEN_R = crate::BitReader;
///Field `DMASMEN` writer - DMA clock enable during Sleep mode bit This bit is set and reset by software. - 0: DMA clock disabled in Sleep mode - 1: DMA clock enabled in Sleep mode (if enabled in DMAEN)
pub type DMASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHSMEN` reader - Flash clocks enable during Flash Sleep PD and CPU Sleep mode bit This bit is set and reset by software. - 0: Flash clocks are disabled in Flash Sleep PD* and CPU Sleep mode - 1: Flash clocks are enabled in Sleep mode Note: Flash Sleep PD is enabled through nvm_control register CONFIG.SLEEP_PD
pub type FLASHSMEN_R = crate::BitReader;
///Field `FLASHSMEN` writer - Flash clocks enable during Flash Sleep PD and CPU Sleep mode bit This bit is set and reset by software. - 0: Flash clocks are disabled in Flash Sleep PD* and CPU Sleep mode - 1: Flash clocks are enabled in Sleep mode Note: Flash Sleep PD is enabled through nvm_control register CONFIG.SLEEP_PD
pub type FLASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOASMEN` reader - GPIOA clock enable during Sleep mode bit This bit is set and reset by software. - 0: GPIOA clock disabled in Sleep mode - 1: GPIOA clock enabled in Sleep mode (if enabled by GPIOAEN)
pub type GPIOASMEN_R = crate::BitReader;
///Field `GPIOASMEN` writer - GPIOA clock enable during Sleep mode bit This bit is set and reset by software. - 0: GPIOA clock disabled in Sleep mode - 1: GPIOA clock enabled in Sleep mode (if enabled by GPIOAEN)
pub type GPIOASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBSMEN` reader - GPIOB clock enable during Sleep mode bit This bit is set and reset by software. - 0: GPIOB clock disabled in Sleep mode - 1: GPIOB clock enabled in Sleep mode (if enabled in GPIOBEN)
pub type GPIOBSMEN_R = crate::BitReader;
///Field `GPIOBSMEN` writer - GPIOB clock enable during Sleep mode bit This bit is set and reset by software. - 0: GPIOB clock disabled in Sleep mode - 1: GPIOB clock enabled in Sleep mode (if enabled in GPIOBEN)
pub type GPIOBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM0SMEN` reader - SRAM0 clock enable during Sleep mode bit This bit is set and reset by software. - 0: SRAM0 clock disabled in Sleep mode - 1: SRAM0 clock enabled in Sleep mode
pub type SRAM0SMEN_R = crate::BitReader;
///Field `SRAM0SMEN` writer - SRAM0 clock enable during Sleep mode bit This bit is set and reset by software. - 0: SRAM0 clock disabled in Sleep mode - 1: SRAM0 clock enabled in Sleep mode
pub type SRAM0SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1SMEN` reader - SRAM1 clock enable during Sleep mode bit This bit is set and reset by software. - 0: SRAM1 clock disabled in Sleep mode - 1: SRAM1 clock enabled in Sleep mode
pub type SRAM1SMEN_R = crate::BitReader;
///Field `SRAM1SMEN` writer - SRAM1 clock enable during Sleep mode bit This bit is set and reset by software. - 0: SRAM1 clock disabled in Sleep mode - 1: SRAM1 clock enabled in Sleep mode
pub type SRAM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCSMEN` reader - CRC clock enable during Sleep mode bit This bit is set and reset by software. - 0: CRC clock disabled in Sleep mode - 1: CRC clock enabled in Sleep mode (if enabled in CRCEN)
pub type CRCSMEN_R = crate::BitReader;
///Field `CRCSMEN` writer - CRC clock enable during Sleep mode bit This bit is set and reset by software. - 0: CRC clock disabled in Sleep mode - 1: CRC clock enabled in Sleep mode (if enabled in CRCEN)
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGSMEN` reader - RNG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: RNG bus clock disabled in Sleep mode - 1: RNG bus clock enabled in Sleep mode (if enabled in RNGEN)
pub type RNGSMEN_R = crate::BitReader;
///Field `RNGSMEN` writer - RNG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: RNG bus clock disabled in Sleep mode - 1: RNG bus clock enabled in Sleep mode (if enabled in RNGEN)
pub type RNGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESSMEN` reader - AES bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: AES bus clock disabled in Sleep mode - 1: AES bus clock enabled in Sleep mode (if enabled in AESEN)
pub type AESSMEN_R = crate::BitReader;
///Field `AESSMEN` writer - AES bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: AES bus clock disabled in Sleep mode - 1: AES bus clock enabled in Sleep mode (if enabled in AESEN)
pub type AESSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DMA clock enable during Sleep mode bit This bit is set and reset by software. - 0: DMA clock disabled in Sleep mode - 1: DMA clock enabled in Sleep mode (if enabled in DMAEN)
    #[inline(always)]
    pub fn dmasmen(&self) -> DMASMEN_R {
        DMASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Flash clocks enable during Flash Sleep PD and CPU Sleep mode bit This bit is set and reset by software. - 0: Flash clocks are disabled in Flash Sleep PD* and CPU Sleep mode - 1: Flash clocks are enabled in Sleep mode Note: Flash Sleep PD is enabled through nvm_control register CONFIG.SLEEP_PD
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOA clock enable during Sleep mode bit This bit is set and reset by software. - 0: GPIOA clock disabled in Sleep mode - 1: GPIOA clock enabled in Sleep mode (if enabled by GPIOAEN)
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIOB clock enable during Sleep mode bit This bit is set and reset by software. - 0: GPIOB clock disabled in Sleep mode - 1: GPIOB clock enabled in Sleep mode (if enabled in GPIOBEN)
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 9 - SRAM0 clock enable during Sleep mode bit This bit is set and reset by software. - 0: SRAM0 clock disabled in Sleep mode - 1: SRAM0 clock enabled in Sleep mode
    #[inline(always)]
    pub fn sram0smen(&self) -> SRAM0SMEN_R {
        SRAM0SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SRAM1 clock enable during Sleep mode bit This bit is set and reset by software. - 0: SRAM1 clock disabled in Sleep mode - 1: SRAM1 clock enabled in Sleep mode
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable during Sleep mode bit This bit is set and reset by software. - 0: CRC clock disabled in Sleep mode - 1: CRC clock enabled in Sleep mode (if enabled in CRCEN)
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 18 - RNG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: RNG bus clock disabled in Sleep mode - 1: RNG bus clock enabled in Sleep mode (if enabled in RNGEN)
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - AES bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: AES bus clock disabled in Sleep mode - 1: AES bus clock enabled in Sleep mode (if enabled in AESEN)
    #[inline(always)]
    pub fn aessmen(&self) -> AESSMEN_R {
        AESSMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBSMENR")
            .field("dmasmen", &self.dmasmen())
            .field("flashsmen", &self.flashsmen())
            .field("gpioasmen", &self.gpioasmen())
            .field("gpiobsmen", &self.gpiobsmen())
            .field("sram0smen", &self.sram0smen())
            .field("sram1smen", &self.sram1smen())
            .field("crcsmen", &self.crcsmen())
            .field("rngsmen", &self.rngsmen())
            .field("aessmen", &self.aessmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA clock enable during Sleep mode bit This bit is set and reset by software. - 0: DMA clock disabled in Sleep mode - 1: DMA clock enabled in Sleep mode (if enabled in DMAEN)
    #[inline(always)]
    pub fn dmasmen(&mut self) -> DMASMEN_W<'_, AHBSMENRrs> {
        DMASMEN_W::new(self, 0)
    }
    ///Bit 1 - Flash clocks enable during Flash Sleep PD and CPU Sleep mode bit This bit is set and reset by software. - 0: Flash clocks are disabled in Flash Sleep PD* and CPU Sleep mode - 1: Flash clocks are enabled in Sleep mode Note: Flash Sleep PD is enabled through nvm_control register CONFIG.SLEEP_PD
    #[inline(always)]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<'_, AHBSMENRrs> {
        FLASHSMEN_W::new(self, 1)
    }
    ///Bit 2 - GPIOA clock enable during Sleep mode bit This bit is set and reset by software. - 0: GPIOA clock disabled in Sleep mode - 1: GPIOA clock enabled in Sleep mode (if enabled by GPIOAEN)
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<'_, AHBSMENRrs> {
        GPIOASMEN_W::new(self, 2)
    }
    ///Bit 3 - GPIOB clock enable during Sleep mode bit This bit is set and reset by software. - 0: GPIOB clock disabled in Sleep mode - 1: GPIOB clock enabled in Sleep mode (if enabled in GPIOBEN)
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<'_, AHBSMENRrs> {
        GPIOBSMEN_W::new(self, 3)
    }
    ///Bit 9 - SRAM0 clock enable during Sleep mode bit This bit is set and reset by software. - 0: SRAM0 clock disabled in Sleep mode - 1: SRAM0 clock enabled in Sleep mode
    #[inline(always)]
    pub fn sram0smen(&mut self) -> SRAM0SMEN_W<'_, AHBSMENRrs> {
        SRAM0SMEN_W::new(self, 9)
    }
    ///Bit 10 - SRAM1 clock enable during Sleep mode bit This bit is set and reset by software. - 0: SRAM1 clock disabled in Sleep mode - 1: SRAM1 clock enabled in Sleep mode
    #[inline(always)]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W<'_, AHBSMENRrs> {
        SRAM1SMEN_W::new(self, 10)
    }
    ///Bit 12 - CRC clock enable during Sleep mode bit This bit is set and reset by software. - 0: CRC clock disabled in Sleep mode - 1: CRC clock enabled in Sleep mode (if enabled in CRCEN)
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<'_, AHBSMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
    ///Bit 18 - RNG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: RNG bus clock disabled in Sleep mode - 1: RNG bus clock enabled in Sleep mode (if enabled in RNGEN)
    #[inline(always)]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<'_, AHBSMENRrs> {
        RNGSMEN_W::new(self, 18)
    }
    ///Bit 20 - AES bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: AES bus clock disabled in Sleep mode - 1: AES bus clock enabled in Sleep mode (if enabled in AESEN)
    #[inline(always)]
    pub fn aessmen(&mut self) -> AESSMEN_W<'_, AHBSMENRrs> {
        AESSMEN_W::new(self, 20)
    }
}
/**AHBSMENR register

You can [`read`](crate::Reg::read) this register and get [`ahbsmenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsmenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RCC:AHBSMENR)*/
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
///`reset()` method sets AHBSMENR to value 0x0014_160f
impl crate::Resettable for AHBSMENRrs {
    const RESET_VALUE: u32 = 0x0014_160f;
}
