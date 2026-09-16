///Register `RTSR3` reader
pub type R = crate::R<RTSR3rs>;
///Register `RTSR3` writer
pub type W = crate::W<RTSR3rs>;
///Field `RT65` reader - RT65
pub type RT65_R = crate::BitReader;
///Field `RT65` writer - RT65
pub type RT65_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT66` reader - RT66
pub type RT66_R = crate::BitReader;
///Field `RT66` writer - RT66
pub type RT66_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT68` reader - RT68
pub type RT68_R = crate::BitReader;
///Field `RT68` writer - RT68
pub type RT68_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT73` reader - RT73
pub type RT73_R = crate::BitReader;
///Field `RT73` writer - RT73
pub type RT73_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT74` reader - RT74
pub type RT74_R = crate::BitReader;
///Field `RT74` writer - RT74
pub type RT74_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - RT65
    #[inline(always)]
    pub fn rt65(&self) -> RT65_R {
        RT65_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RT66
    #[inline(always)]
    pub fn rt66(&self) -> RT66_R {
        RT66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - RT68
    #[inline(always)]
    pub fn rt68(&self) -> RT68_R {
        RT68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 9 - RT73
    #[inline(always)]
    pub fn rt73(&self) -> RT73_R {
        RT73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RT74
    #[inline(always)]
    pub fn rt74(&self) -> RT74_R {
        RT74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTSR3")
            .field("rt65", &self.rt65())
            .field("rt66", &self.rt66())
            .field("rt68", &self.rt68())
            .field("rt73", &self.rt73())
            .field("rt74", &self.rt74())
            .finish()
    }
}
impl W {
    ///Bit 1 - RT65
    #[inline(always)]
    pub fn rt65(&mut self) -> RT65_W<'_, RTSR3rs> {
        RT65_W::new(self, 1)
    }
    ///Bit 2 - RT66
    #[inline(always)]
    pub fn rt66(&mut self) -> RT66_W<'_, RTSR3rs> {
        RT66_W::new(self, 2)
    }
    ///Bit 4 - RT68
    #[inline(always)]
    pub fn rt68(&mut self) -> RT68_W<'_, RTSR3rs> {
        RT68_W::new(self, 4)
    }
    ///Bit 9 - RT73
    #[inline(always)]
    pub fn rt73(&mut self) -> RT73_W<'_, RTSR3rs> {
        RT73_W::new(self, 9)
    }
    ///Bit 10 - RT74
    #[inline(always)]
    pub fn rt74(&mut self) -> RT74_W<'_, RTSR3rs> {
        RT74_W::new(self, 10)
    }
}
/**Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`rtsr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:RTSR3)*/
pub struct RTSR3rs;
impl crate::RegisterSpec for RTSR3rs {
    type Ux = u32;
}
///`read()` method returns [`rtsr3::R`](R) reader structure
impl crate::Readable for RTSR3rs {}
///`write(|w| ..)` method takes [`rtsr3::W`](W) writer structure
impl crate::Writable for RTSR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTSR3 to value 0
impl crate::Resettable for RTSR3rs {}
