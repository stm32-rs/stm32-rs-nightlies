///Register `ADC_AWD1TR` reader
pub type R = crate::R<ADC_AWD1TRrs>;
///Register `ADC_AWD1TR` writer
pub type W = crate::W<ADC_AWD1TRrs>;
///Field `LT1` reader - Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section 16.8: Analog window watchdogs on page 329.
pub type LT1_R = crate::FieldReader<u16>;
///Field `LT1` writer - Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section 16.8: Analog window watchdogs on page 329.
pub type LT1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `HT1` reader - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section 16.8: Analog window watchdogs on page 329.
pub type HT1_R = crate::FieldReader<u16>;
///Field `HT1` writer - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section 16.8: Analog window watchdogs on page 329.
pub type HT1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section 16.8: Analog window watchdogs on page 329.
    #[inline(always)]
    pub fn lt1(&self) -> LT1_R {
        LT1_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section 16.8: Analog window watchdogs on page 329.
    #[inline(always)]
    pub fn ht1(&self) -> HT1_R {
        HT1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_AWD1TR")
            .field("lt1", &self.lt1())
            .field("ht1", &self.ht1())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section 16.8: Analog window watchdogs on page 329.
    #[inline(always)]
    pub fn lt1(&mut self) -> LT1_W<'_, ADC_AWD1TRrs> {
        LT1_W::new(self, 0)
    }
    ///Bits 16:27 - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section 16.8: Analog window watchdogs on page 329.
    #[inline(always)]
    pub fn ht1(&mut self) -> HT1_W<'_, ADC_AWD1TRrs> {
        HT1_W::new(self, 16)
    }
}
/**ADC watchdog threshold register

You can [`read`](crate::Reg::read) this register and get [`adc_awd1tr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_awd1tr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#ADC:ADC_AWD1TR)*/
pub struct ADC_AWD1TRrs;
impl crate::RegisterSpec for ADC_AWD1TRrs {
    type Ux = u32;
}
///`read()` method returns [`adc_awd1tr::R`](R) reader structure
impl crate::Readable for ADC_AWD1TRrs {}
///`write(|w| ..)` method takes [`adc_awd1tr::W`](W) writer structure
impl crate::Writable for ADC_AWD1TRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADC_AWD1TR to value 0x0fff_0000
impl crate::Resettable for ADC_AWD1TRrs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
