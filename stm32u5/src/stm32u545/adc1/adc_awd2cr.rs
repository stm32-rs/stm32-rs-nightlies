///Register `ADC_AWD2CR` reader
pub type R = crate::R<ADC_AWD2CRrs>;
///Register `ADC_AWD2CR` writer
pub type W = crate::W<ADC_AWD2CRrs>;
///Field `AWD2CH` reader - AWD2CH
pub type AWD2CH_R = crate::FieldReader<u32>;
///Field `AWD2CH` writer - AWD2CH
pub type AWD2CH_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - AWD2CH
    #[inline(always)]
    pub fn awd2ch(&self) -> AWD2CH_R {
        AWD2CH_R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_AWD2CR")
            .field("awd2ch", &self.awd2ch())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - AWD2CH
    #[inline(always)]
    #[must_use]
    pub fn awd2ch(&mut self) -> AWD2CH_W<ADC_AWD2CRrs> {
        AWD2CH_W::new(self, 0)
    }
}
/**ADC analog watchdog 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`adc_awd2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_awd2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#ADC1:ADC_AWD2CR)*/
pub struct ADC_AWD2CRrs;
impl crate::RegisterSpec for ADC_AWD2CRrs {
    type Ux = u32;
}
///`read()` method returns [`adc_awd2cr::R`](R) reader structure
impl crate::Readable for ADC_AWD2CRrs {}
///`write(|w| ..)` method takes [`adc_awd2cr::W`](W) writer structure
impl crate::Writable for ADC_AWD2CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_AWD2CR to value 0
impl crate::Resettable for ADC_AWD2CRrs {
    const RESET_VALUE: u32 = 0;
}
