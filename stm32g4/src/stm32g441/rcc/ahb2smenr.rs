#[doc = "Register `AHB2SMENR` reader"]
pub type R = crate::R<AHB2SMENRrs>;
#[doc = "Register `AHB2SMENR` writer"]
pub type W = crate::W<AHB2SMENRrs>;
#[doc = "Field `GPIOASMEN` reader - IO port A clocks enable during Sleep and Stop modes"]
pub type GPIOASMEN_R = crate::BitReader;
#[doc = "Field `GPIOASMEN` writer - IO port A clocks enable during Sleep and Stop modes"]
pub type GPIOASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBSMEN` reader - IO port B clocks enable during Sleep and Stop modes"]
pub type GPIOBSMEN_R = crate::BitReader;
#[doc = "Field `GPIOBSMEN` writer - IO port B clocks enable during Sleep and Stop modes"]
pub type GPIOBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCSMEN` reader - IO port C clocks enable during Sleep and Stop modes"]
pub type GPIOCSMEN_R = crate::BitReader;
#[doc = "Field `GPIOCSMEN` writer - IO port C clocks enable during Sleep and Stop modes"]
pub type GPIOCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODSMEN` reader - IO port D clocks enable during Sleep and Stop modes"]
pub type GPIODSMEN_R = crate::BitReader;
#[doc = "Field `GPIODSMEN` writer - IO port D clocks enable during Sleep and Stop modes"]
pub type GPIODSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOESMEN` reader - IO port E clocks enable during Sleep and Stop modes"]
pub type GPIOESMEN_R = crate::BitReader;
#[doc = "Field `GPIOESMEN` writer - IO port E clocks enable during Sleep and Stop modes"]
pub type GPIOESMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFSMEN` reader - IO port F clocks enable during Sleep and Stop modes"]
pub type GPIOFSMEN_R = crate::BitReader;
#[doc = "Field `GPIOFSMEN` writer - IO port F clocks enable during Sleep and Stop modes"]
pub type GPIOFSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGSMEN` reader - IO port G clocks enable during Sleep and Stop modes"]
pub type GPIOGSMEN_R = crate::BitReader;
#[doc = "Field `GPIOGSMEN` writer - IO port G clocks enable during Sleep and Stop modes"]
pub type GPIOGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCMSRAMSMEN` reader - CCM SRAM interface clocks enable during Sleep and Stop modes"]
pub type CCMSRAMSMEN_R = crate::BitReader;
#[doc = "Field `CCMSRAMSMEN` writer - CCM SRAM interface clocks enable during Sleep and Stop modes"]
pub type CCMSRAMSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2SMEN` reader - SRAM2 interface clocks enable during Sleep and Stop modes"]
pub type SRAM2SMEN_R = crate::BitReader;
#[doc = "Field `SRAM2SMEN` writer - SRAM2 interface clocks enable during Sleep and Stop modes"]
pub type SRAM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12SMEN` reader - ADC clocks enable during Sleep and Stop modes"]
pub type ADC12SMEN_R = crate::BitReader;
#[doc = "Field `ADC12SMEN` writer - ADC clocks enable during Sleep and Stop modes"]
pub type ADC12SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC345SMEN` reader - DCMI clock enable during Sleep and Stop modes"]
pub type ADC345SMEN_R = crate::BitReader;
#[doc = "Field `ADC345SMEN` writer - DCMI clock enable during Sleep and Stop modes"]
pub type ADC345SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1SMEN` reader - AES accelerator clocks enable during Sleep and Stop modes"]
pub type DAC1SMEN_R = crate::BitReader;
#[doc = "Field `DAC1SMEN` writer - AES accelerator clocks enable during Sleep and Stop modes"]
pub type DAC1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC2SMEN` reader - HASH clock enable during Sleep and Stop modes"]
pub type DAC2SMEN_R = crate::BitReader;
#[doc = "Field `DAC2SMEN` writer - HASH clock enable during Sleep and Stop modes"]
pub type DAC2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC3SMEN` reader - DAC3 clock enable during sleep mode"]
pub type DAC3SMEN_R = crate::BitReader;
#[doc = "Field `DAC3SMEN` writer - DAC3 clock enable during sleep mode"]
pub type DAC3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC4SMEN` reader - DAC4 clock enable during sleep mode"]
pub type DAC4SMEN_R = crate::BitReader;
#[doc = "Field `DAC4SMEN` writer - DAC4 clock enable during sleep mode"]
pub type DAC4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESMEN` reader - Cryptography clock enable during sleep mode"]
pub type AESMEN_R = crate::BitReader;
#[doc = "Field `AESMEN` writer - Cryptography clock enable during sleep mode"]
pub type AESMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGSMEN` reader - Random Number Generator clock enable during sleep mode"]
pub type RNGSMEN_R = crate::BitReader;
#[doc = "Field `RNGSMEN` writer - Random Number Generator clock enable during sleep mode"]
pub type RNGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO port A clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GPIOFSMEN_R {
        GPIOFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiogsmen(&self) -> GPIOGSMEN_R {
        GPIOGSMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - CCM SRAM interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn ccmsramsmen(&self) -> CCMSRAMSMEN_R {
        CCMSRAMSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn adc12smen(&self) -> ADC12SMEN_R {
        ADC12SMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DCMI clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn adc345smen(&self) -> ADC345SMEN_R {
        ADC345SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - AES accelerator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dac2smen(&self) -> DAC2SMEN_R {
        DAC2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC3 clock enable during sleep mode"]
    #[inline(always)]
    pub fn dac3smen(&self) -> DAC3SMEN_R {
        DAC3SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DAC4 clock enable during sleep mode"]
    #[inline(always)]
    pub fn dac4smen(&self) -> DAC4SMEN_R {
        DAC4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Cryptography clock enable during sleep mode"]
    #[inline(always)]
    pub fn aesmen(&self) -> AESMEN_R {
        AESMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Random Number Generator clock enable during sleep mode"]
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<AHB2SMENRrs> {
        GPIOASMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<AHB2SMENRrs> {
        GPIOBSMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<AHB2SMENRrs> {
        GPIOCSMEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<AHB2SMENRrs> {
        GPIODSMEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W<AHB2SMENRrs> {
        GPIOESMEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO port F clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofsmen(&mut self) -> GPIOFSMEN_W<AHB2SMENRrs> {
        GPIOFSMEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - IO port G clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogsmen(&mut self) -> GPIOGSMEN_W<AHB2SMENRrs> {
        GPIOGSMEN_W::new(self, 6)
    }
    #[doc = "Bit 9 - CCM SRAM interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn ccmsramsmen(&mut self) -> CCMSRAMSMEN_W<AHB2SMENRrs> {
        CCMSRAMSMEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<AHB2SMENRrs> {
        SRAM2SMEN_W::new(self, 10)
    }
    #[doc = "Bit 13 - ADC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn adc12smen(&mut self) -> ADC12SMEN_W<AHB2SMENRrs> {
        ADC12SMEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - DCMI clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn adc345smen(&mut self) -> ADC345SMEN_W<AHB2SMENRrs> {
        ADC345SMEN_W::new(self, 14)
    }
    #[doc = "Bit 16 - AES accelerator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<AHB2SMENRrs> {
        DAC1SMEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - HASH clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dac2smen(&mut self) -> DAC2SMEN_W<AHB2SMENRrs> {
        DAC2SMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - DAC3 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dac3smen(&mut self) -> DAC3SMEN_W<AHB2SMENRrs> {
        DAC3SMEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - DAC4 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dac4smen(&mut self) -> DAC4SMEN_W<AHB2SMENRrs> {
        DAC4SMEN_W::new(self, 19)
    }
    #[doc = "Bit 24 - Cryptography clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn aesmen(&mut self) -> AESMEN_W<AHB2SMENRrs> {
        AESMEN_W::new(self, 24)
    }
    #[doc = "Bit 26 - Random Number Generator clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<AHB2SMENRrs> {
        RNGSMEN_W::new(self, 26)
    }
}
#[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2SMENRrs;
impl crate::RegisterSpec for AHB2SMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2smenr::R`](R) reader structure"]
impl crate::Readable for AHB2SMENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb2smenr::W`](W) writer structure"]
impl crate::Writable for AHB2SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2SMENR to value 0x050f_667f"]
impl crate::Resettable for AHB2SMENRrs {
    const RESET_VALUE: u32 = 0x050f_667f;
}
