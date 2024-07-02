///Register `ADC_CR` reader
pub type R = crate::R<ADC_CRrs>;
///Register `ADC_CR` writer
pub type W = crate::W<ADC_CRrs>;
///Field `ADEN` reader - ADEN
pub type ADEN_R = crate::BitReader;
///Field `ADDIS` reader - ADDIS
pub type ADDIS_R = crate::BitReader;
///Field `ADSTART` reader - ADSTART
pub type ADSTART_R = crate::BitReader;
///Field `ADSTP` reader - ADSTP
pub type ADSTP_R = crate::BitReader;
///Field `ADVREGEN` reader - ADVREGEN
pub type ADVREGEN_R = crate::BitReader;
///Field `ADVREGEN` writer - ADVREGEN
pub type ADVREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCAL` reader - ADCAL
pub type ADCAL_R = crate::BitReader;
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
    ///Bit 4 - ADSTP
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 28 - ADVREGEN
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 31 - ADCAL
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CR")
            .field("adcal", &self.adcal())
            .field("advregen", &self.advregen())
            .field("adstp", &self.adstp())
            .field("adstart", &self.adstart())
            .field("addis", &self.addis())
            .field("aden", &self.aden())
            .finish()
    }
}
impl W {
    ///Bit 28 - ADVREGEN
    #[inline(always)]
    #[must_use]
    pub fn advregen(&mut self) -> ADVREGEN_W<ADC_CRrs> {
        ADVREGEN_W::new(self, 28)
    }
}
/**ADC control register

You can [`read`](crate::Reg::read) this register and get [`adc_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:ADC_CR)*/
pub struct ADC_CRrs;
impl crate::RegisterSpec for ADC_CRrs {
    type Ux = u32;
}
///`read()` method returns [`adc_cr::R`](R) reader structure
impl crate::Readable for ADC_CRrs {}
///`write(|w| ..)` method takes [`adc_cr::W`](W) writer structure
impl crate::Writable for ADC_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_CR to value 0
impl crate::Resettable for ADC_CRrs {
    const RESET_VALUE: u32 = 0;
}
