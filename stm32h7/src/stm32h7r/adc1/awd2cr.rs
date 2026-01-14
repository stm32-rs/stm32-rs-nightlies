///Register `AWD2CR` reader
pub type R = crate::R<AWD2CRrs>;
///Register `AWD2CR` writer
pub type W = crate::W<AWD2CRrs>;
///Field `AWD2CH` reader - Analog watchdog 2 channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by the analog watchdog 2. AWD2CH\[i\] = 0: ADC analog input channel i is not monitored by AWD2 AWD2CH\[i\] = 1: ADC analog input channel i is monitored by AWD2 When AWD2CH\[18:0\] = 000..0, the analog Watchdog 2 is disabled Note: The channels selected by AWD2CH must be also selected into the SQRi or JSQRi registers. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: Some channels are not connected physically and must not be selected for the analog watchdog.
pub type AWD2CH_R = crate::FieldReader<u32>;
///Field `AWD2CH` writer - Analog watchdog 2 channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by the analog watchdog 2. AWD2CH\[i\] = 0: ADC analog input channel i is not monitored by AWD2 AWD2CH\[i\] = 1: ADC analog input channel i is monitored by AWD2 When AWD2CH\[18:0\] = 000..0, the analog Watchdog 2 is disabled Note: The channels selected by AWD2CH must be also selected into the SQRi or JSQRi registers. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: Some channels are not connected physically and must not be selected for the analog watchdog.
pub type AWD2CH_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    ///Bits 0:18 - Analog watchdog 2 channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by the analog watchdog 2. AWD2CH\[i\] = 0: ADC analog input channel i is not monitored by AWD2 AWD2CH\[i\] = 1: ADC analog input channel i is monitored by AWD2 When AWD2CH\[18:0\] = 000..0, the analog Watchdog 2 is disabled Note: The channels selected by AWD2CH must be also selected into the SQRi or JSQRi registers. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: Some channels are not connected physically and must not be selected for the analog watchdog.
    #[inline(always)]
    pub fn awd2ch(&self) -> AWD2CH_R {
        AWD2CH_R::new(self.bits & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD2CR")
            .field("awd2ch", &self.awd2ch())
            .finish()
    }
}
impl W {
    ///Bits 0:18 - Analog watchdog 2 channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by the analog watchdog 2. AWD2CH\[i\] = 0: ADC analog input channel i is not monitored by AWD2 AWD2CH\[i\] = 1: ADC analog input channel i is monitored by AWD2 When AWD2CH\[18:0\] = 000..0, the analog Watchdog 2 is disabled Note: The channels selected by AWD2CH must be also selected into the SQRi or JSQRi registers. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: Some channels are not connected physically and must not be selected for the analog watchdog.
    #[inline(always)]
    pub fn awd2ch(&mut self) -> AWD2CH_W<'_, AWD2CRrs> {
        AWD2CH_W::new(self, 0)
    }
}
/**ADC Analog Watchdog 2 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`awd2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#ADC1:AWD2CR)*/
pub struct AWD2CRrs;
impl crate::RegisterSpec for AWD2CRrs {
    type Ux = u32;
}
///`read()` method returns [`awd2cr::R`](R) reader structure
impl crate::Readable for AWD2CRrs {}
///`write(|w| ..)` method takes [`awd2cr::W`](W) writer structure
impl crate::Writable for AWD2CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AWD2CR to value 0
impl crate::Resettable for AWD2CRrs {}
