///Register `ADCPS2` reader
pub type R = crate::R<ADCPS2rs>;
///Register `ADCPS2` writer
pub type W = crate::W<ADCPS2rs>;
///Field `ADC6PSC` reader - ADC6PSC
pub type ADC6PSC_R = crate::FieldReader;
///Field `ADC6PSC` writer - ADC6PSC
pub type ADC6PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ADC7PSC` reader - ADC7PSC
pub type ADC7PSC_R = crate::FieldReader;
///Field `ADC7PSC` writer - ADC7PSC
pub type ADC7PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ADC8PSC` reader - ADC8PSC
pub type ADC8PSC_R = crate::FieldReader;
///Field `ADC8PSC` writer - ADC8PSC
pub type ADC8PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ADC9PSC` reader - ADC9PSC
pub type ADC9PSC_R = crate::FieldReader;
///Field `ADC9PSC` writer - ADC9PSC
pub type ADC9PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ADC10PSC` reader - ADC10PSC
pub type ADC10PSC_R = crate::FieldReader;
///Field `ADC10PSC` writer - ADC10PSC
pub type ADC10PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - ADC6PSC
    #[inline(always)]
    pub fn adc6psc(&self) -> ADC6PSC_R {
        ADC6PSC_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:10 - ADC7PSC
    #[inline(always)]
    pub fn adc7psc(&self) -> ADC7PSC_R {
        ADC7PSC_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 12:16 - ADC8PSC
    #[inline(always)]
    pub fn adc8psc(&self) -> ADC8PSC_R {
        ADC8PSC_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    ///Bits 18:22 - ADC9PSC
    #[inline(always)]
    pub fn adc9psc(&self) -> ADC9PSC_R {
        ADC9PSC_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bits 24:28 - ADC10PSC
    #[inline(always)]
    pub fn adc10psc(&self) -> ADC10PSC_R {
        ADC10PSC_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCPS2")
            .field("adc10psc", &self.adc10psc())
            .field("adc9psc", &self.adc9psc())
            .field("adc8psc", &self.adc8psc())
            .field("adc7psc", &self.adc7psc())
            .field("adc6psc", &self.adc6psc())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - ADC6PSC
    #[inline(always)]
    pub fn adc6psc(&mut self) -> ADC6PSC_W<'_, ADCPS2rs> {
        ADC6PSC_W::new(self, 0)
    }
    ///Bits 6:10 - ADC7PSC
    #[inline(always)]
    pub fn adc7psc(&mut self) -> ADC7PSC_W<'_, ADCPS2rs> {
        ADC7PSC_W::new(self, 6)
    }
    ///Bits 12:16 - ADC8PSC
    #[inline(always)]
    pub fn adc8psc(&mut self) -> ADC8PSC_W<'_, ADCPS2rs> {
        ADC8PSC_W::new(self, 12)
    }
    ///Bits 18:22 - ADC9PSC
    #[inline(always)]
    pub fn adc9psc(&mut self) -> ADC9PSC_W<'_, ADCPS2rs> {
        ADC9PSC_W::new(self, 18)
    }
    ///Bits 24:28 - ADC10PSC
    #[inline(always)]
    pub fn adc10psc(&mut self) -> ADC10PSC_W<'_, ADCPS2rs> {
        ADC10PSC_W::new(self, 24)
    }
}
/**HRTIM ADC Post Scaler Register 2

You can [`read`](crate::Reg::read) this register and get [`adcps2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcps2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:ADCPS2)*/
pub struct ADCPS2rs;
impl crate::RegisterSpec for ADCPS2rs {
    type Ux = u32;
}
///`read()` method returns [`adcps2::R`](R) reader structure
impl crate::Readable for ADCPS2rs {}
///`write(|w| ..)` method takes [`adcps2::W`](W) writer structure
impl crate::Writable for ADCPS2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCPS2 to value 0
impl crate::Resettable for ADCPS2rs {}
