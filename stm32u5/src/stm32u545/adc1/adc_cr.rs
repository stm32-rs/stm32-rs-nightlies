#[doc = "Register `ADC_CR` reader"]
pub type R = crate::R<ADC_CRrs>;
#[doc = "Register `ADC_CR` writer"]
pub type W = crate::W<ADC_CRrs>;
#[doc = "Field `ADEN` reader - ADEN"]
pub type ADEN_R = crate::BitReader;
#[doc = "Field `ADDIS` reader - ADDIS"]
pub type ADDIS_R = crate::BitReader;
#[doc = "Field `ADSTART` reader - ADSTART"]
pub type ADSTART_R = crate::BitReader;
#[doc = "Field `JADSTART` reader - JADSTART"]
pub type JADSTART_R = crate::BitReader;
#[doc = "Field `ADSTP` reader - ADSTP"]
pub type ADSTP_R = crate::BitReader;
#[doc = "Field `JADSTP` reader - JADSTP"]
pub type JADSTP_R = crate::BitReader;
#[doc = "Field `ADCALLIN` reader - ADCALLIN"]
pub type ADCALLIN_R = crate::BitReader;
#[doc = "Field `ADCALLIN` writer - ADCALLIN"]
pub type ADCALLIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALINDEX` reader - CALINDEX"]
pub type CALINDEX_R = crate::FieldReader;
#[doc = "Field `CALINDEX` writer - CALINDEX"]
pub type CALINDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADVREGEN` reader - ADVREGEN"]
pub type ADVREGEN_R = crate::BitReader;
#[doc = "Field `ADVREGEN` writer - ADVREGEN"]
pub type ADVREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEEPPWD` reader - DEEPPWD"]
pub type DEEPPWD_R = crate::BitReader;
#[doc = "Field `DEEPPWD` writer - DEEPPWD"]
pub type DEEPPWD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCAL` reader - ADCAL"]
pub type ADCAL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ADEN"]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADDIS"]
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADSTART"]
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JADSTART"]
    #[inline(always)]
    pub fn jadstart(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADSTP"]
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JADSTP"]
    #[inline(always)]
    pub fn jadstp(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - ADCALLIN"]
    #[inline(always)]
    pub fn adcallin(&self) -> ADCALLIN_R {
        ADCALLIN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:27 - CALINDEX"]
    #[inline(always)]
    pub fn calindex(&self) -> CALINDEX_R {
        CALINDEX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - ADVREGEN"]
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DEEPPWD"]
    #[inline(always)]
    pub fn deeppwd(&self) -> DEEPPWD_R {
        DEEPPWD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - ADCAL"]
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - ADCALLIN"]
    #[inline(always)]
    #[must_use]
    pub fn adcallin(&mut self) -> ADCALLIN_W<ADC_CRrs> {
        ADCALLIN_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - CALINDEX"]
    #[inline(always)]
    #[must_use]
    pub fn calindex(&mut self) -> CALINDEX_W<ADC_CRrs> {
        CALINDEX_W::new(self, 24)
    }
    #[doc = "Bit 28 - ADVREGEN"]
    #[inline(always)]
    #[must_use]
    pub fn advregen(&mut self) -> ADVREGEN_W<ADC_CRrs> {
        ADVREGEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - DEEPPWD"]
    #[inline(always)]
    #[must_use]
    pub fn deeppwd(&mut self) -> DEEPPWD_W<ADC_CRrs> {
        DEEPPWD_W::new(self, 29)
    }
}
#[doc = "ADC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_CRrs;
impl crate::RegisterSpec for ADC_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_cr::R`](R) reader structure"]
impl crate::Readable for ADC_CRrs {}
#[doc = "`write(|w| ..)` method takes [`adc_cr::W`](W) writer structure"]
impl crate::Writable for ADC_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CR to value 0x2000_0000"]
impl crate::Resettable for ADC_CRrs {
    const RESET_VALUE: u32 = 0x2000_0000;
}
