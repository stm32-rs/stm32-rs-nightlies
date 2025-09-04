///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `ADRDYIE` reader - ADC ready interrupt enable
pub type ADRDYIE_R = crate::BitReader;
///Field `ADRDYIE` writer - ADC ready interrupt enable
pub type ADRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOSMPIE` reader - End of sampling flag interrupt enable for regular conversions
pub type EOSMPIE_R = crate::BitReader;
///Field `EOSMPIE` writer - End of sampling flag interrupt enable for regular conversions
pub type EOSMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOCIE` reader - End of regular conversion interrupt enable
pub type EOCIE_R = crate::BitReader;
///Field `EOCIE` writer - End of regular conversion interrupt enable
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOSIE` reader - End of regular sequence of conversions interrupt enable
pub type EOSIE_R = crate::BitReader;
///Field `EOSIE` writer - End of regular sequence of conversions interrupt enable
pub type EOSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRIE` reader - Overrun interrupt enable
pub type OVRIE_R = crate::BitReader;
///Field `OVRIE` writer - Overrun interrupt enable
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JEOCIE` reader - End of injected conversion interrupt enable
pub type JEOCIE_R = crate::BitReader;
///Field `JEOCIE` writer - End of injected conversion interrupt enable
pub type JEOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JEOSIE` reader - End of injected sequence of conversions interrupt enable
pub type JEOSIE_R = crate::BitReader;
///Field `JEOSIE` writer - End of injected sequence of conversions interrupt enable
pub type JEOSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1IE` reader - Analog watchdog 1 interrupt enable
pub type AWD1IE_R = crate::BitReader;
///Field `AWD1IE` writer - Analog watchdog 1 interrupt enable
pub type AWD1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2IE` reader - Analog watchdog 2 interrupt enable
pub type AWD2IE_R = crate::BitReader;
///Field `AWD2IE` writer - Analog watchdog 2 interrupt enable
pub type AWD2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD3IE` reader - Analog watchdog 3 interrupt enable
pub type AWD3IE_R = crate::BitReader;
///Field `AWD3IE` writer - Analog watchdog 3 interrupt enable
pub type AWD3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JQOVFIE` reader - Injected context queue overflow interrupt enable
pub type JQOVFIE_R = crate::BitReader;
///Field `JQOVFIE` writer - Injected context queue overflow interrupt enable
pub type JQOVFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ADC ready interrupt enable
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of sampling flag interrupt enable for regular conversions
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End of regular conversion interrupt enable
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - End of regular sequence of conversions interrupt enable
    #[inline(always)]
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Overrun interrupt enable
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - End of injected conversion interrupt enable
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - End of injected sequence of conversions interrupt enable
    #[inline(always)]
    pub fn jeosie(&self) -> JEOSIE_R {
        JEOSIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog 1 interrupt enable
    #[inline(always)]
    pub fn awd1ie(&self) -> AWD1IE_R {
        AWD1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog 2 interrupt enable
    #[inline(always)]
    pub fn awd2ie(&self) -> AWD2IE_R {
        AWD2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog 3 interrupt enable
    #[inline(always)]
    pub fn awd3ie(&self) -> AWD3IE_R {
        AWD3IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Injected context queue overflow interrupt enable
    #[inline(always)]
    pub fn jqovfie(&self) -> JQOVFIE_R {
        JQOVFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("adrdyie", &self.adrdyie())
            .field("eosmpie", &self.eosmpie())
            .field("eocie", &self.eocie())
            .field("eosie", &self.eosie())
            .field("ovrie", &self.ovrie())
            .field("jeocie", &self.jeocie())
            .field("jeosie", &self.jeosie())
            .field("awd1ie", &self.awd1ie())
            .field("awd2ie", &self.awd2ie())
            .field("awd3ie", &self.awd3ie())
            .field("jqovfie", &self.jqovfie())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC ready interrupt enable
    #[inline(always)]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<IERrs> {
        ADRDYIE_W::new(self, 0)
    }
    ///Bit 1 - End of sampling flag interrupt enable for regular conversions
    #[inline(always)]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<IERrs> {
        EOSMPIE_W::new(self, 1)
    }
    ///Bit 2 - End of regular conversion interrupt enable
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W<IERrs> {
        EOCIE_W::new(self, 2)
    }
    ///Bit 3 - End of regular sequence of conversions interrupt enable
    #[inline(always)]
    pub fn eosie(&mut self) -> EOSIE_W<IERrs> {
        EOSIE_W::new(self, 3)
    }
    ///Bit 4 - Overrun interrupt enable
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W<IERrs> {
        OVRIE_W::new(self, 4)
    }
    ///Bit 5 - End of injected conversion interrupt enable
    #[inline(always)]
    pub fn jeocie(&mut self) -> JEOCIE_W<IERrs> {
        JEOCIE_W::new(self, 5)
    }
    ///Bit 6 - End of injected sequence of conversions interrupt enable
    #[inline(always)]
    pub fn jeosie(&mut self) -> JEOSIE_W<IERrs> {
        JEOSIE_W::new(self, 6)
    }
    ///Bit 7 - Analog watchdog 1 interrupt enable
    #[inline(always)]
    pub fn awd1ie(&mut self) -> AWD1IE_W<IERrs> {
        AWD1IE_W::new(self, 7)
    }
    ///Bit 8 - Analog watchdog 2 interrupt enable
    #[inline(always)]
    pub fn awd2ie(&mut self) -> AWD2IE_W<IERrs> {
        AWD2IE_W::new(self, 8)
    }
    ///Bit 9 - Analog watchdog 3 interrupt enable
    #[inline(always)]
    pub fn awd3ie(&mut self) -> AWD3IE_W<IERrs> {
        AWD3IE_W::new(self, 9)
    }
    ///Bit 10 - Injected context queue overflow interrupt enable
    #[inline(always)]
    pub fn jqovfie(&mut self) -> JQOVFIE_W<IERrs> {
        JQOVFIE_W::new(self, 10)
    }
}
/**ADC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ADC1:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
