///Register `AHB2SMENR` reader
pub type R = crate::R<AHB2SMENRrs>;
///Register `AHB2SMENR` writer
pub type W = crate::W<AHB2SMENRrs>;
///Field `GPIOASMEN` reader - IO port A clocks enable during Sleep and Stop modes
pub type GPIOASMEN_R = crate::BitReader;
///Field `GPIOASMEN` writer - IO port A clocks enable during Sleep and Stop modes
pub type GPIOASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBSMEN` reader - IO port B clocks enable during Sleep and Stop modes
pub type GPIOBSMEN_R = crate::BitReader;
///Field `GPIOBSMEN` writer - IO port B clocks enable during Sleep and Stop modes
pub type GPIOBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCSMEN` reader - IO port C clocks enable during Sleep and Stop modes
pub type GPIOCSMEN_R = crate::BitReader;
///Field `GPIOCSMEN` writer - IO port C clocks enable during Sleep and Stop modes
pub type GPIOCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODSMEN` reader - IO port D clocks enable during Sleep and Stop modes
pub type GPIODSMEN_R = crate::BitReader;
///Field `GPIODSMEN` writer - IO port D clocks enable during Sleep and Stop modes
pub type GPIODSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOESMEN` reader - IO port E clocks enable during Sleep and Stop modes
pub type GPIOESMEN_R = crate::BitReader;
///Field `GPIOESMEN` writer - IO port E clocks enable during Sleep and Stop modes
pub type GPIOESMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHSMEN` reader - IO port H clocks enable during Sleep and Stop modes
pub type GPIOHSMEN_R = crate::BitReader;
///Field `GPIOHSMEN` writer - IO port H clocks enable during Sleep and Stop modes
pub type GPIOHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2SMEN` reader - SRAM2 interface clocks enable during Sleep and Stop modes
pub type SRAM2SMEN_R = crate::BitReader;
///Field `SRAM2SMEN` writer - SRAM2 interface clocks enable during Sleep and Stop modes
pub type SRAM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCFSSMEN` reader - ADC clocks enable during Sleep and Stop modes
pub type ADCFSSMEN_R = crate::BitReader;
///Field `ADCFSSMEN` writer - ADC clocks enable during Sleep and Stop modes
pub type ADCFSSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESSMEN` reader - AES accelerator clocks enable during Sleep and Stop modes
pub type AESSMEN_R = crate::BitReader;
///Field `AESSMEN` writer - AES accelerator clocks enable during Sleep and Stop modes
pub type AESSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGSMEN` reader - Random Number Generator clocks enable during Sleep and Stop modes
pub type RNGSMEN_R = crate::BitReader;
///Field `RNGSMEN` writer - Random Number Generator clocks enable during Sleep and Stop modes
pub type RNGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - IO port A clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - IO port H clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - SRAM2 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - ADC clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn adcfssmen(&self) -> ADCFSSMEN_R {
        ADCFSSMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - AES accelerator clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn aessmen(&self) -> AESSMEN_R {
        AESSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Random Number Generator clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2SMENR")
            .field("rngsmen", &self.rngsmen())
            .field("aessmen", &self.aessmen())
            .field("adcfssmen", &self.adcfssmen())
            .field("sram2smen", &self.sram2smen())
            .field("gpiohsmen", &self.gpiohsmen())
            .field("gpioesmen", &self.gpioesmen())
            .field("gpiodsmen", &self.gpiodsmen())
            .field("gpiocsmen", &self.gpiocsmen())
            .field("gpiobsmen", &self.gpiobsmen())
            .field("gpioasmen", &self.gpioasmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<'_, AHB2SMENRrs> {
        GPIOASMEN_W::new(self, 0)
    }
    ///Bit 1 - IO port B clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<'_, AHB2SMENRrs> {
        GPIOBSMEN_W::new(self, 1)
    }
    ///Bit 2 - IO port C clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<'_, AHB2SMENRrs> {
        GPIOCSMEN_W::new(self, 2)
    }
    ///Bit 3 - IO port D clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<'_, AHB2SMENRrs> {
        GPIODSMEN_W::new(self, 3)
    }
    ///Bit 4 - IO port E clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W<'_, AHB2SMENRrs> {
        GPIOESMEN_W::new(self, 4)
    }
    ///Bit 7 - IO port H clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W<'_, AHB2SMENRrs> {
        GPIOHSMEN_W::new(self, 7)
    }
    ///Bit 9 - SRAM2 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<'_, AHB2SMENRrs> {
        SRAM2SMEN_W::new(self, 9)
    }
    ///Bit 13 - ADC clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn adcfssmen(&mut self) -> ADCFSSMEN_W<'_, AHB2SMENRrs> {
        ADCFSSMEN_W::new(self, 13)
    }
    ///Bit 16 - AES accelerator clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn aessmen(&mut self) -> AESSMEN_W<'_, AHB2SMENRrs> {
        AESSMEN_W::new(self, 16)
    }
    ///Bit 18 - Random Number Generator clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<'_, AHB2SMENRrs> {
        RNGSMEN_W::new(self, 18)
    }
}
/**AHB2 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb2smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x1.html#RCC:AHB2SMENR)*/
pub struct AHB2SMENRrs;
impl crate::RegisterSpec for AHB2SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2smenr::R`](R) reader structure
impl crate::Readable for AHB2SMENRrs {}
///`write(|w| ..)` method takes [`ahb2smenr::W`](W) writer structure
impl crate::Writable for AHB2SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2SMENR to value 0x0005_32ff
impl crate::Resettable for AHB2SMENRrs {
    const RESET_VALUE: u32 = 0x0005_32ff;
}
