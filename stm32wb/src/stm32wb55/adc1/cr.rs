#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `ADEN` reader - ADC enable"]
pub type ADEN_R = crate::BitReader;
#[doc = "Field `ADEN` writer - ADC enable"]
pub type ADEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDIS` reader - ADC disable"]
pub type ADDIS_R = crate::BitReader;
#[doc = "Field `ADDIS` writer - ADC disable"]
pub type ADDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTART` reader - ADC group regular conversion start"]
pub type ADSTART_R = crate::BitReader;
#[doc = "Field `ADSTART` writer - ADC group regular conversion start"]
pub type ADSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JADSTART` reader - ADC group injected conversion start"]
pub type JADSTART_R = crate::BitReader;
#[doc = "Field `JADSTART` writer - ADC group injected conversion start"]
pub type JADSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTP` reader - ADC group regular conversion stop"]
pub type ADSTP_R = crate::BitReader;
#[doc = "Field `ADSTP` writer - ADC group regular conversion stop"]
pub type ADSTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JADSTP` reader - ADC group injected conversion stop"]
pub type JADSTP_R = crate::BitReader;
#[doc = "Field `JADSTP` writer - ADC group injected conversion stop"]
pub type JADSTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADVREGEN` reader - ADC voltage regulator enable"]
pub type ADVREGEN_R = crate::BitReader;
#[doc = "Field `ADVREGEN` writer - ADC voltage regulator enable"]
pub type ADVREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEEPPWD` reader - ADC deep power down enable"]
pub type DEEPPWD_R = crate::BitReader;
#[doc = "Field `DEEPPWD` writer - ADC deep power down enable"]
pub type DEEPPWD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCALDIF` reader - ADC differential mode for calibration"]
pub type ADCALDIF_R = crate::BitReader;
#[doc = "Field `ADCALDIF` writer - ADC differential mode for calibration"]
pub type ADCALDIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCAL` reader - ADC calibration"]
pub type ADCAL_R = crate::BitReader;
#[doc = "Field `ADCAL` writer - ADC calibration"]
pub type ADCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC disable"]
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC group regular conversion start"]
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC group injected conversion start"]
    #[inline(always)]
    pub fn jadstart(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC group regular conversion stop"]
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC group injected conversion stop"]
    #[inline(always)]
    pub fn jadstp(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC voltage regulator enable"]
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ADC deep power down enable"]
    #[inline(always)]
    pub fn deeppwd(&self) -> DEEPPWD_R {
        DEEPPWD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ADC differential mode for calibration"]
    #[inline(always)]
    pub fn adcaldif(&self) -> ADCALDIF_R {
        ADCALDIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ADC calibration"]
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    #[must_use]
    pub fn aden(&mut self) -> ADEN_W<CRrs> {
        ADEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC disable"]
    #[inline(always)]
    #[must_use]
    pub fn addis(&mut self) -> ADDIS_W<CRrs> {
        ADDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC group regular conversion start"]
    #[inline(always)]
    #[must_use]
    pub fn adstart(&mut self) -> ADSTART_W<CRrs> {
        ADSTART_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC group injected conversion start"]
    #[inline(always)]
    #[must_use]
    pub fn jadstart(&mut self) -> JADSTART_W<CRrs> {
        JADSTART_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC group regular conversion stop"]
    #[inline(always)]
    #[must_use]
    pub fn adstp(&mut self) -> ADSTP_W<CRrs> {
        ADSTP_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC group injected conversion stop"]
    #[inline(always)]
    #[must_use]
    pub fn jadstp(&mut self) -> JADSTP_W<CRrs> {
        JADSTP_W::new(self, 5)
    }
    #[doc = "Bit 28 - ADC voltage regulator enable"]
    #[inline(always)]
    #[must_use]
    pub fn advregen(&mut self) -> ADVREGEN_W<CRrs> {
        ADVREGEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - ADC deep power down enable"]
    #[inline(always)]
    #[must_use]
    pub fn deeppwd(&mut self) -> DEEPPWD_W<CRrs> {
        DEEPPWD_W::new(self, 29)
    }
    #[doc = "Bit 30 - ADC differential mode for calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcaldif(&mut self) -> ADCALDIF_W<CRrs> {
        ADCALDIF_W::new(self, 30)
    }
    #[doc = "Bit 31 - ADC calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcal(&mut self) -> ADCAL_W<CRrs> {
        ADCAL_W::new(self, 31)
    }
}
#[doc = "ADC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
