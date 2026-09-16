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
///Field `GPIOFSMEN` reader - IO port F clocks enable during Sleep and Stop modes
pub type GPIOFSMEN_R = crate::BitReader;
///Field `GPIOFSMEN` writer - IO port F clocks enable during Sleep and Stop modes
pub type GPIOFSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGSMEN` reader - IO port G clocks enable during Sleep and Stop modes
pub type GPIOGSMEN_R = crate::BitReader;
///Field `GPIOGSMEN` writer - IO port G clocks enable during Sleep and Stop modes
pub type GPIOGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCMSRAMSMEN` reader - CCM SRAM interface clocks enable during Sleep and Stop modes
pub type CCMSRAMSMEN_R = crate::BitReader;
///Field `CCMSRAMSMEN` writer - CCM SRAM interface clocks enable during Sleep and Stop modes
pub type CCMSRAMSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2SMEN` reader - SRAM2 interface clocks enable during Sleep and Stop modes
pub type SRAM2SMEN_R = crate::BitReader;
///Field `SRAM2SMEN` writer - SRAM2 interface clocks enable during Sleep and Stop modes
pub type SRAM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC12SMEN` reader - ADC12 clocks enable during Sleep and Stop modes
pub type ADC12SMEN_R = crate::BitReader;
///Field `ADC12SMEN` writer - ADC12 clocks enable during Sleep and Stop modes
pub type ADC12SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC345SMEN` reader - ADC345 clock enable
pub type ADC345SMEN_R = crate::BitReader;
///Field `ADC345SMEN` writer - ADC345 clock enable
pub type ADC345SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC1SMEN` reader - DAC1 clock enable
pub type DAC1SMEN_R = crate::BitReader;
///Field `DAC1SMEN` writer - DAC1 clock enable
pub type DAC1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC2SMEN` reader - DAC2 clock enable
pub type DAC2SMEN_R = crate::BitReader;
///Field `DAC2SMEN` writer - DAC2 clock enable
pub type DAC2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC3SMEN` reader - DAC3 clock enable
pub type DAC3SMEN_R = crate::BitReader;
///Field `DAC3SMEN` writer - DAC3 clock enable
pub type DAC3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC4SMEN` reader - DAC4 clock enable
pub type DAC4SMEN_R = crate::BitReader;
///Field `DAC4SMEN` writer - DAC4 clock enable
pub type DAC4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESSMEN` reader - AESM clocks enable
pub type AESSMEN_R = crate::BitReader;
///Field `AESSMEN` writer - AESM clocks enable
pub type AESSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGEN` reader - RNG enable
pub type RNGEN_R = crate::BitReader;
///Field `RNGEN` writer - RNG enable
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 5 - IO port F clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GPIOFSMEN_R {
        GPIOFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiogsmen(&self) -> GPIOGSMEN_R {
        GPIOGSMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - CCM SRAM interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn ccmsramsmen(&self) -> CCMSRAMSMEN_R {
        CCMSRAMSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - ADC12 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn adc12smen(&self) -> ADC12SMEN_R {
        ADC12SMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ADC345 clock enable
    #[inline(always)]
    pub fn adc345smen(&self) -> ADC345SMEN_R {
        ADC345SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - DAC1 clock enable
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DAC2 clock enable
    #[inline(always)]
    pub fn dac2smen(&self) -> DAC2SMEN_R {
        DAC2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DAC3 clock enable
    #[inline(always)]
    pub fn dac3smen(&self) -> DAC3SMEN_R {
        DAC3SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DAC4 clock enable
    #[inline(always)]
    pub fn dac4smen(&self) -> DAC4SMEN_R {
        DAC4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - AESM clocks enable
    #[inline(always)]
    pub fn aessmen(&self) -> AESSMEN_R {
        AESSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - RNG enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2SMENR")
            .field("gpioasmen", &self.gpioasmen())
            .field("gpiobsmen", &self.gpiobsmen())
            .field("gpiocsmen", &self.gpiocsmen())
            .field("gpiodsmen", &self.gpiodsmen())
            .field("gpioesmen", &self.gpioesmen())
            .field("gpiofsmen", &self.gpiofsmen())
            .field("gpiogsmen", &self.gpiogsmen())
            .field("ccmsramsmen", &self.ccmsramsmen())
            .field("sram2smen", &self.sram2smen())
            .field("adc12smen", &self.adc12smen())
            .field("adc345smen", &self.adc345smen())
            .field("dac1smen", &self.dac1smen())
            .field("dac2smen", &self.dac2smen())
            .field("dac3smen", &self.dac3smen())
            .field("dac4smen", &self.dac4smen())
            .field("aessmen", &self.aessmen())
            .field("rngen", &self.rngen())
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
    ///Bit 5 - IO port F clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiofsmen(&mut self) -> GPIOFSMEN_W<'_, AHB2SMENRrs> {
        GPIOFSMEN_W::new(self, 5)
    }
    ///Bit 6 - IO port G clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiogsmen(&mut self) -> GPIOGSMEN_W<'_, AHB2SMENRrs> {
        GPIOGSMEN_W::new(self, 6)
    }
    ///Bit 9 - CCM SRAM interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn ccmsramsmen(&mut self) -> CCMSRAMSMEN_W<'_, AHB2SMENRrs> {
        CCMSRAMSMEN_W::new(self, 9)
    }
    ///Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<'_, AHB2SMENRrs> {
        SRAM2SMEN_W::new(self, 10)
    }
    ///Bit 13 - ADC12 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn adc12smen(&mut self) -> ADC12SMEN_W<'_, AHB2SMENRrs> {
        ADC12SMEN_W::new(self, 13)
    }
    ///Bit 14 - ADC345 clock enable
    #[inline(always)]
    pub fn adc345smen(&mut self) -> ADC345SMEN_W<'_, AHB2SMENRrs> {
        ADC345SMEN_W::new(self, 14)
    }
    ///Bit 16 - DAC1 clock enable
    #[inline(always)]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<'_, AHB2SMENRrs> {
        DAC1SMEN_W::new(self, 16)
    }
    ///Bit 17 - DAC2 clock enable
    #[inline(always)]
    pub fn dac2smen(&mut self) -> DAC2SMEN_W<'_, AHB2SMENRrs> {
        DAC2SMEN_W::new(self, 17)
    }
    ///Bit 18 - DAC3 clock enable
    #[inline(always)]
    pub fn dac3smen(&mut self) -> DAC3SMEN_W<'_, AHB2SMENRrs> {
        DAC3SMEN_W::new(self, 18)
    }
    ///Bit 19 - DAC4 clock enable
    #[inline(always)]
    pub fn dac4smen(&mut self) -> DAC4SMEN_W<'_, AHB2SMENRrs> {
        DAC4SMEN_W::new(self, 19)
    }
    ///Bit 24 - AESM clocks enable
    #[inline(always)]
    pub fn aessmen(&mut self) -> AESSMEN_W<'_, AHB2SMENRrs> {
        AESSMEN_W::new(self, 24)
    }
    ///Bit 26 - RNG enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<'_, AHB2SMENRrs> {
        RNGEN_W::new(self, 26)
    }
}
/**AHB2 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb2smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G4A1.html#RCC:AHB2SMENR)*/
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
///`reset()` method sets AHB2SMENR to value 0x050f_667f
impl crate::Resettable for AHB2SMENRrs {
    const RESET_VALUE: u32 = 0x050f_667f;
}
