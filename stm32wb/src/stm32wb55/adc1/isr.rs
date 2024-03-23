#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<ISRrs>;
#[doc = "Field `ADRDY` reader - ADC ready flag"]
pub type ADRDY_R = crate::BitReader;
#[doc = "Field `ADRDY` writer - ADC ready flag"]
pub type ADRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSMP` reader - ADC group regular end of sampling flag"]
pub type EOSMP_R = crate::BitReader;
#[doc = "Field `EOSMP` writer - ADC group regular end of sampling flag"]
pub type EOSMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC` reader - ADC group regular end of unitary conversion flag"]
pub type EOC_R = crate::BitReader;
#[doc = "Field `EOC` writer - ADC group regular end of unitary conversion flag"]
pub type EOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOS` reader - ADC group regular end of sequence conversions flag"]
pub type EOS_R = crate::BitReader;
#[doc = "Field `EOS` writer - ADC group regular end of sequence conversions flag"]
pub type EOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR` reader - ADC group regular overrun flag"]
pub type OVR_R = crate::BitReader;
#[doc = "Field `OVR` writer - ADC group regular overrun flag"]
pub type OVR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOC` reader - ADC group injected end of unitary conversion flag"]
pub type JEOC_R = crate::BitReader;
#[doc = "Field `JEOC` writer - ADC group injected end of unitary conversion flag"]
pub type JEOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOS` reader - ADC group injected end of sequence conversions flag"]
pub type JEOS_R = crate::BitReader;
#[doc = "Field `JEOS` writer - ADC group injected end of sequence conversions flag"]
pub type JEOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1` reader - ADC analog watchdog 1 flag"]
pub type AWD1_R = crate::BitReader;
#[doc = "Field `AWD1` writer - ADC analog watchdog 1 flag"]
pub type AWD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD2` reader - ADC analog watchdog 2 flag"]
pub type AWD2_R = crate::BitReader;
#[doc = "Field `AWD2` writer - ADC analog watchdog 2 flag"]
pub type AWD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD3` reader - ADC analog watchdog 3 flag"]
pub type AWD3_R = crate::BitReader;
#[doc = "Field `AWD3` writer - ADC analog watchdog 3 flag"]
pub type AWD3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JQOVF` reader - ADC group injected contexts queue overflow flag"]
pub type JQOVF_R = crate::BitReader;
#[doc = "Field `JQOVF` writer - ADC group injected contexts queue overflow flag"]
pub type JQOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC ready flag"]
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC group regular end of sampling flag"]
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion flag"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions flag"]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC group regular overrun flag"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC group injected end of unitary conversion flag"]
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC group injected end of sequence conversions flag"]
    #[inline(always)]
    pub fn jeos(&self) -> JEOS_R {
        JEOS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC analog watchdog 1 flag"]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC analog watchdog 2 flag"]
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC analog watchdog 3 flag"]
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC group injected contexts queue overflow flag"]
    #[inline(always)]
    pub fn jqovf(&self) -> JQOVF_R {
        JQOVF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn adrdy(&mut self) -> ADRDY_W<ISRrs> {
        ADRDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC group regular end of sampling flag"]
    #[inline(always)]
    #[must_use]
    pub fn eosmp(&mut self) -> EOSMP_W<ISRrs> {
        EOSMP_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion flag"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<ISRrs> {
        EOC_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions flag"]
    #[inline(always)]
    #[must_use]
    pub fn eos(&mut self) -> EOS_W<ISRrs> {
        EOS_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC group regular overrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<ISRrs> {
        OVR_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC group injected end of unitary conversion flag"]
    #[inline(always)]
    #[must_use]
    pub fn jeoc(&mut self) -> JEOC_W<ISRrs> {
        JEOC_W::new(self, 5)
    }
    #[doc = "Bit 6 - ADC group injected end of sequence conversions flag"]
    #[inline(always)]
    #[must_use]
    pub fn jeos(&mut self) -> JEOS_W<ISRrs> {
        JEOS_W::new(self, 6)
    }
    #[doc = "Bit 7 - ADC analog watchdog 1 flag"]
    #[inline(always)]
    #[must_use]
    pub fn awd1(&mut self) -> AWD1_W<ISRrs> {
        AWD1_W::new(self, 7)
    }
    #[doc = "Bit 8 - ADC analog watchdog 2 flag"]
    #[inline(always)]
    #[must_use]
    pub fn awd2(&mut self) -> AWD2_W<ISRrs> {
        AWD2_W::new(self, 8)
    }
    #[doc = "Bit 9 - ADC analog watchdog 3 flag"]
    #[inline(always)]
    #[must_use]
    pub fn awd3(&mut self) -> AWD3_W<ISRrs> {
        AWD3_W::new(self, 9)
    }
    #[doc = "Bit 10 - ADC group injected contexts queue overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn jqovf(&mut self) -> JQOVF_W<ISRrs> {
        JQOVF_W::new(self, 10)
    }
}
#[doc = "ADC interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
