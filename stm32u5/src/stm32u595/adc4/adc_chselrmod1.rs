///Register `ADC_CHSELRMOD1` reader
pub type R = crate::R<ADC_CHSELRMOD1rs>;
///Register `ADC_CHSELRMOD1` writer
pub type W = crate::W<ADC_CHSELRMOD1rs>;
///Field `SQ1` reader - SQ1
pub type SQ1_R = crate::FieldReader;
///Field `SQ1` writer - SQ1
pub type SQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ2` reader - SQ2
pub type SQ2_R = crate::FieldReader;
///Field `SQ2` writer - SQ2
pub type SQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ3` reader - SQ3
pub type SQ3_R = crate::FieldReader;
///Field `SQ3` writer - SQ3
pub type SQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ4` reader - SQ4
pub type SQ4_R = crate::FieldReader;
///Field `SQ4` writer - SQ4
pub type SQ4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ5` reader - SQ5
pub type SQ5_R = crate::FieldReader;
///Field `SQ5` writer - SQ5
pub type SQ5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ6` reader - SQ6
pub type SQ6_R = crate::FieldReader;
///Field `SQ6` writer - SQ6
pub type SQ6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ7` reader - SQ7
pub type SQ7_R = crate::FieldReader;
///Field `SQ7` writer - SQ7
pub type SQ7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SQ8` reader - SQ8
pub type SQ8_R = crate::FieldReader;
///Field `SQ8` writer - SQ8
pub type SQ8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - SQ1
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - SQ2
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - SQ3
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - SQ4
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - SQ5
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - SQ6
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - SQ7
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - SQ8
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CHSELRMOD1")
            .field("sq8", &self.sq8())
            .field("sq7", &self.sq7())
            .field("sq6", &self.sq6())
            .field("sq5", &self.sq5())
            .field("sq4", &self.sq4())
            .field("sq3", &self.sq3())
            .field("sq2", &self.sq2())
            .field("sq1", &self.sq1())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - SQ1
    #[inline(always)]
    #[must_use]
    pub fn sq1(&mut self) -> SQ1_W<ADC_CHSELRMOD1rs> {
        SQ1_W::new(self, 0)
    }
    ///Bits 4:7 - SQ2
    #[inline(always)]
    #[must_use]
    pub fn sq2(&mut self) -> SQ2_W<ADC_CHSELRMOD1rs> {
        SQ2_W::new(self, 4)
    }
    ///Bits 8:11 - SQ3
    #[inline(always)]
    #[must_use]
    pub fn sq3(&mut self) -> SQ3_W<ADC_CHSELRMOD1rs> {
        SQ3_W::new(self, 8)
    }
    ///Bits 12:15 - SQ4
    #[inline(always)]
    #[must_use]
    pub fn sq4(&mut self) -> SQ4_W<ADC_CHSELRMOD1rs> {
        SQ4_W::new(self, 12)
    }
    ///Bits 16:19 - SQ5
    #[inline(always)]
    #[must_use]
    pub fn sq5(&mut self) -> SQ5_W<ADC_CHSELRMOD1rs> {
        SQ5_W::new(self, 16)
    }
    ///Bits 20:23 - SQ6
    #[inline(always)]
    #[must_use]
    pub fn sq6(&mut self) -> SQ6_W<ADC_CHSELRMOD1rs> {
        SQ6_W::new(self, 20)
    }
    ///Bits 24:27 - SQ7
    #[inline(always)]
    #[must_use]
    pub fn sq7(&mut self) -> SQ7_W<ADC_CHSELRMOD1rs> {
        SQ7_W::new(self, 24)
    }
    ///Bits 28:31 - SQ8
    #[inline(always)]
    #[must_use]
    pub fn sq8(&mut self) -> SQ8_W<ADC_CHSELRMOD1rs> {
        SQ8_W::new(self, 28)
    }
}
/**ADC channel selection register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`adc_chselrmod1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_chselrmod1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#ADC4:ADC_CHSELRMOD1)*/
pub struct ADC_CHSELRMOD1rs;
impl crate::RegisterSpec for ADC_CHSELRMOD1rs {
    type Ux = u32;
}
///`read()` method returns [`adc_chselrmod1::R`](R) reader structure
impl crate::Readable for ADC_CHSELRMOD1rs {}
///`write(|w| ..)` method takes [`adc_chselrmod1::W`](W) writer structure
impl crate::Writable for ADC_CHSELRMOD1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_CHSELRMOD1 to value 0
impl crate::Resettable for ADC_CHSELRMOD1rs {
    const RESET_VALUE: u32 = 0;
}
