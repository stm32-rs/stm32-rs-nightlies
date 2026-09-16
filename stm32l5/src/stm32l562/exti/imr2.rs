///Register `IMR2` reader
pub type R = crate::R<IMR2rs>;
///Register `IMR2` writer
pub type W = crate::W<IMR2rs>;
///Field `IM32` reader - CPU wakeup with interrupt mask on event input
pub type IM32_R = crate::BitReader;
///Field `IM32` writer - CPU wakeup with interrupt mask on event input
pub type IM32_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM33` reader - CPU wakeup with interrupt mask on event input
pub type IM33_R = crate::BitReader;
///Field `IM33` writer - CPU wakeup with interrupt mask on event input
pub type IM33_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM34` reader - CPU wakeup with interrupt mask on event input
pub type IM34_R = crate::BitReader;
///Field `IM34` writer - CPU wakeup with interrupt mask on event input
pub type IM34_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM35` reader - CPU wakeup with interrupt mask on event input
pub type IM35_R = crate::BitReader;
///Field `IM35` writer - CPU wakeup with interrupt mask on event input
pub type IM35_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM36` reader - CPU wakeup with interrupt mask on event input
pub type IM36_R = crate::BitReader;
///Field `IM36` writer - CPU wakeup with interrupt mask on event input
pub type IM36_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM37` reader - CPU wakeup with interrupt mask on event input
pub type IM37_R = crate::BitReader;
///Field `IM37` writer - CPU wakeup with interrupt mask on event input
pub type IM37_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM38` reader - CPU wakeup with interrupt mask on event input
pub type IM38_R = crate::BitReader;
///Field `IM38` writer - CPU wakeup with interrupt mask on event input
pub type IM38_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM40` reader - CPU wakeup with interrupt mask on event input
pub type IM40_R = crate::BitReader;
///Field `IM40` writer - CPU wakeup with interrupt mask on event input
pub type IM40_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM41` reader - CPU wakeup with interrupt mask on event input
pub type IM41_R = crate::BitReader;
///Field `IM41` writer - CPU wakeup with interrupt mask on event input
pub type IM41_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM42` reader - CPU wakeup with interrupt mask on event input
pub type IM42_R = crate::BitReader;
///Field `IM42` writer - CPU wakeup with interrupt mask on event input
pub type IM42_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im32(&self) -> IM32_R {
        IM32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im33(&self) -> IM33_R {
        IM33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im34(&self) -> IM34_R {
        IM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im35(&self) -> IM35_R {
        IM35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im36(&self) -> IM36_R {
        IM36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im37(&self) -> IM37_R {
        IM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im38(&self) -> IM38_R {
        IM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im40(&self) -> IM40_R {
        IM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im41(&self) -> IM41_R {
        IM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im42(&self) -> IM42_R {
        IM42_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR2")
            .field("im32", &self.im32())
            .field("im33", &self.im33())
            .field("im34", &self.im34())
            .field("im35", &self.im35())
            .field("im36", &self.im36())
            .field("im37", &self.im37())
            .field("im38", &self.im38())
            .field("im40", &self.im40())
            .field("im41", &self.im41())
            .field("im42", &self.im42())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im32(&mut self) -> IM32_W<'_, IMR2rs> {
        IM32_W::new(self, 0)
    }
    ///Bit 1 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im33(&mut self) -> IM33_W<'_, IMR2rs> {
        IM33_W::new(self, 1)
    }
    ///Bit 2 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im34(&mut self) -> IM34_W<'_, IMR2rs> {
        IM34_W::new(self, 2)
    }
    ///Bit 3 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im35(&mut self) -> IM35_W<'_, IMR2rs> {
        IM35_W::new(self, 3)
    }
    ///Bit 4 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im36(&mut self) -> IM36_W<'_, IMR2rs> {
        IM36_W::new(self, 4)
    }
    ///Bit 5 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im37(&mut self) -> IM37_W<'_, IMR2rs> {
        IM37_W::new(self, 5)
    }
    ///Bit 6 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im38(&mut self) -> IM38_W<'_, IMR2rs> {
        IM38_W::new(self, 6)
    }
    ///Bit 8 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im40(&mut self) -> IM40_W<'_, IMR2rs> {
        IM40_W::new(self, 8)
    }
    ///Bit 9 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im41(&mut self) -> IM41_W<'_, IMR2rs> {
        IM41_W::new(self, 9)
    }
    ///Bit 10 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im42(&mut self) -> IM42_W<'_, IMR2rs> {
        IM42_W::new(self, 10)
    }
}
/**EXTI CPUm wakeup with interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`imr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#EXTI:IMR2)*/
pub struct IMR2rs;
impl crate::RegisterSpec for IMR2rs {
    type Ux = u32;
}
///`read()` method returns [`imr2::R`](R) reader structure
impl crate::Readable for IMR2rs {}
///`write(|w| ..)` method takes [`imr2::W`](W) writer structure
impl crate::Writable for IMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IMR2 to value 0x0787
impl crate::Resettable for IMR2rs {
    const RESET_VALUE: u32 = 0x0787;
}
