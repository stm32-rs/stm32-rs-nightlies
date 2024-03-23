#[doc = "Register `ADC_CR` reader"]
pub type R = crate::R<ADC_CRrs>;
#[doc = "Register `ADC_CR` writer"]
pub type W = crate::W<ADC_CRrs>;
#[doc = "Field `ADEN` reader - ADEN"]
pub type ADEN_R = crate::BitReader;
#[doc = "Field `ADEN` writer - ADEN"]
pub type ADEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDIS` reader - ADDIS"]
pub type ADDIS_R = crate::BitReader;
#[doc = "Field `ADDIS` writer - ADDIS"]
pub type ADDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTART` reader - ADSTART"]
pub type ADSTART_R = crate::BitReader;
#[doc = "Field `ADSTART` writer - ADSTART"]
pub type ADSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JADSTART` reader - JADSTART"]
pub type JADSTART_R = crate::BitReader;
#[doc = "Field `JADSTART` writer - JADSTART"]
pub type JADSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTP` reader - ADSTP"]
pub type ADSTP_R = crate::BitReader;
#[doc = "Field `ADSTP` writer - ADSTP"]
pub type ADSTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JADSTP` reader - JADSTP"]
pub type JADSTP_R = crate::BitReader;
#[doc = "Field `JADSTP` writer - JADSTP"]
pub type JADSTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOST` reader - BOOST"]
pub type BOOST_R = crate::BitReader;
#[doc = "Field `BOOST` writer - BOOST"]
pub type BOOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCALLIN` reader - ADCALLIN"]
pub type ADCALLIN_R = crate::BitReader;
#[doc = "Field `ADCALLIN` writer - ADCALLIN"]
pub type ADCALLIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINCALRDYW1` reader - LINCALRDYW1"]
pub type LINCALRDYW1_R = crate::BitReader;
#[doc = "Field `LINCALRDYW1` writer - LINCALRDYW1"]
pub type LINCALRDYW1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINCALRDYW2` reader - LINCALRDYW2"]
pub type LINCALRDYW2_R = crate::BitReader;
#[doc = "Field `LINCALRDYW2` writer - LINCALRDYW2"]
pub type LINCALRDYW2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINCALRDYW3` reader - LINCALRDYW3"]
pub type LINCALRDYW3_R = crate::BitReader;
#[doc = "Field `LINCALRDYW3` writer - LINCALRDYW3"]
pub type LINCALRDYW3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINCALRDYW4` reader - LINCALRDYW4"]
pub type LINCALRDYW4_R = crate::BitReader;
#[doc = "Field `LINCALRDYW4` writer - LINCALRDYW4"]
pub type LINCALRDYW4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINCALRDYW5` reader - LINCALRDYW5"]
pub type LINCALRDYW5_R = crate::BitReader;
#[doc = "Field `LINCALRDYW5` writer - LINCALRDYW5"]
pub type LINCALRDYW5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINCALRDYW6` reader - LINCALRDYW6"]
pub type LINCALRDYW6_R = crate::BitReader;
#[doc = "Field `LINCALRDYW6` writer - LINCALRDYW6"]
pub type LINCALRDYW6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADVREGEN` reader - ADVREGEN"]
pub type ADVREGEN_R = crate::BitReader;
#[doc = "Field `ADVREGEN` writer - ADVREGEN"]
pub type ADVREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEEPPWD` reader - DEEPPWD"]
pub type DEEPPWD_R = crate::BitReader;
#[doc = "Field `DEEPPWD` writer - DEEPPWD"]
pub type DEEPPWD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCALDIF` reader - ADCALDIF"]
pub type ADCALDIF_R = crate::BitReader;
#[doc = "Field `ADCALDIF` writer - ADCALDIF"]
pub type ADCALDIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCAL` reader - ADCAL"]
pub type ADCAL_R = crate::BitReader;
#[doc = "Field `ADCAL` writer - ADCAL"]
pub type ADCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 8 - BOOST"]
    #[inline(always)]
    pub fn boost(&self) -> BOOST_R {
        BOOST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - ADCALLIN"]
    #[inline(always)]
    pub fn adcallin(&self) -> ADCALLIN_R {
        ADCALLIN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 22 - LINCALRDYW1"]
    #[inline(always)]
    pub fn lincalrdyw1(&self) -> LINCALRDYW1_R {
        LINCALRDYW1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LINCALRDYW2"]
    #[inline(always)]
    pub fn lincalrdyw2(&self) -> LINCALRDYW2_R {
        LINCALRDYW2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - LINCALRDYW3"]
    #[inline(always)]
    pub fn lincalrdyw3(&self) -> LINCALRDYW3_R {
        LINCALRDYW3_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LINCALRDYW4"]
    #[inline(always)]
    pub fn lincalrdyw4(&self) -> LINCALRDYW4_R {
        LINCALRDYW4_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LINCALRDYW5"]
    #[inline(always)]
    pub fn lincalrdyw5(&self) -> LINCALRDYW5_R {
        LINCALRDYW5_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LINCALRDYW6"]
    #[inline(always)]
    pub fn lincalrdyw6(&self) -> LINCALRDYW6_R {
        LINCALRDYW6_R::new(((self.bits >> 27) & 1) != 0)
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
    #[doc = "Bit 30 - ADCALDIF"]
    #[inline(always)]
    pub fn adcaldif(&self) -> ADCALDIF_R {
        ADCALDIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ADCAL"]
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADEN"]
    #[inline(always)]
    #[must_use]
    pub fn aden(&mut self) -> ADEN_W<ADC_CRrs> {
        ADEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADDIS"]
    #[inline(always)]
    #[must_use]
    pub fn addis(&mut self) -> ADDIS_W<ADC_CRrs> {
        ADDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADSTART"]
    #[inline(always)]
    #[must_use]
    pub fn adstart(&mut self) -> ADSTART_W<ADC_CRrs> {
        ADSTART_W::new(self, 2)
    }
    #[doc = "Bit 3 - JADSTART"]
    #[inline(always)]
    #[must_use]
    pub fn jadstart(&mut self) -> JADSTART_W<ADC_CRrs> {
        JADSTART_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADSTP"]
    #[inline(always)]
    #[must_use]
    pub fn adstp(&mut self) -> ADSTP_W<ADC_CRrs> {
        ADSTP_W::new(self, 4)
    }
    #[doc = "Bit 5 - JADSTP"]
    #[inline(always)]
    #[must_use]
    pub fn jadstp(&mut self) -> JADSTP_W<ADC_CRrs> {
        JADSTP_W::new(self, 5)
    }
    #[doc = "Bit 8 - BOOST"]
    #[inline(always)]
    #[must_use]
    pub fn boost(&mut self) -> BOOST_W<ADC_CRrs> {
        BOOST_W::new(self, 8)
    }
    #[doc = "Bit 16 - ADCALLIN"]
    #[inline(always)]
    #[must_use]
    pub fn adcallin(&mut self) -> ADCALLIN_W<ADC_CRrs> {
        ADCALLIN_W::new(self, 16)
    }
    #[doc = "Bit 22 - LINCALRDYW1"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw1(&mut self) -> LINCALRDYW1_W<ADC_CRrs> {
        LINCALRDYW1_W::new(self, 22)
    }
    #[doc = "Bit 23 - LINCALRDYW2"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw2(&mut self) -> LINCALRDYW2_W<ADC_CRrs> {
        LINCALRDYW2_W::new(self, 23)
    }
    #[doc = "Bit 24 - LINCALRDYW3"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw3(&mut self) -> LINCALRDYW3_W<ADC_CRrs> {
        LINCALRDYW3_W::new(self, 24)
    }
    #[doc = "Bit 25 - LINCALRDYW4"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw4(&mut self) -> LINCALRDYW4_W<ADC_CRrs> {
        LINCALRDYW4_W::new(self, 25)
    }
    #[doc = "Bit 26 - LINCALRDYW5"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw5(&mut self) -> LINCALRDYW5_W<ADC_CRrs> {
        LINCALRDYW5_W::new(self, 26)
    }
    #[doc = "Bit 27 - LINCALRDYW6"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw6(&mut self) -> LINCALRDYW6_W<ADC_CRrs> {
        LINCALRDYW6_W::new(self, 27)
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
    #[doc = "Bit 30 - ADCALDIF"]
    #[inline(always)]
    #[must_use]
    pub fn adcaldif(&mut self) -> ADCALDIF_W<ADC_CRrs> {
        ADCALDIF_W::new(self, 30)
    }
    #[doc = "Bit 31 - ADCAL"]
    #[inline(always)]
    #[must_use]
    pub fn adcal(&mut self) -> ADCAL_W<ADC_CRrs> {
        ADCAL_W::new(self, 31)
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
