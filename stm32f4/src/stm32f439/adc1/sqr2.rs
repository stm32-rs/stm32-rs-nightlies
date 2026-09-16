///Register `SQR2` reader
pub type R = crate::R<SQR2rs>;
///Register `SQR2` writer
pub type W = crate::W<SQR2rs>;
///Field `SQ7` reader - 7th conversion in regular sequence
pub type SQ7_R = crate::FieldReader;
///Field `SQ7` writer - 7th conversion in regular sequence
pub type SQ7_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ8` reader - 8th conversion in regular sequence
pub type SQ8_R = crate::FieldReader;
///Field `SQ8` writer - 8th conversion in regular sequence
pub type SQ8_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ9` reader - 9th conversion in regular sequence
pub type SQ9_R = crate::FieldReader;
///Field `SQ9` writer - 9th conversion in regular sequence
pub type SQ9_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ10` reader - 10th conversion in regular sequence
pub type SQ10_R = crate::FieldReader;
///Field `SQ10` writer - 10th conversion in regular sequence
pub type SQ10_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ11` reader - 11th conversion in regular sequence
pub type SQ11_R = crate::FieldReader;
///Field `SQ11` writer - 11th conversion in regular sequence
pub type SQ11_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ12` reader - 12th conversion in regular sequence
pub type SQ12_R = crate::FieldReader;
///Field `SQ12` writer - 12th conversion in regular sequence
pub type SQ12_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - 7th conversion in regular sequence
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - 8th conversion in regular sequence
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - 9th conversion in regular sequence
    #[inline(always)]
    pub fn sq9(&self) -> SQ9_R {
        SQ9_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19 - 10th conversion in regular sequence
    #[inline(always)]
    pub fn sq10(&self) -> SQ10_R {
        SQ10_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 20:24 - 11th conversion in regular sequence
    #[inline(always)]
    pub fn sq11(&self) -> SQ11_R {
        SQ11_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bits 25:29 - 12th conversion in regular sequence
    #[inline(always)]
    pub fn sq12(&self) -> SQ12_R {
        SQ12_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SQR2")
            .field("sq12", &self.sq12())
            .field("sq11", &self.sq11())
            .field("sq10", &self.sq10())
            .field("sq9", &self.sq9())
            .field("sq8", &self.sq8())
            .field("sq7", &self.sq7())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - 7th conversion in regular sequence
    #[inline(always)]
    pub fn sq7(&mut self) -> SQ7_W<'_, SQR2rs> {
        SQ7_W::new(self, 0)
    }
    ///Bits 5:9 - 8th conversion in regular sequence
    #[inline(always)]
    pub fn sq8(&mut self) -> SQ8_W<'_, SQR2rs> {
        SQ8_W::new(self, 5)
    }
    ///Bits 10:14 - 9th conversion in regular sequence
    #[inline(always)]
    pub fn sq9(&mut self) -> SQ9_W<'_, SQR2rs> {
        SQ9_W::new(self, 10)
    }
    ///Bits 15:19 - 10th conversion in regular sequence
    #[inline(always)]
    pub fn sq10(&mut self) -> SQ10_W<'_, SQR2rs> {
        SQ10_W::new(self, 15)
    }
    ///Bits 20:24 - 11th conversion in regular sequence
    #[inline(always)]
    pub fn sq11(&mut self) -> SQ11_W<'_, SQR2rs> {
        SQ11_W::new(self, 20)
    }
    ///Bits 25:29 - 12th conversion in regular sequence
    #[inline(always)]
    pub fn sq12(&mut self) -> SQ12_W<'_, SQR2rs> {
        SQ12_W::new(self, 25)
    }
}
/**regular sequence register 2

You can [`read`](crate::Reg::read) this register and get [`sqr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#ADC1:SQR2)*/
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
