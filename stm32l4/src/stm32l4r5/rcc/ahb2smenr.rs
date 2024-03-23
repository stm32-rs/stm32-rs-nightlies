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
#[doc = "Field `GPIOHSMEN` reader - IO port H clocks enable during Sleep and Stop modes"]
pub type GPIOHSMEN_R = crate::BitReader;
#[doc = "Field `GPIOHSMEN` writer - IO port H clocks enable during Sleep and Stop modes"]
pub type GPIOHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOISMEN` reader - IO port I clocks enable during Sleep and Stop modes"]
pub type GPIOISMEN_R = crate::BitReader;
#[doc = "Field `GPIOISMEN` writer - IO port I clocks enable during Sleep and Stop modes"]
pub type GPIOISMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2SMEN` reader - SRAM2 interface clocks enable during Sleep and Stop modes"]
pub type SRAM2SMEN_R = crate::BitReader;
#[doc = "Field `SRAM2SMEN` writer - SRAM2 interface clocks enable during Sleep and Stop modes"]
pub type SRAM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3SMEN` reader - SRAM2 interface clocks enable during Sleep and Stop modes"]
pub type SRAM3SMEN_R = crate::BitReader;
#[doc = "Field `SRAM3SMEN` writer - SRAM2 interface clocks enable during Sleep and Stop modes"]
pub type SRAM3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGFSSMEN` reader - OTG full speed clocks enable during Sleep and Stop modes"]
pub type OTGFSSMEN_R = crate::BitReader;
#[doc = "Field `OTGFSSMEN` writer - OTG full speed clocks enable during Sleep and Stop modes"]
pub type OTGFSSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCFSSMEN` reader - ADC clocks enable during Sleep and Stop modes"]
pub type ADCFSSMEN_R = crate::BitReader;
#[doc = "Field `ADCFSSMEN` writer - ADC clocks enable during Sleep and Stop modes"]
pub type ADCFSSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMISMEN` reader - DCMI clock enable during Sleep and Stop modes"]
pub type DCMISMEN_R = crate::BitReader;
#[doc = "Field `DCMISMEN` writer - DCMI clock enable during Sleep and Stop modes"]
pub type DCMISMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESSMEN` reader - AES accelerator clocks enable during Sleep and Stop modes"]
pub type AESSMEN_R = crate::BitReader;
#[doc = "Field `AESSMEN` writer - AES accelerator clocks enable during Sleep and Stop modes"]
pub type AESSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHSMEN` reader - HASH clock enable during Sleep and Stop modes"]
pub type HASHSMEN_R = crate::BitReader;
#[doc = "Field `HASHSMEN` writer - HASH clock enable during Sleep and Stop modes"]
pub type HASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGSMEN` reader - Random Number Generator clocks enable during Sleep and Stop modes"]
pub type RNGSMEN_R = crate::BitReader;
#[doc = "Field `RNGSMEN` writer - Random Number Generator clocks enable during Sleep and Stop modes"]
pub type RNGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPIMSMEN` reader - OctoSPI IO manager clocks enable during Sleep and Stop modes"]
pub type OSPIMSMEN_R = crate::BitReader;
#[doc = "Field `OSPIMSMEN` writer - OctoSPI IO manager clocks enable during Sleep and Stop modes"]
pub type OSPIMSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC1SMEN` reader - SDMMC1 clocks enable during Sleep and Stop modes"]
pub type SDMMC1SMEN_R = crate::BitReader;
#[doc = "Field `SDMMC1SMEN` writer - SDMMC1 clocks enable during Sleep and Stop modes"]
pub type SDMMC1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 7 - IO port H clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IO port I clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioismen(&self) -> GPIOISMEN_R {
        GPIOISMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram3smen(&self) -> SRAM3SMEN_R {
        SRAM3SMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - OTG full speed clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn otgfssmen(&self) -> OTGFSSMEN_R {
        OTGFSSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn adcfssmen(&self) -> ADCFSSMEN_R {
        ADCFSSMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DCMI clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dcmismen(&self) -> DCMISMEN_R {
        DCMISMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - AES accelerator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn aessmen(&self) -> AESSMEN_R {
        AESSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn hashsmen(&self) -> HASHSMEN_R {
        HASHSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Random Number Generator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - OctoSPI IO manager clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn ospimsmen(&self) -> OSPIMSMEN_R {
        OSPIMSMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - SDMMC1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sdmmc1smen(&self) -> SDMMC1SMEN_R {
        SDMMC1SMEN_R::new(((self.bits >> 22) & 1) != 0)
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
    #[doc = "Bit 7 - IO port H clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W<AHB2SMENRrs> {
        GPIOHSMEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - IO port I clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpioismen(&mut self) -> GPIOISMEN_W<AHB2SMENRrs> {
        GPIOISMEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<AHB2SMENRrs> {
        SRAM2SMEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sram3smen(&mut self) -> SRAM3SMEN_W<AHB2SMENRrs> {
        SRAM3SMEN_W::new(self, 10)
    }
    #[doc = "Bit 12 - OTG full speed clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn otgfssmen(&mut self) -> OTGFSSMEN_W<AHB2SMENRrs> {
        OTGFSSMEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - ADC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn adcfssmen(&mut self) -> ADCFSSMEN_W<AHB2SMENRrs> {
        ADCFSSMEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - DCMI clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dcmismen(&mut self) -> DCMISMEN_W<AHB2SMENRrs> {
        DCMISMEN_W::new(self, 14)
    }
    #[doc = "Bit 16 - AES accelerator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn aessmen(&mut self) -> AESSMEN_W<AHB2SMENRrs> {
        AESSMEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - HASH clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn hashsmen(&mut self) -> HASHSMEN_W<AHB2SMENRrs> {
        HASHSMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Random Number Generator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<AHB2SMENRrs> {
        RNGSMEN_W::new(self, 18)
    }
    #[doc = "Bit 20 - OctoSPI IO manager clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn ospimsmen(&mut self) -> OSPIMSMEN_W<AHB2SMENRrs> {
        OSPIMSMEN_W::new(self, 20)
    }
    #[doc = "Bit 22 - SDMMC1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1smen(&mut self) -> SDMMC1SMEN_W<AHB2SMENRrs> {
        SDMMC1SMEN_W::new(self, 22)
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
#[doc = "`reset()` method sets AHB2SMENR to value 0x0005_32ff"]
impl crate::Resettable for AHB2SMENRrs {
    const RESET_VALUE: u32 = 0x0005_32ff;
}
