///Register `CCR5` reader
pub type R = crate::R<CCR5rs>;
///Register `CCR5` writer
pub type W = crate::W<CCR5rs>;
///Field `CCR5` reader - Capture/compare 5 value
pub type CCR5_R = crate::FieldReader<u32>;
///Field `CCR5` writer - Capture/compare 5 value
pub type CCR5_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
///Field `GC5C1` reader - Group channel 5 and channel 1
pub type GC5C1_R = crate::BitReader;
///Field `GC5C1` writer - Group channel 5 and channel 1
pub type GC5C1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GC5C2` reader - Group channel 5 and channel 2
pub type GC5C2_R = crate::BitReader;
///Field `GC5C2` writer - Group channel 5 and channel 2
pub type GC5C2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GC5C3` reader - Group channel 5 and channel 3
pub type GC5C3_R = crate::BitReader;
///Field `GC5C3` writer - Group channel 5 and channel 3
pub type GC5C3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:19 - Capture/compare 5 value
    #[inline(always)]
    pub fn ccr5(&self) -> CCR5_R {
        CCR5_R::new(self.bits & 0x000f_ffff)
    }
    ///Bit 29 - Group channel 5 and channel 1
    #[inline(always)]
    pub fn gc5c1(&self) -> GC5C1_R {
        GC5C1_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Group channel 5 and channel 2
    #[inline(always)]
    pub fn gc5c2(&self) -> GC5C2_R {
        GC5C2_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Group channel 5 and channel 3
    #[inline(always)]
    pub fn gc5c3(&self) -> GC5C3_R {
        GC5C3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR5")
            .field("ccr5", &self.ccr5())
            .field("gc5c1", &self.gc5c1())
            .field("gc5c2", &self.gc5c2())
            .field("gc5c3", &self.gc5c3())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - Capture/compare 5 value
    #[inline(always)]
    pub fn ccr5(&mut self) -> CCR5_W<CCR5rs> {
        CCR5_W::new(self, 0)
    }
    ///Bit 29 - Group channel 5 and channel 1
    #[inline(always)]
    pub fn gc5c1(&mut self) -> GC5C1_W<CCR5rs> {
        GC5C1_W::new(self, 29)
    }
    ///Bit 30 - Group channel 5 and channel 2
    #[inline(always)]
    pub fn gc5c2(&mut self) -> GC5C2_W<CCR5rs> {
        GC5C2_W::new(self, 30)
    }
    ///Bit 31 - Group channel 5 and channel 3
    #[inline(always)]
    pub fn gc5c3(&mut self) -> GC5C3_W<CCR5rs> {
        GC5C3_W::new(self, 31)
    }
}
/**TIM8 capture/compare register 5

You can [`read`](crate::Reg::read) this register and get [`ccr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#TIM8:CCR5)*/
pub struct CCR5rs;
impl crate::RegisterSpec for CCR5rs {
    type Ux = u32;
}
///`read()` method returns [`ccr5::R`](R) reader structure
impl crate::Readable for CCR5rs {}
///`write(|w| ..)` method takes [`ccr5::W`](W) writer structure
impl crate::Writable for CCR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR5 to value 0
impl crate::Resettable for CCR5rs {}
