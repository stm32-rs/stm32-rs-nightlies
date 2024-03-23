#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "Field `ADRDYIE` reader - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type ADRDYIE_R = crate::BitReader;
#[doc = "Field `ADRDYIE` writer - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type ADRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSMPIE` reader - End of sampling flag interrupt enable This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EOSMPIE_R = crate::BitReader;
#[doc = "Field `EOSMPIE` writer - End of sampling flag interrupt enable This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EOSMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCIE` reader - End of conversion interrupt enable This bit is set and cleared by software to enable/disable the end of conversion interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EOCIE_R = crate::BitReader;
#[doc = "Field `EOCIE` writer - End of conversion interrupt enable This bit is set and cleared by software to enable/disable the end of conversion interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSIE` reader - End of conversion sequence interrupt enable This bit is set and cleared by software to enable/disable the end of sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EOSIE_R = crate::BitReader;
#[doc = "Field `EOSIE` writer - End of conversion sequence interrupt enable This bit is set and cleared by software to enable/disable the end of sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EOSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIE` reader - Overrun interrupt enable This bit is set and cleared by software to enable/disable the overrun interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type OVRIE_R = crate::BitReader;
#[doc = "Field `OVRIE` writer - Overrun interrupt enable This bit is set and cleared by software to enable/disable the overrun interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1IE` reader - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1IE_R = crate::BitReader;
#[doc = "Field `AWD1IE` writer - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD2IE` reader - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD2IE_R = crate::BitReader;
#[doc = "Field `AWD2IE` writer - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD3IE` reader - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD3IE_R = crate::BitReader;
#[doc = "Field `AWD3IE` writer - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCALIE` reader - End of calibration interrupt enable This bit is set and cleared by software to enable/disable the end of calibration interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EOCALIE_R = crate::BitReader;
#[doc = "Field `EOCALIE` writer - End of calibration interrupt enable This bit is set and cleared by software to enable/disable the end of calibration interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EOCALIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCRDYIE` reader - Channel Configuration Ready Interrupt enable This bit is set and cleared by software to enable/disable the channel configuration ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type CCRDYIE_R = crate::BitReader;
#[doc = "Field `CCRDYIE` writer - Channel Configuration Ready Interrupt enable This bit is set and cleared by software to enable/disable the channel configuration ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type CCRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of sampling flag interrupt enable This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of conversion interrupt enable This bit is set and cleared by software to enable/disable the end of conversion interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of conversion sequence interrupt enable This bit is set and cleared by software to enable/disable the end of sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overrun interrupt enable This bit is set and cleared by software to enable/disable the overrun interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd1ie(&self) -> AWD1IE_R {
        AWD1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd2ie(&self) -> AWD2IE_R {
        AWD2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd3ie(&self) -> AWD3IE_R {
        AWD3IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - End of calibration interrupt enable This bit is set and cleared by software to enable/disable the end of calibration interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn eocalie(&self) -> EOCALIE_R {
        EOCALIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel Configuration Ready Interrupt enable This bit is set and cleared by software to enable/disable the channel configuration ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ccrdyie(&self) -> CCRDYIE_R {
        CCRDYIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<IERrs> {
        ADRDYIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - End of sampling flag interrupt enable This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<IERrs> {
        EOSMPIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - End of conversion interrupt enable This bit is set and cleared by software to enable/disable the end of conversion interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<IERrs> {
        EOCIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - End of conversion sequence interrupt enable This bit is set and cleared by software to enable/disable the end of sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn eosie(&mut self) -> EOSIE_W<IERrs> {
        EOSIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Overrun interrupt enable This bit is set and cleared by software to enable/disable the overrun interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<IERrs> {
        OVRIE_W::new(self, 4)
    }
    #[doc = "Bit 7 - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd1ie(&mut self) -> AWD1IE_W<IERrs> {
        AWD1IE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ie(&mut self) -> AWD2IE_W<IERrs> {
        AWD2IE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd3ie(&mut self) -> AWD3IE_W<IERrs> {
        AWD3IE_W::new(self, 9)
    }
    #[doc = "Bit 11 - End of calibration interrupt enable This bit is set and cleared by software to enable/disable the end of calibration interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn eocalie(&mut self) -> EOCALIE_W<IERrs> {
        EOCALIE_W::new(self, 11)
    }
    #[doc = "Bit 13 - Channel Configuration Ready Interrupt enable This bit is set and cleared by software to enable/disable the channel configuration ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ccrdyie(&mut self) -> CCRDYIE_W<IERrs> {
        CCRDYIE_W::new(self, 13)
    }
}
#[doc = "ADC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IERrs {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
