///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
///Field `ADRDY` reader - ADC ready
pub type ADRDY_R = crate::BitReader;
///Field `ADRDY` writer - ADC ready
pub type ADRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOSMP` reader - End of sampling flag
pub type EOSMP_R = crate::BitReader;
///Field `EOSMP` writer - End of sampling flag
pub type EOSMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOC` reader - End of conversion flag
pub type EOC_R = crate::BitReader;
///Field `EOC` writer - End of conversion flag
pub type EOC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOS` reader - End of regular sequence flag
pub type EOS_R = crate::BitReader;
///Field `EOS` writer - End of regular sequence flag
pub type EOS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVR` reader - ADC overrun
pub type OVR_R = crate::BitReader;
///Field `OVR` writer - ADC overrun
pub type OVR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JEOC` reader - Injected channel end of conversion flag
pub type JEOC_R = crate::BitReader;
///Field `JEOC` writer - Injected channel end of conversion flag
pub type JEOC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JEOS` reader - Injected channel end of sequence flag
pub type JEOS_R = crate::BitReader;
///Field `JEOS` writer - Injected channel end of sequence flag
pub type JEOS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1` reader - Analog watchdog 1 flag
pub type AWD1_R = crate::BitReader;
///Field `AWD1` writer - Analog watchdog 1 flag
pub type AWD1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2` reader - Analog watchdog 2 flag
pub type AWD2_R = crate::BitReader;
///Field `AWD2` writer - Analog watchdog 2 flag
pub type AWD2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD3` reader - Analog watchdog 3 flag
pub type AWD3_R = crate::BitReader;
///Field `AWD3` writer - Analog watchdog 3 flag
pub type AWD3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JQOVF` reader - Injected context queue overflow
pub type JQOVF_R = crate::BitReader;
///Field `JQOVF` writer - Injected context queue overflow
pub type JQOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ADC ready
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of sampling flag
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End of conversion flag
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - End of regular sequence flag
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC overrun
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Injected channel end of conversion flag
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Injected channel end of sequence flag
    #[inline(always)]
    pub fn jeos(&self) -> JEOS_R {
        JEOS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog 1 flag
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog 2 flag
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog 3 flag
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Injected context queue overflow
    #[inline(always)]
    pub fn jqovf(&self) -> JQOVF_R {
        JQOVF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("adrdy", &self.adrdy())
            .field("eosmp", &self.eosmp())
            .field("eoc", &self.eoc())
            .field("eos", &self.eos())
            .field("ovr", &self.ovr())
            .field("jeoc", &self.jeoc())
            .field("jeos", &self.jeos())
            .field("awd1", &self.awd1())
            .field("awd2", &self.awd2())
            .field("awd3", &self.awd3())
            .field("jqovf", &self.jqovf())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC ready
    #[inline(always)]
    pub fn adrdy(&mut self) -> ADRDY_W<'_, ISRrs> {
        ADRDY_W::new(self, 0)
    }
    ///Bit 1 - End of sampling flag
    #[inline(always)]
    pub fn eosmp(&mut self) -> EOSMP_W<'_, ISRrs> {
        EOSMP_W::new(self, 1)
    }
    ///Bit 2 - End of conversion flag
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W<'_, ISRrs> {
        EOC_W::new(self, 2)
    }
    ///Bit 3 - End of regular sequence flag
    #[inline(always)]
    pub fn eos(&mut self) -> EOS_W<'_, ISRrs> {
        EOS_W::new(self, 3)
    }
    ///Bit 4 - ADC overrun
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W<'_, ISRrs> {
        OVR_W::new(self, 4)
    }
    ///Bit 5 - Injected channel end of conversion flag
    #[inline(always)]
    pub fn jeoc(&mut self) -> JEOC_W<'_, ISRrs> {
        JEOC_W::new(self, 5)
    }
    ///Bit 6 - Injected channel end of sequence flag
    #[inline(always)]
    pub fn jeos(&mut self) -> JEOS_W<'_, ISRrs> {
        JEOS_W::new(self, 6)
    }
    ///Bit 7 - Analog watchdog 1 flag
    #[inline(always)]
    pub fn awd1(&mut self) -> AWD1_W<'_, ISRrs> {
        AWD1_W::new(self, 7)
    }
    ///Bit 8 - Analog watchdog 2 flag
    #[inline(always)]
    pub fn awd2(&mut self) -> AWD2_W<'_, ISRrs> {
        AWD2_W::new(self, 8)
    }
    ///Bit 9 - Analog watchdog 3 flag
    #[inline(always)]
    pub fn awd3(&mut self) -> AWD3_W<'_, ISRrs> {
        AWD3_W::new(self, 9)
    }
    ///Bit 10 - Injected context queue overflow
    #[inline(always)]
    pub fn jqovf(&mut self) -> JQOVF_W<'_, ISRrs> {
        JQOVF_W::new(self, 10)
    }
}
/**ADC interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
