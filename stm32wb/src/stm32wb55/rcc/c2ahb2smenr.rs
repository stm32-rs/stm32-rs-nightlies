#[doc = "Register `C2AHB2SMENR` reader"]
pub type R = crate::R<C2AHB2SMENRrs>;
#[doc = "Register `C2AHB2SMENR` writer"]
pub type W = crate::W<C2AHB2SMENRrs>;
#[doc = "Field `GPIOASMEN` reader - CPU2 IO port A clocks enable during Sleep and Stop modes"]
pub type GPIOASMEN_R = crate::BitReader;
#[doc = "Field `GPIOASMEN` writer - CPU2 IO port A clocks enable during Sleep and Stop modes"]
pub type GPIOASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBSMEN` reader - CPU2 IO port B clocks enable during Sleep and Stop modes"]
pub type GPIOBSMEN_R = crate::BitReader;
#[doc = "Field `GPIOBSMEN` writer - CPU2 IO port B clocks enable during Sleep and Stop modes"]
pub type GPIOBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCSMEN` reader - CPU2 IO port C clocks enable during Sleep and Stop modes"]
pub type GPIOCSMEN_R = crate::BitReader;
#[doc = "Field `GPIOCSMEN` writer - CPU2 IO port C clocks enable during Sleep and Stop modes"]
pub type GPIOCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODSMEN` reader - CPU2 IO port D clocks enable during Sleep and Stop modes"]
pub type GPIODSMEN_R = crate::BitReader;
#[doc = "Field `GPIODSMEN` writer - CPU2 IO port D clocks enable during Sleep and Stop modes"]
pub type GPIODSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOESMEN` reader - CPU2 IO port E clocks enable during Sleep and Stop modes"]
pub type GPIOESMEN_R = crate::BitReader;
#[doc = "Field `GPIOESMEN` writer - CPU2 IO port E clocks enable during Sleep and Stop modes"]
pub type GPIOESMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHSMEN` reader - CPU2 IO port H clocks enable during Sleep and Stop modes"]
pub type GPIOHSMEN_R = crate::BitReader;
#[doc = "Field `GPIOHSMEN` writer - CPU2 IO port H clocks enable during Sleep and Stop modes"]
pub type GPIOHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCFSSMEN` reader - CPU2 ADC clocks enable during Sleep and Stop modes"]
pub type ADCFSSMEN_R = crate::BitReader;
#[doc = "Field `ADCFSSMEN` writer - CPU2 ADC clocks enable during Sleep and Stop modes"]
pub type ADCFSSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES1SMEN` reader - CPU2 AES1 accelerator clocks enable during Sleep and Stop modes"]
pub type AES1SMEN_R = crate::BitReader;
#[doc = "Field `AES1SMEN` writer - CPU2 AES1 accelerator clocks enable during Sleep and Stop modes"]
pub type AES1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPU2 IO port A clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU2 IO port B clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU2 IO port C clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU2 IO port D clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU2 IO port E clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU2 IO port H clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU2 ADC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn adcfssmen(&self) -> ADCFSSMEN_R {
        ADCFSSMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU2 AES1 accelerator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn aes1smen(&self) -> AES1SMEN_R {
        AES1SMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU2 IO port A clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<C2AHB2SMENRrs> {
        GPIOASMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU2 IO port B clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<C2AHB2SMENRrs> {
        GPIOBSMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU2 IO port C clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<C2AHB2SMENRrs> {
        GPIOCSMEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - CPU2 IO port D clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<C2AHB2SMENRrs> {
        GPIODSMEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - CPU2 IO port E clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W<C2AHB2SMENRrs> {
        GPIOESMEN_W::new(self, 4)
    }
    #[doc = "Bit 7 - CPU2 IO port H clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W<C2AHB2SMENRrs> {
        GPIOHSMEN_W::new(self, 7)
    }
    #[doc = "Bit 13 - CPU2 ADC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn adcfssmen(&mut self) -> ADCFSSMEN_W<C2AHB2SMENRrs> {
        ADCFSSMEN_W::new(self, 13)
    }
    #[doc = "Bit 16 - CPU2 AES1 accelerator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn aes1smen(&mut self) -> AES1SMEN_W<C2AHB2SMENRrs> {
        AES1SMEN_W::new(self, 16)
    }
}
#[doc = "CPU2 AHB2 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2ahb2smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2ahb2smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2AHB2SMENRrs;
impl crate::RegisterSpec for C2AHB2SMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2ahb2smenr::R`](R) reader structure"]
impl crate::Readable for C2AHB2SMENRrs {}
#[doc = "`write(|w| ..)` method takes [`c2ahb2smenr::W`](W) writer structure"]
impl crate::Writable for C2AHB2SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2AHB2SMENR to value 0x0001_209f"]
impl crate::Resettable for C2AHB2SMENRrs {
    const RESET_VALUE: u32 = 0x0001_209f;
}
