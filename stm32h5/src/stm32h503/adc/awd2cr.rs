#[doc = "Register `AWD2CR` reader"]
pub type R = crate::R<AWD2CRrs>;
#[doc = "Register `AWD2CR` writer"]
pub type W = crate::W<AWD2CRrs>;
#[doc = "Field `AWD2CH` reader - Analog watchdog 2 channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by the analog watchdog 2. AWD2CH\\[i\\]
= 0: ADC analog input channel i is not monitored by AWD2 AWD2CH\\[i\\]
= 1: ADC analog input channel i is monitored by AWD2 When AWD2CH\\[19:0\\]
= 000..0, the analog Watchdog 2 is disabled Note: The channels selected by AWD2CH must be also selected into the SQRi or JSQRi registers. The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically and must not be selected for the analog watchdog."]
pub type AWD2CH_R = crate::FieldReader<u32>;
#[doc = "Field `AWD2CH` writer - Analog watchdog 2 channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by the analog watchdog 2. AWD2CH\\[i\\]
= 0: ADC analog input channel i is not monitored by AWD2 AWD2CH\\[i\\]
= 1: ADC analog input channel i is monitored by AWD2 When AWD2CH\\[19:0\\]
= 000..0, the analog Watchdog 2 is disabled Note: The channels selected by AWD2CH must be also selected into the SQRi or JSQRi registers. The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically and must not be selected for the analog watchdog."]
pub type AWD2CH_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Analog watchdog 2 channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by the analog watchdog 2. AWD2CH\\[i\\]
= 0: ADC analog input channel i is not monitored by AWD2 AWD2CH\\[i\\]
= 1: ADC analog input channel i is monitored by AWD2 When AWD2CH\\[19:0\\]
= 000..0, the analog Watchdog 2 is disabled Note: The channels selected by AWD2CH must be also selected into the SQRi or JSQRi registers. The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically and must not be selected for the analog watchdog."]
    #[inline(always)]
    pub fn awd2ch(&self) -> AWD2CH_R {
        AWD2CH_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Analog watchdog 2 channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by the analog watchdog 2. AWD2CH\\[i\\]
= 0: ADC analog input channel i is not monitored by AWD2 AWD2CH\\[i\\]
= 1: ADC analog input channel i is monitored by AWD2 When AWD2CH\\[19:0\\]
= 000..0, the analog Watchdog 2 is disabled Note: The channels selected by AWD2CH must be also selected into the SQRi or JSQRi registers. The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically and must not be selected for the analog watchdog."]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 0)
    }
}
#[doc = "ADC Analog Watchdog 2 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd2cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd2cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWD2CRrs;
impl crate::RegisterSpec for AWD2CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awd2cr::R`](R) reader structure"]
impl crate::Readable for AWD2CRrs {}
#[doc = "`write(|w| ..)` method takes [`awd2cr::W`](W) writer structure"]
impl crate::Writable for AWD2CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWD2CR to value 0"]
impl crate::Resettable for AWD2CRrs {
    const RESET_VALUE: u32 = 0;
}
