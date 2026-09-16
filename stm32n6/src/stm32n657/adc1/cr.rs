///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `ADEN` reader - ADC enable control
pub type ADEN_R = crate::BitReader;
///Field `ADEN` writer - ADC enable control
pub type ADEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDIS` reader - ADC disable command
pub type ADDIS_R = crate::BitReader;
///Field `ADDIS` writer - ADC disable command
pub type ADDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADSTART` reader - ADC start of regular conversion
pub type ADSTART_R = crate::BitReader;
///Field `ADSTART` writer - ADC start of regular conversion
pub type ADSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JADSTART` reader - ADC start of injected conversion
pub type JADSTART_R = crate::BitReader;
///Field `JADSTART` writer - ADC start of injected conversion
pub type JADSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADSTP` reader - ADC stop of regular conversion command
pub type ADSTP_R = crate::BitReader;
///Field `ADSTP` writer - ADC stop of regular conversion command
pub type ADSTP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JADSTP` reader - ADC stop of injected conversion command
pub type JADSTP_R = crate::BitReader;
///Field `JADSTP` writer - ADC stop of injected conversion command
pub type JADSTP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEEPPWD` reader - Deep-power-down enable
pub type DEEPPWD_R = crate::BitReader;
///Field `DEEPPWD` writer - Deep-power-down enable
pub type DEEPPWD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCALDIF` reader - Differential mode for calibration
pub type ADCALDIF_R = crate::BitReader;
///Field `ADCALDIF` writer - Differential mode for calibration
pub type ADCALDIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCAL` reader - ADC calibration
pub type ADCAL_R = crate::BitReader;
///Field `ADCAL` writer - ADC calibration
pub type ADCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ADC enable control
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADC disable command
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADC start of regular conversion
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ADC start of injected conversion
    #[inline(always)]
    pub fn jadstart(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC stop of regular conversion command
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADC stop of injected conversion command
    #[inline(always)]
    pub fn jadstp(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 29 - Deep-power-down enable
    #[inline(always)]
    pub fn deeppwd(&self) -> DEEPPWD_R {
        DEEPPWD_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Differential mode for calibration
    #[inline(always)]
    pub fn adcaldif(&self) -> ADCALDIF_R {
        ADCALDIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ADC calibration
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("aden", &self.aden())
            .field("addis", &self.addis())
            .field("adstart", &self.adstart())
            .field("jadstart", &self.jadstart())
            .field("adstp", &self.adstp())
            .field("jadstp", &self.jadstp())
            .field("deeppwd", &self.deeppwd())
            .field("adcaldif", &self.adcaldif())
            .field("adcal", &self.adcal())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC enable control
    #[inline(always)]
    pub fn aden(&mut self) -> ADEN_W<'_, CRrs> {
        ADEN_W::new(self, 0)
    }
    ///Bit 1 - ADC disable command
    #[inline(always)]
    pub fn addis(&mut self) -> ADDIS_W<'_, CRrs> {
        ADDIS_W::new(self, 1)
    }
    ///Bit 2 - ADC start of regular conversion
    #[inline(always)]
    pub fn adstart(&mut self) -> ADSTART_W<'_, CRrs> {
        ADSTART_W::new(self, 2)
    }
    ///Bit 3 - ADC start of injected conversion
    #[inline(always)]
    pub fn jadstart(&mut self) -> JADSTART_W<'_, CRrs> {
        JADSTART_W::new(self, 3)
    }
    ///Bit 4 - ADC stop of regular conversion command
    #[inline(always)]
    pub fn adstp(&mut self) -> ADSTP_W<'_, CRrs> {
        ADSTP_W::new(self, 4)
    }
    ///Bit 5 - ADC stop of injected conversion command
    #[inline(always)]
    pub fn jadstp(&mut self) -> JADSTP_W<'_, CRrs> {
        JADSTP_W::new(self, 5)
    }
    ///Bit 29 - Deep-power-down enable
    #[inline(always)]
    pub fn deeppwd(&mut self) -> DEEPPWD_W<'_, CRrs> {
        DEEPPWD_W::new(self, 29)
    }
    ///Bit 30 - Differential mode for calibration
    #[inline(always)]
    pub fn adcaldif(&mut self) -> ADCALDIF_W<'_, CRrs> {
        ADCALDIF_W::new(self, 30)
    }
    ///Bit 31 - ADC calibration
    #[inline(always)]
    pub fn adcal(&mut self) -> ADCAL_W<'_, CRrs> {
        ADCAL_W::new(self, 31)
    }
}
/**ADC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0x2000_0000
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x2000_0000;
}
