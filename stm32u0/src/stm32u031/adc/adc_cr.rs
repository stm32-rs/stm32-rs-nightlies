///Register `ADC_CR` reader
pub type R = crate::R<ADC_CRrs>;
///Register `ADC_CR` writer
pub type W = crate::W<ADC_CRrs>;
///Field `ADEN` reader - ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command.
pub type ADEN_R = crate::BitReader;
///Field `ADEN` writer - ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command.
pub type ADEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDIS` reader - ADC disable command
pub type ADDIS_R = crate::BitReader;
///Field `ADDIS` writer - ADC disable command
pub type ADDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADSTART` reader - ADC start conversion command
pub type ADSTART_R = crate::BitReader;
///Field `ADSTART` writer - ADC start conversion command
pub type ADSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADSTP` reader - ADC stop conversion command
pub type ADSTP_R = crate::BitReader;
///Field `ADSTP` writer - ADC stop conversion command
pub type ADSTP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADVREGEN` reader - ADC Voltage Regulator Enable
pub type ADVREGEN_R = crate::BitReader;
///Field `ADVREGEN` writer - ADC Voltage Regulator Enable
pub type ADVREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCAL` reader - ADC calibration This bit is set by software to start the calibration of the ADC.
pub type ADCAL_R = crate::BitReader;
///Field `ADCAL` writer - ADC calibration This bit is set by software to start the calibration of the ADC.
pub type ADCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command.
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADC disable command
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADC start conversion command
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - ADC stop conversion command
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 28 - ADC Voltage Regulator Enable
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 31 - ADC calibration This bit is set by software to start the calibration of the ADC.
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CR")
            .field("aden", &self.aden())
            .field("addis", &self.addis())
            .field("adstart", &self.adstart())
            .field("adstp", &self.adstp())
            .field("advregen", &self.advregen())
            .field("adcal", &self.adcal())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command.
    #[inline(always)]
    pub fn aden(&mut self) -> ADEN_W<ADC_CRrs> {
        ADEN_W::new(self, 0)
    }
    ///Bit 1 - ADC disable command
    #[inline(always)]
    pub fn addis(&mut self) -> ADDIS_W<ADC_CRrs> {
        ADDIS_W::new(self, 1)
    }
    ///Bit 2 - ADC start conversion command
    #[inline(always)]
    pub fn adstart(&mut self) -> ADSTART_W<ADC_CRrs> {
        ADSTART_W::new(self, 2)
    }
    ///Bit 4 - ADC stop conversion command
    #[inline(always)]
    pub fn adstp(&mut self) -> ADSTP_W<ADC_CRrs> {
        ADSTP_W::new(self, 4)
    }
    ///Bit 28 - ADC Voltage Regulator Enable
    #[inline(always)]
    pub fn advregen(&mut self) -> ADVREGEN_W<ADC_CRrs> {
        ADVREGEN_W::new(self, 28)
    }
    ///Bit 31 - ADC calibration This bit is set by software to start the calibration of the ADC.
    #[inline(always)]
    pub fn adcal(&mut self) -> ADCAL_W<ADC_CRrs> {
        ADCAL_W::new(self, 31)
    }
}
/**ADC control register

You can [`read`](crate::Reg::read) this register and get [`adc_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#ADC:ADC_CR)*/
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
