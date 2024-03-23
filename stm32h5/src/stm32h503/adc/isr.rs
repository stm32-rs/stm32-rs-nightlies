#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<ISRrs>;
#[doc = "Field `ADRDY` reader - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
pub type ADRDY_R = crate::BitReader;
#[doc = "Field `ADRDY` writer - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
pub type ADRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSMP` reader - End of sampling flag This bit is set by hardware during the conversion of any channel (only for regular channels), at the end of the sampling phase."]
pub type EOSMP_R = crate::BitReader;
#[doc = "Field `EOSMP` writer - End of sampling flag This bit is set by hardware during the conversion of any channel (only for regular channels), at the end of the sampling phase."]
pub type EOSMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC` reader - End of conversion flag This bit is set by hardware at the end of each regular conversion of a channel when a new data is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register"]
pub type EOC_R = crate::BitReader;
#[doc = "Field `EOC` writer - End of conversion flag This bit is set by hardware at the end of each regular conversion of a channel when a new data is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register"]
pub type EOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOS` reader - End of regular sequence flag This bit is set by hardware at the end of the conversions of a regular sequence of channels. It is cleared by software writing 1 to it."]
pub type EOS_R = crate::BitReader;
#[doc = "Field `EOS` writer - End of regular sequence flag This bit is set by hardware at the end of the conversions of a regular sequence of channels. It is cleared by software writing 1 to it."]
pub type EOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR` reader - ADC overrun This bit is set by hardware when an overrun occurs on a regular channel, meaning that a new conversion has completed while the EOC flag was already set. It is cleared by software writing 1 to it."]
pub type OVR_R = crate::BitReader;
#[doc = "Field `OVR` writer - ADC overrun This bit is set by hardware when an overrun occurs on a regular channel, meaning that a new conversion has completed while the EOC flag was already set. It is cleared by software writing 1 to it."]
pub type OVR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOC` reader - Injected channel end of conversion flag This bit is set by hardware at the end of each injected conversion of a channel when a new data is available in the corresponding ADC_JDRy register. It is cleared by software writing 1 to it or by reading the corresponding ADC_JDRy register"]
pub type JEOC_R = crate::BitReader;
#[doc = "Field `JEOC` writer - Injected channel end of conversion flag This bit is set by hardware at the end of each injected conversion of a channel when a new data is available in the corresponding ADC_JDRy register. It is cleared by software writing 1 to it or by reading the corresponding ADC_JDRy register"]
pub type JEOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOS` reader - Injected channel end of sequence flag This bit is set by hardware at the end of the conversions of all injected channels in the group. It is cleared by software writing 1 to it."]
pub type JEOS_R = crate::BitReader;
#[doc = "Field `JEOS` writer - Injected channel end of sequence flag This bit is set by hardware at the end of the conversions of all injected channels in the group. It is cleared by software writing 1 to it."]
pub type JEOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1` reader - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT1\\[11:0\\]
and HT1\\[11:0\\]
of ADC_TR1 register. It is cleared by software. writing 1 to it."]
pub type AWD1_R = crate::BitReader;
#[doc = "Field `AWD1` writer - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT1\\[11:0\\]
and HT1\\[11:0\\]
of ADC_TR1 register. It is cleared by software. writing 1 to it."]
pub type AWD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD2` reader - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT2\\[7:0\\]
and HT2\\[7:0\\]
of ADC_TR2 register. It is cleared by software writing 1 to it."]
pub type AWD2_R = crate::BitReader;
#[doc = "Field `AWD2` writer - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT2\\[7:0\\]
and HT2\\[7:0\\]
of ADC_TR2 register. It is cleared by software writing 1 to it."]
pub type AWD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD3` reader - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT3\\[7:0\\]
and HT3\\[7:0\\]
of ADC_TR3 register. It is cleared by software writing 1 to it."]
pub type AWD3_R = crate::BitReader;
#[doc = "Field `AWD3` writer - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT3\\[7:0\\]
and HT3\\[7:0\\]
of ADC_TR3 register. It is cleared by software writing 1 to it."]
pub type AWD3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JQOVF` reader - Injected context queue overflow This bit is set by hardware when an Overflow of the Injected Queue of Context occurs. It is cleared by software writing 1 to it. Refer to for more information."]
pub type JQOVF_R = crate::BitReader;
#[doc = "Field `JQOVF` writer - Injected context queue overflow This bit is set by hardware when an Overflow of the Injected Queue of Context occurs. It is cleared by software writing 1 to it. Refer to for more information."]
pub type JQOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of sampling flag This bit is set by hardware during the conversion of any channel (only for regular channels), at the end of the sampling phase."]
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of conversion flag This bit is set by hardware at the end of each regular conversion of a channel when a new data is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of regular sequence flag This bit is set by hardware at the end of the conversions of a regular sequence of channels. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC overrun This bit is set by hardware when an overrun occurs on a regular channel, meaning that a new conversion has completed while the EOC flag was already set. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Injected channel end of conversion flag This bit is set by hardware at the end of each injected conversion of a channel when a new data is available in the corresponding ADC_JDRy register. It is cleared by software writing 1 to it or by reading the corresponding ADC_JDRy register"]
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Injected channel end of sequence flag This bit is set by hardware at the end of the conversions of all injected channels in the group. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn jeos(&self) -> JEOS_R {
        JEOS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT1\\[11:0\\]
and HT1\\[11:0\\]
of ADC_TR1 register. It is cleared by software. writing 1 to it."]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT2\\[7:0\\]
and HT2\\[7:0\\]
of ADC_TR2 register. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT3\\[7:0\\]
and HT3\\[7:0\\]
of ADC_TR3 register. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Injected context queue overflow This bit is set by hardware when an Overflow of the Injected Queue of Context occurs. It is cleared by software writing 1 to it. Refer to for more information."]
    #[inline(always)]
    pub fn jqovf(&self) -> JQOVF_R {
        JQOVF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn adrdy(&mut self) -> ADRDY_W<ISRrs> {
        ADRDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - End of sampling flag This bit is set by hardware during the conversion of any channel (only for regular channels), at the end of the sampling phase."]
    #[inline(always)]
    #[must_use]
    pub fn eosmp(&mut self) -> EOSMP_W<ISRrs> {
        EOSMP_W::new(self, 1)
    }
    #[doc = "Bit 2 - End of conversion flag This bit is set by hardware at the end of each regular conversion of a channel when a new data is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<ISRrs> {
        EOC_W::new(self, 2)
    }
    #[doc = "Bit 3 - End of regular sequence flag This bit is set by hardware at the end of the conversions of a regular sequence of channels. It is cleared by software writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn eos(&mut self) -> EOS_W<ISRrs> {
        EOS_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC overrun This bit is set by hardware when an overrun occurs on a regular channel, meaning that a new conversion has completed while the EOC flag was already set. It is cleared by software writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<ISRrs> {
        OVR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Injected channel end of conversion flag This bit is set by hardware at the end of each injected conversion of a channel when a new data is available in the corresponding ADC_JDRy register. It is cleared by software writing 1 to it or by reading the corresponding ADC_JDRy register"]
    #[inline(always)]
    #[must_use]
    pub fn jeoc(&mut self) -> JEOC_W<ISRrs> {
        JEOC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Injected channel end of sequence flag This bit is set by hardware at the end of the conversions of all injected channels in the group. It is cleared by software writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn jeos(&mut self) -> JEOS_W<ISRrs> {
        JEOS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT1\\[11:0\\]
and HT1\\[11:0\\]
of ADC_TR1 register. It is cleared by software. writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn awd1(&mut self) -> AWD1_W<ISRrs> {
        AWD1_W::new(self, 7)
    }
    #[doc = "Bit 8 - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT2\\[7:0\\]
and HT2\\[7:0\\]
of ADC_TR2 register. It is cleared by software writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn awd2(&mut self) -> AWD2_W<ISRrs> {
        AWD2_W::new(self, 8)
    }
    #[doc = "Bit 9 - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT3\\[7:0\\]
and HT3\\[7:0\\]
of ADC_TR3 register. It is cleared by software writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn awd3(&mut self) -> AWD3_W<ISRrs> {
        AWD3_W::new(self, 9)
    }
    #[doc = "Bit 10 - Injected context queue overflow This bit is set by hardware when an Overflow of the Injected Queue of Context occurs. It is cleared by software writing 1 to it. Refer to for more information."]
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
