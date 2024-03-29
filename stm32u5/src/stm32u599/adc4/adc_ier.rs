#[doc = "Register `ADC_IER` reader"]
pub type R = crate::R<ADC_IERrs>;
#[doc = "Register `ADC_IER` writer"]
pub type W = crate::W<ADC_IERrs>;
#[doc = "Field `ADRDYIE` reader - ADRDYIE"]
pub type ADRDYIE_R = crate::BitReader;
#[doc = "Field `ADRDYIE` writer - ADRDYIE"]
pub type ADRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSMPIE` reader - EOSMPIE"]
pub type EOSMPIE_R = crate::BitReader;
#[doc = "Field `EOSMPIE` writer - EOSMPIE"]
pub type EOSMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCIE` reader - EOCIE"]
pub type EOCIE_R = crate::BitReader;
#[doc = "Field `EOCIE` writer - EOCIE"]
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSIE` reader - EOSIE"]
pub type EOSIE_R = crate::BitReader;
#[doc = "Field `EOSIE` writer - EOSIE"]
pub type EOSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIE` reader - OVRIE"]
pub type OVRIE_R = crate::BitReader;
#[doc = "Field `OVRIE` writer - OVRIE"]
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1IE` reader - AWD1IE"]
pub type AWD1IE_R = crate::BitReader;
#[doc = "Field `AWD1IE` writer - AWD1IE"]
pub type AWD1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD2IE` reader - AWD2IE"]
pub type AWD2IE_R = crate::BitReader;
#[doc = "Field `AWD2IE` writer - AWD2IE"]
pub type AWD2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD3IE` reader - AWD3IE"]
pub type AWD3IE_R = crate::BitReader;
#[doc = "Field `AWD3IE` writer - AWD3IE"]
pub type AWD3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCALIE` reader - EOCALIE"]
pub type EOCALIE_R = crate::BitReader;
#[doc = "Field `EOCALIE` writer - EOCALIE"]
pub type EOCALIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDORDYIE` reader - LDORDYIE"]
pub type LDORDYIE_R = crate::BitReader;
#[doc = "Field `LDORDYIE` writer - LDORDYIE"]
pub type LDORDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADRDYIE"]
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EOSMPIE"]
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EOCIE"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOSIE"]
    #[inline(always)]
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OVRIE"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - AWD1IE"]
    #[inline(always)]
    pub fn awd1ie(&self) -> AWD1IE_R {
        AWD1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AWD2IE"]
    #[inline(always)]
    pub fn awd2ie(&self) -> AWD2IE_R {
        AWD2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AWD3IE"]
    #[inline(always)]
    pub fn awd3ie(&self) -> AWD3IE_R {
        AWD3IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - EOCALIE"]
    #[inline(always)]
    pub fn eocalie(&self) -> EOCALIE_R {
        EOCALIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LDORDYIE"]
    #[inline(always)]
    pub fn ldordyie(&self) -> LDORDYIE_R {
        LDORDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADRDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<ADC_IERrs> {
        ADRDYIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - EOSMPIE"]
    #[inline(always)]
    #[must_use]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<ADC_IERrs> {
        EOSMPIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - EOCIE"]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<ADC_IERrs> {
        EOCIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - EOSIE"]
    #[inline(always)]
    #[must_use]
    pub fn eosie(&mut self) -> EOSIE_W<ADC_IERrs> {
        EOSIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - OVRIE"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<ADC_IERrs> {
        OVRIE_W::new(self, 4)
    }
    #[doc = "Bit 7 - AWD1IE"]
    #[inline(always)]
    #[must_use]
    pub fn awd1ie(&mut self) -> AWD1IE_W<ADC_IERrs> {
        AWD1IE_W::new(self, 7)
    }
    #[doc = "Bit 8 - AWD2IE"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ie(&mut self) -> AWD2IE_W<ADC_IERrs> {
        AWD2IE_W::new(self, 8)
    }
    #[doc = "Bit 9 - AWD3IE"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ie(&mut self) -> AWD3IE_W<ADC_IERrs> {
        AWD3IE_W::new(self, 9)
    }
    #[doc = "Bit 11 - EOCALIE"]
    #[inline(always)]
    #[must_use]
    pub fn eocalie(&mut self) -> EOCALIE_W<ADC_IERrs> {
        EOCALIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - LDORDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn ldordyie(&mut self) -> LDORDYIE_W<ADC_IERrs> {
        LDORDYIE_W::new(self, 12)
    }
}
#[doc = "ADC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_IERrs;
impl crate::RegisterSpec for ADC_IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_ier::R`](R) reader structure"]
impl crate::Readable for ADC_IERrs {}
#[doc = "`write(|w| ..)` method takes [`adc_ier::W`](W) writer structure"]
impl crate::Writable for ADC_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_IER to value 0"]
impl crate::Resettable for ADC_IERrs {
    const RESET_VALUE: u32 = 0;
}
