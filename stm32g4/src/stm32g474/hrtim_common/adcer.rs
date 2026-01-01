///Register `ADCER` reader
pub type R = crate::R<ADCERrs>;
///Register `ADCER` writer
pub type W = crate::W<ADCERrs>;
///Field `ADC5TRG` reader - ADC5TRG
pub type ADC5TRG_R = crate::FieldReader;
///Field `ADC5TRG` writer - ADC5TRG
pub type ADC5TRG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ADC6TRG` reader - ADC6TRG
pub type ADC6TRG_R = crate::FieldReader;
///Field `ADC6TRG` writer - ADC6TRG
pub type ADC6TRG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ADC7TRG` reader - ADC7TRG
pub type ADC7TRG_R = crate::FieldReader;
///Field `ADC7TRG` writer - ADC7TRG
pub type ADC7TRG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ADC8TRG` reader - ADC8TRG
pub type ADC8TRG_R = crate::FieldReader;
///Field `ADC8TRG` writer - ADC8TRG
pub type ADC8TRG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ADC9TRG` reader - ADC9TRG
pub type ADC9TRG_R = crate::FieldReader;
///Field `ADC9TRG` writer - ADC9TRG
pub type ADC9TRG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ADC10TRG` reader - ADC10TRG
pub type ADC10TRG_R = crate::FieldReader;
///Field `ADC10TRG` writer - ADC10TRG
pub type ADC10TRG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - ADC5TRG
    #[inline(always)]
    pub fn adc5trg(&self) -> ADC5TRG_R {
        ADC5TRG_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - ADC6TRG
    #[inline(always)]
    pub fn adc6trg(&self) -> ADC6TRG_R {
        ADC6TRG_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - ADC7TRG
    #[inline(always)]
    pub fn adc7trg(&self) -> ADC7TRG_R {
        ADC7TRG_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 16:20 - ADC8TRG
    #[inline(always)]
    pub fn adc8trg(&self) -> ADC8TRG_R {
        ADC8TRG_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:25 - ADC9TRG
    #[inline(always)]
    pub fn adc9trg(&self) -> ADC9TRG_R {
        ADC9TRG_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bits 26:30 - ADC10TRG
    #[inline(always)]
    pub fn adc10trg(&self) -> ADC10TRG_R {
        ADC10TRG_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCER")
            .field("adc10trg", &self.adc10trg())
            .field("adc9trg", &self.adc9trg())
            .field("adc8trg", &self.adc8trg())
            .field("adc7trg", &self.adc7trg())
            .field("adc6trg", &self.adc6trg())
            .field("adc5trg", &self.adc5trg())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - ADC5TRG
    #[inline(always)]
    pub fn adc5trg(&mut self) -> ADC5TRG_W<'_, ADCERrs> {
        ADC5TRG_W::new(self, 0)
    }
    ///Bits 5:9 - ADC6TRG
    #[inline(always)]
    pub fn adc6trg(&mut self) -> ADC6TRG_W<'_, ADCERrs> {
        ADC6TRG_W::new(self, 5)
    }
    ///Bits 10:14 - ADC7TRG
    #[inline(always)]
    pub fn adc7trg(&mut self) -> ADC7TRG_W<'_, ADCERrs> {
        ADC7TRG_W::new(self, 10)
    }
    ///Bits 16:20 - ADC8TRG
    #[inline(always)]
    pub fn adc8trg(&mut self) -> ADC8TRG_W<'_, ADCERrs> {
        ADC8TRG_W::new(self, 16)
    }
    ///Bits 21:25 - ADC9TRG
    #[inline(always)]
    pub fn adc9trg(&mut self) -> ADC9TRG_W<'_, ADCERrs> {
        ADC9TRG_W::new(self, 21)
    }
    ///Bits 26:30 - ADC10TRG
    #[inline(always)]
    pub fn adc10trg(&mut self) -> ADC10TRG_W<'_, ADCERrs> {
        ADC10TRG_W::new(self, 26)
    }
}
/**HRTIM ADC Extended Trigger Register

You can [`read`](crate::Reg::read) this register and get [`adcer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:ADCER)*/
pub struct ADCERrs;
impl crate::RegisterSpec for ADCERrs {
    type Ux = u32;
}
///`read()` method returns [`adcer::R`](R) reader structure
impl crate::Readable for ADCERrs {}
///`write(|w| ..)` method takes [`adcer::W`](W) writer structure
impl crate::Writable for ADCERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCER to value 0
impl crate::Resettable for ADCERrs {}
