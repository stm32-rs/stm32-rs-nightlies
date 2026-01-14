///Register `SQR3` reader
pub type R = crate::R<SQR3rs>;
///Register `SQR3` writer
pub type W = crate::W<SQR3rs>;
///Field `SQ10` reader - SQ10
pub type SQ10_R = crate::FieldReader;
///Field `SQ10` writer - SQ10
pub type SQ10_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ11` reader - SQ11
pub type SQ11_R = crate::FieldReader;
///Field `SQ11` writer - SQ11
pub type SQ11_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ12` reader - SQ12
pub type SQ12_R = crate::FieldReader;
///Field `SQ12` writer - SQ12
pub type SQ12_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ13` reader - SQ13
pub type SQ13_R = crate::FieldReader;
///Field `SQ13` writer - SQ13
pub type SQ13_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ14` reader - SQ14
pub type SQ14_R = crate::FieldReader;
///Field `SQ14` writer - SQ14
pub type SQ14_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - SQ10
    #[inline(always)]
    pub fn sq10(&self) -> SQ10_R {
        SQ10_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:10 - SQ11
    #[inline(always)]
    pub fn sq11(&self) -> SQ11_R {
        SQ11_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 12:16 - SQ12
    #[inline(always)]
    pub fn sq12(&self) -> SQ12_R {
        SQ12_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    ///Bits 18:22 - SQ13
    #[inline(always)]
    pub fn sq13(&self) -> SQ13_R {
        SQ13_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bits 24:28 - SQ14
    #[inline(always)]
    pub fn sq14(&self) -> SQ14_R {
        SQ14_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SQR3")
            .field("sq10", &self.sq10())
            .field("sq11", &self.sq11())
            .field("sq12", &self.sq12())
            .field("sq13", &self.sq13())
            .field("sq14", &self.sq14())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - SQ10
    #[inline(always)]
    pub fn sq10(&mut self) -> SQ10_W<'_, SQR3rs> {
        SQ10_W::new(self, 0)
    }
    ///Bits 6:10 - SQ11
    #[inline(always)]
    pub fn sq11(&mut self) -> SQ11_W<'_, SQR3rs> {
        SQ11_W::new(self, 6)
    }
    ///Bits 12:16 - SQ12
    #[inline(always)]
    pub fn sq12(&mut self) -> SQ12_W<'_, SQR3rs> {
        SQ12_W::new(self, 12)
    }
    ///Bits 18:22 - SQ13
    #[inline(always)]
    pub fn sq13(&mut self) -> SQ13_W<'_, SQR3rs> {
        SQ13_W::new(self, 18)
    }
    ///Bits 24:28 - SQ14
    #[inline(always)]
    pub fn sq14(&mut self) -> SQ14_W<'_, SQR3rs> {
        SQ14_W::new(self, 24)
    }
}
/**ADC regular sequence register 3

You can [`read`](crate::Reg::read) this register and get [`sqr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ADC2:SQR3)*/
pub struct SQR3rs;
impl crate::RegisterSpec for SQR3rs {
    type Ux = u32;
}
///`read()` method returns [`sqr3::R`](R) reader structure
impl crate::Readable for SQR3rs {}
///`write(|w| ..)` method takes [`sqr3::W`](W) writer structure
impl crate::Writable for SQR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SQR3 to value 0
impl crate::Resettable for SQR3rs {}
