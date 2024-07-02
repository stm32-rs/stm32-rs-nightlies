///Register `ADC_OR` reader
pub type R = crate::R<ADC_ORrs>;
///Register `ADC_OR` writer
pub type W = crate::W<ADC_ORrs>;
///Field `CHN21SEL` reader - CHN21SEL
pub type CHN21SEL_R = crate::BitReader;
///Field `CHN21SEL` writer - CHN21SEL
pub type CHN21SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CHN21SEL
    #[inline(always)]
    pub fn chn21sel(&self) -> CHN21SEL_R {
        CHN21SEL_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_OR")
            .field("chn21sel", &self.chn21sel())
            .finish()
    }
}
impl W {
    ///Bit 0 - CHN21SEL
    #[inline(always)]
    #[must_use]
    pub fn chn21sel(&mut self) -> CHN21SEL_W<ADC_ORrs> {
        CHN21SEL_W::new(self, 0)
    }
}
/**ADC option register

You can [`read`](crate::Reg::read) this register and get [`adc_or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC4:ADC_OR)*/
pub struct ADC_ORrs;
impl crate::RegisterSpec for ADC_ORrs {
    type Ux = u32;
}
///`read()` method returns [`adc_or::R`](R) reader structure
impl crate::Readable for ADC_ORrs {}
///`write(|w| ..)` method takes [`adc_or::W`](W) writer structure
impl crate::Writable for ADC_ORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_OR to value 0
impl crate::Resettable for ADC_ORrs {
    const RESET_VALUE: u32 = 0;
}
