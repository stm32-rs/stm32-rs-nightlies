///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `ADRDYIE` reader - ADC ready interrupt
pub type ADRDYIE_R = crate::BitReader;
///Field `ADRDYIE` writer - ADC ready interrupt
pub type ADRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOSMPIE` reader - ADC group regular end of sampling interrupt
pub type EOSMPIE_R = crate::BitReader;
///Field `EOSMPIE` writer - ADC group regular end of sampling interrupt
pub type EOSMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOCIE` reader - ADC group regular end of unitary conversion interrupt
pub type EOCIE_R = crate::BitReader;
///Field `EOCIE` writer - ADC group regular end of unitary conversion interrupt
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOSIE` reader - ADC group regular end of sequence conversions interrupt
pub type EOSIE_R = crate::BitReader;
///Field `EOSIE` writer - ADC group regular end of sequence conversions interrupt
pub type EOSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRIE` reader - ADC group regular overrun interrupt
pub type OVRIE_R = crate::BitReader;
///Field `OVRIE` writer - ADC group regular overrun interrupt
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JEOCIE` reader - ADC group injected end of unitary conversion interrupt
pub type JEOCIE_R = crate::BitReader;
///Field `JEOCIE` writer - ADC group injected end of unitary conversion interrupt
pub type JEOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JEOSIE` reader - ADC group injected end of sequence conversions interrupt
pub type JEOSIE_R = crate::BitReader;
///Field `JEOSIE` writer - ADC group injected end of sequence conversions interrupt
pub type JEOSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1IE` reader - ADC analog watchdog 1 interrupt
pub type AWD1IE_R = crate::BitReader;
///Field `AWD1IE` writer - ADC analog watchdog 1 interrupt
pub type AWD1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2IE` reader - ADC analog watchdog 2 interrupt
pub type AWD2IE_R = crate::BitReader;
///Field `AWD2IE` writer - ADC analog watchdog 2 interrupt
pub type AWD2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD3IE` reader - ADC analog watchdog 3 interrupt
pub type AWD3IE_R = crate::BitReader;
///Field `AWD3IE` writer - ADC analog watchdog 3 interrupt
pub type AWD3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JQOVFIE` reader - ADC group injected contexts queue overflow interrupt
pub type JQOVFIE_R = crate::BitReader;
///Field `JQOVFIE` writer - ADC group injected contexts queue overflow interrupt
pub type JQOVFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ADC ready interrupt
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADC group regular end of sampling interrupt
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADC group regular end of unitary conversion interrupt
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ADC group regular end of sequence conversions interrupt
    #[inline(always)]
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC group regular overrun interrupt
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADC group injected end of unitary conversion interrupt
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ADC group injected end of sequence conversions interrupt
    #[inline(always)]
    pub fn jeosie(&self) -> JEOSIE_R {
        JEOSIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ADC analog watchdog 1 interrupt
    #[inline(always)]
    pub fn awd1ie(&self) -> AWD1IE_R {
        AWD1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ADC analog watchdog 2 interrupt
    #[inline(always)]
    pub fn awd2ie(&self) -> AWD2IE_R {
        AWD2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ADC analog watchdog 3 interrupt
    #[inline(always)]
    pub fn awd3ie(&self) -> AWD3IE_R {
        AWD3IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADC group injected contexts queue overflow interrupt
    #[inline(always)]
    pub fn jqovfie(&self) -> JQOVFIE_R {
        JQOVFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("jqovfie", &self.jqovfie())
            .field("awd3ie", &self.awd3ie())
            .field("awd2ie", &self.awd2ie())
            .field("awd1ie", &self.awd1ie())
            .field("jeosie", &self.jeosie())
            .field("jeocie", &self.jeocie())
            .field("ovrie", &self.ovrie())
            .field("eosie", &self.eosie())
            .field("eocie", &self.eocie())
            .field("eosmpie", &self.eosmpie())
            .field("adrdyie", &self.adrdyie())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC ready interrupt
    #[inline(always)]
    #[must_use]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<IERrs> {
        ADRDYIE_W::new(self, 0)
    }
    ///Bit 1 - ADC group regular end of sampling interrupt
    #[inline(always)]
    #[must_use]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<IERrs> {
        EOSMPIE_W::new(self, 1)
    }
    ///Bit 2 - ADC group regular end of unitary conversion interrupt
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<IERrs> {
        EOCIE_W::new(self, 2)
    }
    ///Bit 3 - ADC group regular end of sequence conversions interrupt
    #[inline(always)]
    #[must_use]
    pub fn eosie(&mut self) -> EOSIE_W<IERrs> {
        EOSIE_W::new(self, 3)
    }
    ///Bit 4 - ADC group regular overrun interrupt
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<IERrs> {
        OVRIE_W::new(self, 4)
    }
    ///Bit 5 - ADC group injected end of unitary conversion interrupt
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JEOCIE_W<IERrs> {
        JEOCIE_W::new(self, 5)
    }
    ///Bit 6 - ADC group injected end of sequence conversions interrupt
    #[inline(always)]
    #[must_use]
    pub fn jeosie(&mut self) -> JEOSIE_W<IERrs> {
        JEOSIE_W::new(self, 6)
    }
    ///Bit 7 - ADC analog watchdog 1 interrupt
    #[inline(always)]
    #[must_use]
    pub fn awd1ie(&mut self) -> AWD1IE_W<IERrs> {
        AWD1IE_W::new(self, 7)
    }
    ///Bit 8 - ADC analog watchdog 2 interrupt
    #[inline(always)]
    #[must_use]
    pub fn awd2ie(&mut self) -> AWD2IE_W<IERrs> {
        AWD2IE_W::new(self, 8)
    }
    ///Bit 9 - ADC analog watchdog 3 interrupt
    #[inline(always)]
    #[must_use]
    pub fn awd3ie(&mut self) -> AWD3IE_W<IERrs> {
        AWD3IE_W::new(self, 9)
    }
    ///Bit 10 - ADC group injected contexts queue overflow interrupt
    #[inline(always)]
    #[must_use]
    pub fn jqovfie(&mut self) -> JQOVFIE_W<IERrs> {
        JQOVFIE_W::new(self, 10)
    }
}
/**ADC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#ADC1:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
