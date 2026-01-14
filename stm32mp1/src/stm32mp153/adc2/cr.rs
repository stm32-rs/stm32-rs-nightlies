///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `ADEN` reader - ADEN
pub type ADEN_R = crate::BitReader;
///Field `ADEN` writer - ADEN
pub type ADEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDIS` reader - ADDIS
pub type ADDIS_R = crate::BitReader;
///Field `ADDIS` writer - ADDIS
pub type ADDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADSTART` reader - ADSTART
pub type ADSTART_R = crate::BitReader;
///Field `ADSTART` writer - ADSTART
pub type ADSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JADSTART` reader - JADSTART
pub type JADSTART_R = crate::BitReader;
///Field `JADSTART` writer - JADSTART
pub type JADSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADSTP` reader - ADSTP
pub type ADSTP_R = crate::BitReader;
///Field `ADSTP` writer - ADSTP
pub type ADSTP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JADSTP` reader - JADSTP
pub type JADSTP_R = crate::BitReader;
///Field `JADSTP` writer - JADSTP
pub type JADSTP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOST` reader - BOOST
pub type BOOST_R = crate::BitReader;
///Field `BOOST` writer - BOOST
pub type BOOST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCALLIN` reader - ADCALLIN
pub type ADCALLIN_R = crate::BitReader;
///Field `ADCALLIN` writer - ADCALLIN
pub type ADCALLIN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINCALRDYW1` reader - LINCALRDYW1
pub type LINCALRDYW1_R = crate::BitReader;
///Field `LINCALRDYW1` writer - LINCALRDYW1
pub type LINCALRDYW1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINCALRDYW2` reader - LINCALRDYW2
pub type LINCALRDYW2_R = crate::BitReader;
///Field `LINCALRDYW2` writer - LINCALRDYW2
pub type LINCALRDYW2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINCALRDYW3` reader - LINCALRDYW3
pub type LINCALRDYW3_R = crate::BitReader;
///Field `LINCALRDYW3` writer - LINCALRDYW3
pub type LINCALRDYW3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINCALRDYW4` reader - LINCALRDYW4
pub type LINCALRDYW4_R = crate::BitReader;
///Field `LINCALRDYW4` writer - LINCALRDYW4
pub type LINCALRDYW4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINCALRDYW5` reader - LINCALRDYW5
pub type LINCALRDYW5_R = crate::BitReader;
///Field `LINCALRDYW5` writer - LINCALRDYW5
pub type LINCALRDYW5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINCALRDYW6` reader - LINCALRDYW6
pub type LINCALRDYW6_R = crate::BitReader;
///Field `LINCALRDYW6` writer - LINCALRDYW6
pub type LINCALRDYW6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADVREGEN` reader - ADVREGEN
pub type ADVREGEN_R = crate::BitReader;
///Field `ADVREGEN` writer - ADVREGEN
pub type ADVREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEEPPWD` reader - DEEPPWD
pub type DEEPPWD_R = crate::BitReader;
///Field `DEEPPWD` writer - DEEPPWD
pub type DEEPPWD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCALDIF` reader - ADCALDIF
pub type ADCALDIF_R = crate::BitReader;
///Field `ADCALDIF` writer - ADCALDIF
pub type ADCALDIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCAL` reader - ADCAL
pub type ADCAL_R = crate::BitReader;
///Field `ADCAL` writer - ADCAL
pub type ADCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ADEN
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADDIS
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADSTART
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - JADSTART
    #[inline(always)]
    pub fn jadstart(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADSTP
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - JADSTP
    #[inline(always)]
    pub fn jadstp(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - BOOST
    #[inline(always)]
    pub fn boost(&self) -> BOOST_R {
        BOOST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - ADCALLIN
    #[inline(always)]
    pub fn adcallin(&self) -> ADCALLIN_R {
        ADCALLIN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 22 - LINCALRDYW1
    #[inline(always)]
    pub fn lincalrdyw1(&self) -> LINCALRDYW1_R {
        LINCALRDYW1_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - LINCALRDYW2
    #[inline(always)]
    pub fn lincalrdyw2(&self) -> LINCALRDYW2_R {
        LINCALRDYW2_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - LINCALRDYW3
    #[inline(always)]
    pub fn lincalrdyw3(&self) -> LINCALRDYW3_R {
        LINCALRDYW3_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - LINCALRDYW4
    #[inline(always)]
    pub fn lincalrdyw4(&self) -> LINCALRDYW4_R {
        LINCALRDYW4_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - LINCALRDYW5
    #[inline(always)]
    pub fn lincalrdyw5(&self) -> LINCALRDYW5_R {
        LINCALRDYW5_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - LINCALRDYW6
    #[inline(always)]
    pub fn lincalrdyw6(&self) -> LINCALRDYW6_R {
        LINCALRDYW6_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - ADVREGEN
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DEEPPWD
    #[inline(always)]
    pub fn deeppwd(&self) -> DEEPPWD_R {
        DEEPPWD_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - ADCALDIF
    #[inline(always)]
    pub fn adcaldif(&self) -> ADCALDIF_R {
        ADCALDIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ADCAL
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
            .field("boost", &self.boost())
            .field("adcallin", &self.adcallin())
            .field("lincalrdyw1", &self.lincalrdyw1())
            .field("lincalrdyw2", &self.lincalrdyw2())
            .field("lincalrdyw3", &self.lincalrdyw3())
            .field("lincalrdyw4", &self.lincalrdyw4())
            .field("lincalrdyw5", &self.lincalrdyw5())
            .field("lincalrdyw6", &self.lincalrdyw6())
            .field("advregen", &self.advregen())
            .field("deeppwd", &self.deeppwd())
            .field("adcaldif", &self.adcaldif())
            .field("adcal", &self.adcal())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADEN
    #[inline(always)]
    pub fn aden(&mut self) -> ADEN_W<'_, CRrs> {
        ADEN_W::new(self, 0)
    }
    ///Bit 1 - ADDIS
    #[inline(always)]
    pub fn addis(&mut self) -> ADDIS_W<'_, CRrs> {
        ADDIS_W::new(self, 1)
    }
    ///Bit 2 - ADSTART
    #[inline(always)]
    pub fn adstart(&mut self) -> ADSTART_W<'_, CRrs> {
        ADSTART_W::new(self, 2)
    }
    ///Bit 3 - JADSTART
    #[inline(always)]
    pub fn jadstart(&mut self) -> JADSTART_W<'_, CRrs> {
        JADSTART_W::new(self, 3)
    }
    ///Bit 4 - ADSTP
    #[inline(always)]
    pub fn adstp(&mut self) -> ADSTP_W<'_, CRrs> {
        ADSTP_W::new(self, 4)
    }
    ///Bit 5 - JADSTP
    #[inline(always)]
    pub fn jadstp(&mut self) -> JADSTP_W<'_, CRrs> {
        JADSTP_W::new(self, 5)
    }
    ///Bit 8 - BOOST
    #[inline(always)]
    pub fn boost(&mut self) -> BOOST_W<'_, CRrs> {
        BOOST_W::new(self, 8)
    }
    ///Bit 16 - ADCALLIN
    #[inline(always)]
    pub fn adcallin(&mut self) -> ADCALLIN_W<'_, CRrs> {
        ADCALLIN_W::new(self, 16)
    }
    ///Bit 22 - LINCALRDYW1
    #[inline(always)]
    pub fn lincalrdyw1(&mut self) -> LINCALRDYW1_W<'_, CRrs> {
        LINCALRDYW1_W::new(self, 22)
    }
    ///Bit 23 - LINCALRDYW2
    #[inline(always)]
    pub fn lincalrdyw2(&mut self) -> LINCALRDYW2_W<'_, CRrs> {
        LINCALRDYW2_W::new(self, 23)
    }
    ///Bit 24 - LINCALRDYW3
    #[inline(always)]
    pub fn lincalrdyw3(&mut self) -> LINCALRDYW3_W<'_, CRrs> {
        LINCALRDYW3_W::new(self, 24)
    }
    ///Bit 25 - LINCALRDYW4
    #[inline(always)]
    pub fn lincalrdyw4(&mut self) -> LINCALRDYW4_W<'_, CRrs> {
        LINCALRDYW4_W::new(self, 25)
    }
    ///Bit 26 - LINCALRDYW5
    #[inline(always)]
    pub fn lincalrdyw5(&mut self) -> LINCALRDYW5_W<'_, CRrs> {
        LINCALRDYW5_W::new(self, 26)
    }
    ///Bit 27 - LINCALRDYW6
    #[inline(always)]
    pub fn lincalrdyw6(&mut self) -> LINCALRDYW6_W<'_, CRrs> {
        LINCALRDYW6_W::new(self, 27)
    }
    ///Bit 28 - ADVREGEN
    #[inline(always)]
    pub fn advregen(&mut self) -> ADVREGEN_W<'_, CRrs> {
        ADVREGEN_W::new(self, 28)
    }
    ///Bit 29 - DEEPPWD
    #[inline(always)]
    pub fn deeppwd(&mut self) -> DEEPPWD_W<'_, CRrs> {
        DEEPPWD_W::new(self, 29)
    }
    ///Bit 30 - ADCALDIF
    #[inline(always)]
    pub fn adcaldif(&mut self) -> ADCALDIF_W<'_, CRrs> {
        ADCALDIF_W::new(self, 30)
    }
    ///Bit 31 - ADCAL
    #[inline(always)]
    pub fn adcal(&mut self) -> ADCAL_W<'_, CRrs> {
        ADCAL_W::new(self, 31)
    }
}
/**ADC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ADC2:CR)*/
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
