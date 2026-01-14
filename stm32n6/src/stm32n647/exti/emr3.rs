///Register `EMR3` reader
pub type R = crate::R<EMR3rs>;
///Register `EMR3` writer
pub type W = crate::W<EMR3rs>;
///Field `EM64` reader - CPU wake-up with interrupt mask on event input x
pub type EM64_R = crate::BitReader;
///Field `EM64` writer - CPU wake-up with interrupt mask on event input x
pub type EM64_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM65` reader - CPU wake-up with interrupt mask on event input x
pub type EM65_R = crate::BitReader;
///Field `EM65` writer - CPU wake-up with interrupt mask on event input x
pub type EM65_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM66` reader - CPU wake-up with interrupt mask on event input x
pub type EM66_R = crate::BitReader;
///Field `EM66` writer - CPU wake-up with interrupt mask on event input x
pub type EM66_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM68` reader - CPU wake-up with interrupt mask on event input x
pub type EM68_R = crate::BitReader;
///Field `EM68` writer - CPU wake-up with interrupt mask on event input x
pub type EM68_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM69` reader - CPU wake-up with interrupt mask on event input x
pub type EM69_R = crate::BitReader;
///Field `EM69` writer - CPU wake-up with interrupt mask on event input x
pub type EM69_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM70` reader - CPU wake-up with interrupt mask on event input x
pub type EM70_R = crate::BitReader;
///Field `EM70` writer - CPU wake-up with interrupt mask on event input x
pub type EM70_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM71` reader - CPU wake-up with interrupt mask on event input x
pub type EM71_R = crate::BitReader;
///Field `EM71` writer - CPU wake-up with interrupt mask on event input x
pub type EM71_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM72` reader - CPU wake-up with interrupt mask on event input x
pub type EM72_R = crate::BitReader;
///Field `EM72` writer - CPU wake-up with interrupt mask on event input x
pub type EM72_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM73` reader - CPU wake-up with interrupt mask on event input x
pub type EM73_R = crate::BitReader;
///Field `EM73` writer - CPU wake-up with interrupt mask on event input x
pub type EM73_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM74` reader - CPU wake-up with interrupt mask on event input x
pub type EM74_R = crate::BitReader;
///Field `EM74` writer - CPU wake-up with interrupt mask on event input x
pub type EM74_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM77` reader - CPU wake-up with event on event input 77
pub type EM77_R = crate::BitReader;
///Field `EM77` writer - CPU wake-up with event on event input 77
pub type EM77_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em64(&self) -> EM64_R {
        EM64_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em65(&self) -> EM65_R {
        EM65_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em66(&self) -> EM66_R {
        EM66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em68(&self) -> EM68_R {
        EM68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em69(&self) -> EM69_R {
        EM69_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em70(&self) -> EM70_R {
        EM70_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em71(&self) -> EM71_R {
        EM71_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em72(&self) -> EM72_R {
        EM72_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em73(&self) -> EM73_R {
        EM73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em74(&self) -> EM74_R {
        EM74_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - CPU wake-up with event on event input 77
    #[inline(always)]
    pub fn em77(&self) -> EM77_R {
        EM77_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMR3")
            .field("em64", &self.em64())
            .field("em65", &self.em65())
            .field("em66", &self.em66())
            .field("em68", &self.em68())
            .field("em69", &self.em69())
            .field("em70", &self.em70())
            .field("em71", &self.em71())
            .field("em72", &self.em72())
            .field("em73", &self.em73())
            .field("em74", &self.em74())
            .field("em77", &self.em77())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em64(&mut self) -> EM64_W<'_, EMR3rs> {
        EM64_W::new(self, 0)
    }
    ///Bit 1 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em65(&mut self) -> EM65_W<'_, EMR3rs> {
        EM65_W::new(self, 1)
    }
    ///Bit 2 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em66(&mut self) -> EM66_W<'_, EMR3rs> {
        EM66_W::new(self, 2)
    }
    ///Bit 4 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em68(&mut self) -> EM68_W<'_, EMR3rs> {
        EM68_W::new(self, 4)
    }
    ///Bit 5 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em69(&mut self) -> EM69_W<'_, EMR3rs> {
        EM69_W::new(self, 5)
    }
    ///Bit 6 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em70(&mut self) -> EM70_W<'_, EMR3rs> {
        EM70_W::new(self, 6)
    }
    ///Bit 7 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em71(&mut self) -> EM71_W<'_, EMR3rs> {
        EM71_W::new(self, 7)
    }
    ///Bit 8 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em72(&mut self) -> EM72_W<'_, EMR3rs> {
        EM72_W::new(self, 8)
    }
    ///Bit 9 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em73(&mut self) -> EM73_W<'_, EMR3rs> {
        EM73_W::new(self, 9)
    }
    ///Bit 10 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn em74(&mut self) -> EM74_W<'_, EMR3rs> {
        EM74_W::new(self, 10)
    }
    ///Bit 13 - CPU wake-up with event on event input 77
    #[inline(always)]
    pub fn em77(&mut self) -> EM77_W<'_, EMR3rs> {
        EM77_W::new(self, 13)
    }
}
/**EXTI CPU wake-up with event mask register 3

You can [`read`](crate::Reg::read) this register and get [`emr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#EXTI:EMR3)*/
pub struct EMR3rs;
impl crate::RegisterSpec for EMR3rs {
    type Ux = u32;
}
///`read()` method returns [`emr3::R`](R) reader structure
impl crate::Readable for EMR3rs {}
///`write(|w| ..)` method takes [`emr3::W`](W) writer structure
impl crate::Writable for EMR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EMR3 to value 0
impl crate::Resettable for EMR3rs {}
