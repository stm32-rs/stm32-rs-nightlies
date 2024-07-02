///Register `ADC_AWD3TR` reader
pub type R = crate::R<ADC_AWD3TRrs>;
///Register `ADC_AWD3TR` writer
pub type W = crate::W<ADC_AWD3TRrs>;
///Field `LT3` reader - LT3
pub type LT3_R = crate::FieldReader<u16>;
///Field `LT3` writer - LT3
pub type LT3_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `HT3` reader - HT3
pub type HT3_R = crate::FieldReader<u16>;
///Field `HT3` writer - HT3
pub type HT3_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - LT3
    #[inline(always)]
    pub fn lt3(&self) -> LT3_R {
        LT3_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - HT3
    #[inline(always)]
    pub fn ht3(&self) -> HT3_R {
        HT3_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_AWD3TR")
            .field("ht3", &self.ht3())
            .field("lt3", &self.lt3())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - LT3
    #[inline(always)]
    #[must_use]
    pub fn lt3(&mut self) -> LT3_W<ADC_AWD3TRrs> {
        LT3_W::new(self, 0)
    }
    ///Bits 16:27 - HT3
    #[inline(always)]
    #[must_use]
    pub fn ht3(&mut self) -> HT3_W<ADC_AWD3TRrs> {
        HT3_W::new(self, 16)
    }
}
/**ADC watchdog threshold register

You can [`read`](crate::Reg::read) this register and get [`adc_awd3tr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_awd3tr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADC4:ADC_AWD3TR)*/
pub struct ADC_AWD3TRrs;
impl crate::RegisterSpec for ADC_AWD3TRrs {
    type Ux = u32;
}
///`read()` method returns [`adc_awd3tr::R`](R) reader structure
impl crate::Readable for ADC_AWD3TRrs {}
///`write(|w| ..)` method takes [`adc_awd3tr::W`](W) writer structure
impl crate::Writable for ADC_AWD3TRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_AWD3TR to value 0x0fff_0000
impl crate::Resettable for ADC_AWD3TRrs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
