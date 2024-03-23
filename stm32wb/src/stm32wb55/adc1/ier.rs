#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "Field `ADRDYIE` reader - ADC ready interrupt"]
pub type ADRDYIE_R = crate::BitReader;
#[doc = "Field `ADRDYIE` writer - ADC ready interrupt"]
pub type ADRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSMPIE` reader - ADC group regular end of sampling interrupt"]
pub type EOSMPIE_R = crate::BitReader;
#[doc = "Field `EOSMPIE` writer - ADC group regular end of sampling interrupt"]
pub type EOSMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCIE` reader - ADC group regular end of unitary conversion interrupt"]
pub type EOCIE_R = crate::BitReader;
#[doc = "Field `EOCIE` writer - ADC group regular end of unitary conversion interrupt"]
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSIE` reader - ADC group regular end of sequence conversions interrupt"]
pub type EOSIE_R = crate::BitReader;
#[doc = "Field `EOSIE` writer - ADC group regular end of sequence conversions interrupt"]
pub type EOSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIE` reader - ADC group regular overrun interrupt"]
pub type OVRIE_R = crate::BitReader;
#[doc = "Field `OVRIE` writer - ADC group regular overrun interrupt"]
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOCIE` reader - ADC group injected end of unitary conversion interrupt"]
pub type JEOCIE_R = crate::BitReader;
#[doc = "Field `JEOCIE` writer - ADC group injected end of unitary conversion interrupt"]
pub type JEOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOSIE` reader - ADC group injected end of sequence conversions interrupt"]
pub type JEOSIE_R = crate::BitReader;
#[doc = "Field `JEOSIE` writer - ADC group injected end of sequence conversions interrupt"]
pub type JEOSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1IE` reader - ADC analog watchdog 1 interrupt"]
pub type AWD1IE_R = crate::BitReader;
#[doc = "Field `AWD1IE` writer - ADC analog watchdog 1 interrupt"]
pub type AWD1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD2IE` reader - ADC analog watchdog 2 interrupt"]
pub type AWD2IE_R = crate::BitReader;
#[doc = "Field `AWD2IE` writer - ADC analog watchdog 2 interrupt"]
pub type AWD2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD3IE` reader - ADC analog watchdog 3 interrupt"]
pub type AWD3IE_R = crate::BitReader;
#[doc = "Field `AWD3IE` writer - ADC analog watchdog 3 interrupt"]
pub type AWD3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JQOVFIE` reader - ADC group injected contexts queue overflow interrupt"]
pub type JQOVFIE_R = crate::BitReader;
#[doc = "Field `JQOVFIE` writer - ADC group injected contexts queue overflow interrupt"]
pub type JQOVFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC ready interrupt"]
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC group regular end of sampling interrupt"]
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion interrupt"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions interrupt"]
    #[inline(always)]
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC group regular overrun interrupt"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC group injected end of unitary conversion interrupt"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC group injected end of sequence conversions interrupt"]
    #[inline(always)]
    pub fn jeosie(&self) -> JEOSIE_R {
        JEOSIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC analog watchdog 1 interrupt"]
    #[inline(always)]
    pub fn awd1ie(&self) -> AWD1IE_R {
        AWD1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC analog watchdog 2 interrupt"]
    #[inline(always)]
    pub fn awd2ie(&self) -> AWD2IE_R {
        AWD2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC analog watchdog 3 interrupt"]
    #[inline(always)]
    pub fn awd3ie(&self) -> AWD3IE_R {
        AWD3IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC group injected contexts queue overflow interrupt"]
    #[inline(always)]
    pub fn jqovfie(&self) -> JQOVFIE_R {
        JQOVFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ready interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<IERrs> {
        ADRDYIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC group regular end of sampling interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<IERrs> {
        EOSMPIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<IERrs> {
        EOCIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eosie(&mut self) -> EOSIE_W<IERrs> {
        EOSIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC group regular overrun interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<IERrs> {
        OVRIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC group injected end of unitary conversion interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JEOCIE_W<IERrs> {
        JEOCIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - ADC group injected end of sequence conversions interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn jeosie(&mut self) -> JEOSIE_W<IERrs> {
        JEOSIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - ADC analog watchdog 1 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn awd1ie(&mut self) -> AWD1IE_W<IERrs> {
        AWD1IE_W::new(self, 7)
    }
    #[doc = "Bit 8 - ADC analog watchdog 2 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ie(&mut self) -> AWD2IE_W<IERrs> {
        AWD2IE_W::new(self, 8)
    }
    #[doc = "Bit 9 - ADC analog watchdog 3 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ie(&mut self) -> AWD3IE_W<IERrs> {
        AWD3IE_W::new(self, 9)
    }
    #[doc = "Bit 10 - ADC group injected contexts queue overflow interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn jqovfie(&mut self) -> JQOVFIE_W<IERrs> {
        JQOVFIE_W::new(self, 10)
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
