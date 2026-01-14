///Register `AWD3CR` reader
pub type R = crate::R<AWD3CRrs>;
///Register `AWD3CR` writer
pub type W = crate::W<AWD3CRrs>;
///Field `AWD3CH` reader - Analog watchdog 3 channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by the analog watchdog 3. AWD3CH\[i\] = 0: ADC analog input channel i is not monitored by AWD3 AWD3CH\[i\] = 1: ADC analog input channel i is monitored by AWD3 When AWD3CH\[18:0\] = 000..0, the analog Watchdog 3 is disabled Note: The channels selected by AWD3CH must be also selected into the SQRi or JSQRi registers. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: Some channels are not connected physically and must not be selected for the analog watchdog.
pub type AWD3CH_R = crate::FieldReader<u32>;
///Field `AWD3CH` writer - Analog watchdog 3 channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by the analog watchdog 3. AWD3CH\[i\] = 0: ADC analog input channel i is not monitored by AWD3 AWD3CH\[i\] = 1: ADC analog input channel i is monitored by AWD3 When AWD3CH\[18:0\] = 000..0, the analog Watchdog 3 is disabled Note: The channels selected by AWD3CH must be also selected into the SQRi or JSQRi registers. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: Some channels are not connected physically and must not be selected for the analog watchdog.
pub type AWD3CH_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    ///Bits 0:18 - Analog watchdog 3 channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by the analog watchdog 3. AWD3CH\[i\] = 0: ADC analog input channel i is not monitored by AWD3 AWD3CH\[i\] = 1: ADC analog input channel i is monitored by AWD3 When AWD3CH\[18:0\] = 000..0, the analog Watchdog 3 is disabled Note: The channels selected by AWD3CH must be also selected into the SQRi or JSQRi registers. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: Some channels are not connected physically and must not be selected for the analog watchdog.
    #[inline(always)]
    pub fn awd3ch(&self) -> AWD3CH_R {
        AWD3CH_R::new(self.bits & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD3CR")
            .field("awd3ch", &self.awd3ch())
            .finish()
    }
}
impl W {
    ///Bits 0:18 - Analog watchdog 3 channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by the analog watchdog 3. AWD3CH\[i\] = 0: ADC analog input channel i is not monitored by AWD3 AWD3CH\[i\] = 1: ADC analog input channel i is monitored by AWD3 When AWD3CH\[18:0\] = 000..0, the analog Watchdog 3 is disabled Note: The channels selected by AWD3CH must be also selected into the SQRi or JSQRi registers. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: Some channels are not connected physically and must not be selected for the analog watchdog.
    #[inline(always)]
    pub fn awd3ch(&mut self) -> AWD3CH_W<'_, AWD3CRrs> {
        AWD3CH_W::new(self, 0)
    }
}
/**ADC Analog Watchdog 3 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`awd3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ADC1:AWD3CR)*/
pub struct AWD3CRrs;
impl crate::RegisterSpec for AWD3CRrs {
    type Ux = u32;
}
///`read()` method returns [`awd3cr::R`](R) reader structure
impl crate::Readable for AWD3CRrs {}
///`write(|w| ..)` method takes [`awd3cr::W`](W) writer structure
impl crate::Writable for AWD3CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AWD3CR to value 0
impl crate::Resettable for AWD3CRrs {}
