///Register `SQR2` reader
pub type R = crate::R<SQR2rs>;
///Register `SQR2` writer
pub type W = crate::W<SQR2rs>;
///Field `SQ5` reader - SQ5
pub type SQ5_R = crate::FieldReader;
///Field `SQ5` writer - SQ5
pub type SQ5_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ6` reader - SQ6
pub type SQ6_R = crate::FieldReader;
///Field `SQ6` writer - SQ6
pub type SQ6_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ7` reader - SQ7
pub type SQ7_R = crate::FieldReader;
///Field `SQ7` writer - SQ7
pub type SQ7_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ8` reader - SQ8
pub type SQ8_R = crate::FieldReader;
///Field `SQ8` writer - SQ8
pub type SQ8_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ9` reader - SQ9
pub type SQ9_R = crate::FieldReader;
///Field `SQ9` writer - SQ9
pub type SQ9_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - SQ5
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:10 - SQ6
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 12:16 - SQ7
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    ///Bits 18:22 - SQ8
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bits 24:28 - SQ9
    #[inline(always)]
    pub fn sq9(&self) -> SQ9_R {
        SQ9_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SQR2")
            .field("sq5", &self.sq5())
            .field("sq6", &self.sq6())
            .field("sq7", &self.sq7())
            .field("sq8", &self.sq8())
            .field("sq9", &self.sq9())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - SQ5
    #[inline(always)]
    pub fn sq5(&mut self) -> SQ5_W<'_, SQR2rs> {
        SQ5_W::new(self, 0)
    }
    ///Bits 6:10 - SQ6
    #[inline(always)]
    pub fn sq6(&mut self) -> SQ6_W<'_, SQR2rs> {
        SQ6_W::new(self, 6)
    }
    ///Bits 12:16 - SQ7
    #[inline(always)]
    pub fn sq7(&mut self) -> SQ7_W<'_, SQR2rs> {
        SQ7_W::new(self, 12)
    }
    ///Bits 18:22 - SQ8
    #[inline(always)]
    pub fn sq8(&mut self) -> SQ8_W<'_, SQR2rs> {
        SQ8_W::new(self, 18)
    }
    ///Bits 24:28 - SQ9
    #[inline(always)]
    pub fn sq9(&mut self) -> SQ9_W<'_, SQR2rs> {
        SQ9_W::new(self, 24)
    }
}
/**ADC regular sequence register 2

You can [`read`](crate::Reg::read) this register and get [`sqr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ADC:SQR2)*/
pub struct SQR2rs;
impl crate::RegisterSpec for SQR2rs {
    type Ux = u32;
}
///`read()` method returns [`sqr2::R`](R) reader structure
impl crate::Readable for SQR2rs {}
///`write(|w| ..)` method takes [`sqr2::W`](W) writer structure
impl crate::Writable for SQR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SQR2 to value 0
impl crate::Resettable for SQR2rs {}
