///Register `ADC_IER` reader
pub type R = crate::R<ADC_IERrs>;
///Register `ADC_IER` writer
pub type W = crate::W<ADC_IERrs>;
///Field `ADRDYIE` reader - ADRDYIE
pub type ADRDYIE_R = crate::BitReader;
///Field `ADRDYIE` writer - ADRDYIE
pub type ADRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOSMPIE` reader - EOSMPIE
pub type EOSMPIE_R = crate::BitReader;
///Field `EOSMPIE` writer - EOSMPIE
pub type EOSMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOCIE` reader - EOCIE
pub type EOCIE_R = crate::BitReader;
///Field `EOCIE` writer - EOCIE
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOSIE` reader - EOSIE
pub type EOSIE_R = crate::BitReader;
///Field `EOSIE` writer - EOSIE
pub type EOSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRIE` reader - OVRIE
pub type OVRIE_R = crate::BitReader;
///Field `OVRIE` writer - OVRIE
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1IE` reader - AWD1IE
pub type AWD1IE_R = crate::BitReader;
///Field `AWD1IE` writer - AWD1IE
pub type AWD1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2IE` reader - AWD2IE
pub type AWD2IE_R = crate::BitReader;
///Field `AWD2IE` writer - AWD2IE
pub type AWD2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD3IE` reader - AWD3IE
pub type AWD3IE_R = crate::BitReader;
///Field `AWD3IE` writer - AWD3IE
pub type AWD3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOCALIE` reader - EOCALIE
pub type EOCALIE_R = crate::BitReader;
///Field `EOCALIE` writer - EOCALIE
pub type EOCALIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LDORDYIE` reader - LDORDYIE
pub type LDORDYIE_R = crate::BitReader;
///Field `LDORDYIE` writer - LDORDYIE
pub type LDORDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ADRDYIE
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EOSMPIE
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - EOCIE
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EOSIE
    #[inline(always)]
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OVRIE
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - AWD1IE
    #[inline(always)]
    pub fn awd1ie(&self) -> AWD1IE_R {
        AWD1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AWD2IE
    #[inline(always)]
    pub fn awd2ie(&self) -> AWD2IE_R {
        AWD2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AWD3IE
    #[inline(always)]
    pub fn awd3ie(&self) -> AWD3IE_R {
        AWD3IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - EOCALIE
    #[inline(always)]
    pub fn eocalie(&self) -> EOCALIE_R {
        EOCALIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LDORDYIE
    #[inline(always)]
    pub fn ldordyie(&self) -> LDORDYIE_R {
        LDORDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_IER")
            .field("ldordyie", &self.ldordyie())
            .field("eocalie", &self.eocalie())
            .field("awd3ie", &self.awd3ie())
            .field("awd2ie", &self.awd2ie())
            .field("awd1ie", &self.awd1ie())
            .field("ovrie", &self.ovrie())
            .field("eosie", &self.eosie())
            .field("eocie", &self.eocie())
            .field("eosmpie", &self.eosmpie())
            .field("adrdyie", &self.adrdyie())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADRDYIE
    #[inline(always)]
    #[must_use]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<ADC_IERrs> {
        ADRDYIE_W::new(self, 0)
    }
    ///Bit 1 - EOSMPIE
    #[inline(always)]
    #[must_use]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<ADC_IERrs> {
        EOSMPIE_W::new(self, 1)
    }
    ///Bit 2 - EOCIE
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<ADC_IERrs> {
        EOCIE_W::new(self, 2)
    }
    ///Bit 3 - EOSIE
    #[inline(always)]
    #[must_use]
    pub fn eosie(&mut self) -> EOSIE_W<ADC_IERrs> {
        EOSIE_W::new(self, 3)
    }
    ///Bit 4 - OVRIE
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<ADC_IERrs> {
        OVRIE_W::new(self, 4)
    }
    ///Bit 7 - AWD1IE
    #[inline(always)]
    #[must_use]
    pub fn awd1ie(&mut self) -> AWD1IE_W<ADC_IERrs> {
        AWD1IE_W::new(self, 7)
    }
    ///Bit 8 - AWD2IE
    #[inline(always)]
    #[must_use]
    pub fn awd2ie(&mut self) -> AWD2IE_W<ADC_IERrs> {
        AWD2IE_W::new(self, 8)
    }
    ///Bit 9 - AWD3IE
    #[inline(always)]
    #[must_use]
    pub fn awd3ie(&mut self) -> AWD3IE_W<ADC_IERrs> {
        AWD3IE_W::new(self, 9)
    }
    ///Bit 11 - EOCALIE
    #[inline(always)]
    #[must_use]
    pub fn eocalie(&mut self) -> EOCALIE_W<ADC_IERrs> {
        EOCALIE_W::new(self, 11)
    }
    ///Bit 12 - LDORDYIE
    #[inline(always)]
    #[must_use]
    pub fn ldordyie(&mut self) -> LDORDYIE_W<ADC_IERrs> {
        LDORDYIE_W::new(self, 12)
    }
}
/**ADC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`adc_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:ADC_IER)*/
pub struct ADC_IERrs;
impl crate::RegisterSpec for ADC_IERrs {
    type Ux = u32;
}
///`read()` method returns [`adc_ier::R`](R) reader structure
impl crate::Readable for ADC_IERrs {}
///`write(|w| ..)` method takes [`adc_ier::W`](W) writer structure
impl crate::Writable for ADC_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_IER to value 0
impl crate::Resettable for ADC_IERrs {
    const RESET_VALUE: u32 = 0;
}
