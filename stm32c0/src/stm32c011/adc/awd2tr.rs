///Register `AWD2TR` reader
pub type R = crate::R<AWD2TRrs>;
///Register `AWD2TR` writer
pub type W = crate::W<AWD2TRrs>;
///Field `LT2` reader - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on page 359.
pub type LT2_R = crate::FieldReader<u16>;
///Field `LT2` writer - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on page 359.
pub type LT2_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
///Field `HT2` reader - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on page 359.
pub type HT2_R = crate::FieldReader<u16>;
///Field `HT2` writer - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on page 359.
pub type HT2_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    ///Bits 0:11 - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on page 359.
    #[inline(always)]
    pub fn lt2(&self) -> LT2_R {
        LT2_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on page 359.
    #[inline(always)]
    pub fn ht2(&self) -> HT2_R {
        HT2_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD2TR")
            .field("lt2", &self.lt2())
            .field("ht2", &self.ht2())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on page 359.
    #[inline(always)]
    pub fn lt2(&mut self) -> LT2_W<AWD2TRrs> {
        LT2_W::new(self, 0)
    }
    ///Bits 16:27 - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on page 359.
    #[inline(always)]
    pub fn ht2(&mut self) -> HT2_W<AWD2TRrs> {
        HT2_W::new(self, 16)
    }
}
/**ADC watchdog threshold register

You can [`read`](crate::Reg::read) this register and get [`awd2tr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2tr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#ADC:AWD2TR)*/
pub struct AWD2TRrs;
impl crate::RegisterSpec for AWD2TRrs {
    type Ux = u32;
}
///`read()` method returns [`awd2tr::R`](R) reader structure
impl crate::Readable for AWD2TRrs {}
///`write(|w| ..)` method takes [`awd2tr::W`](W) writer structure
impl crate::Writable for AWD2TRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AWD2TR to value 0x0fff_0000
impl crate::Resettable for AWD2TRrs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
