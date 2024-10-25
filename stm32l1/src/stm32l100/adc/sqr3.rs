///Register `SQR3` reader
pub type R = crate::R<SQR3rs>;
///Register `SQR3` writer
pub type W = crate::W<SQR3rs>;
///Field `SQ13` reader - 13th conversion in regular sequence
pub type SQ13_R = crate::FieldReader;
///Field `SQ13` writer - 13th conversion in regular sequence
pub type SQ13_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ14` reader - 14th conversion in regular sequence
pub type SQ14_R = crate::FieldReader;
///Field `SQ14` writer - 14th conversion in regular sequence
pub type SQ14_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ15` reader - 15th conversion in regular sequence
pub type SQ15_R = crate::FieldReader;
///Field `SQ15` writer - 15th conversion in regular sequence
pub type SQ15_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ16` reader - 16th conversion in regular sequence
pub type SQ16_R = crate::FieldReader;
///Field `SQ16` writer - 16th conversion in regular sequence
pub type SQ16_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ17` reader - 17th conversion in regular sequence
pub type SQ17_R = crate::FieldReader;
///Field `SQ17` writer - 17th conversion in regular sequence
pub type SQ17_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ18` reader - 18th conversion in regular sequence
pub type SQ18_R = crate::FieldReader;
///Field `SQ18` writer - 18th conversion in regular sequence
pub type SQ18_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - 13th conversion in regular sequence
    #[inline(always)]
    pub fn sq13(&self) -> SQ13_R {
        SQ13_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - 14th conversion in regular sequence
    #[inline(always)]
    pub fn sq14(&self) -> SQ14_R {
        SQ14_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - 15th conversion in regular sequence
    #[inline(always)]
    pub fn sq15(&self) -> SQ15_R {
        SQ15_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19 - 16th conversion in regular sequence
    #[inline(always)]
    pub fn sq16(&self) -> SQ16_R {
        SQ16_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 20:24 - 17th conversion in regular sequence
    #[inline(always)]
    pub fn sq17(&self) -> SQ17_R {
        SQ17_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bits 25:29 - 18th conversion in regular sequence
    #[inline(always)]
    pub fn sq18(&self) -> SQ18_R {
        SQ18_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SQR3")
            .field("sq18", &self.sq18())
            .field("sq17", &self.sq17())
            .field("sq16", &self.sq16())
            .field("sq15", &self.sq15())
            .field("sq14", &self.sq14())
            .field("sq13", &self.sq13())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - 13th conversion in regular sequence
    #[inline(always)]
    #[must_use]
    pub fn sq13(&mut self) -> SQ13_W<SQR3rs> {
        SQ13_W::new(self, 0)
    }
    ///Bits 5:9 - 14th conversion in regular sequence
    #[inline(always)]
    #[must_use]
    pub fn sq14(&mut self) -> SQ14_W<SQR3rs> {
        SQ14_W::new(self, 5)
    }
    ///Bits 10:14 - 15th conversion in regular sequence
    #[inline(always)]
    #[must_use]
    pub fn sq15(&mut self) -> SQ15_W<SQR3rs> {
        SQ15_W::new(self, 10)
    }
    ///Bits 15:19 - 16th conversion in regular sequence
    #[inline(always)]
    #[must_use]
    pub fn sq16(&mut self) -> SQ16_W<SQR3rs> {
        SQ16_W::new(self, 15)
    }
    ///Bits 20:24 - 17th conversion in regular sequence
    #[inline(always)]
    #[must_use]
    pub fn sq17(&mut self) -> SQ17_W<SQR3rs> {
        SQ17_W::new(self, 20)
    }
    ///Bits 25:29 - 18th conversion in regular sequence
    #[inline(always)]
    #[must_use]
    pub fn sq18(&mut self) -> SQ18_W<SQR3rs> {
        SQ18_W::new(self, 25)
    }
}
/**regular sequence register 3

You can [`read`](crate::Reg::read) this register and get [`sqr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#ADC:SQR3)*/
pub struct SQR3rs;
impl crate::RegisterSpec for SQR3rs {
    type Ux = u32;
}
///`read()` method returns [`sqr3::R`](R) reader structure
impl crate::Readable for SQR3rs {}
///`write(|w| ..)` method takes [`sqr3::W`](W) writer structure
impl crate::Writable for SQR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SQR3 to value 0
impl crate::Resettable for SQR3rs {
    const RESET_VALUE: u32 = 0;
}
