///Register `RTSR3` reader
pub type R = crate::R<RTSR3rs>;
///Register `RTSR3` writer
pub type W = crate::W<RTSR3rs>;
///Field `RT66` reader - Rising trigger event configuration bit of configurable event input 66
pub type RT66_R = crate::BitReader;
///Field `RT66` writer - Rising trigger event configuration bit of configurable event input 66
pub type RT66_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT68` reader - Rising trigger event configuration bit of configurable event input x
pub type RT68_R = crate::BitReader;
///Field `RT68` writer - Rising trigger event configuration bit of configurable event input x
pub type RT68_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT69` reader - Rising trigger event configuration bit of configurable event input x
pub type RT69_R = crate::BitReader;
///Field `RT69` writer - Rising trigger event configuration bit of configurable event input x
pub type RT69_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT70` reader - Rising trigger event configuration bit of configurable event input x
pub type RT70_R = crate::BitReader;
///Field `RT70` writer - Rising trigger event configuration bit of configurable event input x
pub type RT70_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT71` reader - Rising trigger event configuration bit of configurable event input x
pub type RT71_R = crate::BitReader;
///Field `RT71` writer - Rising trigger event configuration bit of configurable event input x
pub type RT71_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT72` reader - Rising trigger event configuration bit of configurable event input x
pub type RT72_R = crate::BitReader;
///Field `RT72` writer - Rising trigger event configuration bit of configurable event input x
pub type RT72_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT73` reader - Rising trigger event configuration bit of configurable event input x
pub type RT73_R = crate::BitReader;
///Field `RT73` writer - Rising trigger event configuration bit of configurable event input x
pub type RT73_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT74` reader - Rising trigger event configuration bit of configurable event input x
pub type RT74_R = crate::BitReader;
///Field `RT74` writer - Rising trigger event configuration bit of configurable event input x
pub type RT74_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Rising trigger event configuration bit of configurable event input 66
    #[inline(always)]
    pub fn rt66(&self) -> RT66_R {
        RT66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt68(&self) -> RT68_R {
        RT68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt69(&self) -> RT69_R {
        RT69_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt70(&self) -> RT70_R {
        RT70_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt71(&self) -> RT71_R {
        RT71_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt72(&self) -> RT72_R {
        RT72_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt73(&self) -> RT73_R {
        RT73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt74(&self) -> RT74_R {
        RT74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTSR3")
            .field("rt66", &self.rt66())
            .field("rt68", &self.rt68())
            .field("rt69", &self.rt69())
            .field("rt70", &self.rt70())
            .field("rt71", &self.rt71())
            .field("rt72", &self.rt72())
            .field("rt73", &self.rt73())
            .field("rt74", &self.rt74())
            .finish()
    }
}
impl W {
    ///Bit 2 - Rising trigger event configuration bit of configurable event input 66
    #[inline(always)]
    pub fn rt66(&mut self) -> RT66_W<'_, RTSR3rs> {
        RT66_W::new(self, 2)
    }
    ///Bit 4 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt68(&mut self) -> RT68_W<'_, RTSR3rs> {
        RT68_W::new(self, 4)
    }
    ///Bit 5 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt69(&mut self) -> RT69_W<'_, RTSR3rs> {
        RT69_W::new(self, 5)
    }
    ///Bit 6 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt70(&mut self) -> RT70_W<'_, RTSR3rs> {
        RT70_W::new(self, 6)
    }
    ///Bit 7 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt71(&mut self) -> RT71_W<'_, RTSR3rs> {
        RT71_W::new(self, 7)
    }
    ///Bit 8 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt72(&mut self) -> RT72_W<'_, RTSR3rs> {
        RT72_W::new(self, 8)
    }
    ///Bit 9 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt73(&mut self) -> RT73_W<'_, RTSR3rs> {
        RT73_W::new(self, 9)
    }
    ///Bit 10 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt74(&mut self) -> RT74_W<'_, RTSR3rs> {
        RT74_W::new(self, 10)
    }
}
/**EXTI rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#EXTI:RTSR3)*/
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
