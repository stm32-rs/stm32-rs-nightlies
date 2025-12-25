///Register `IMR3` reader
pub type R = crate::R<IMR3rs>;
///Register `IMR3` writer
pub type W = crate::W<IMR3rs>;
///Field `IM64` reader - CPU wake-up with interrupt mask on event input x
pub type IM64_R = crate::BitReader;
///Field `IM64` writer - CPU wake-up with interrupt mask on event input x
pub type IM64_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM65` reader - CPU wake-up with interrupt mask on event input x
pub type IM65_R = crate::BitReader;
///Field `IM65` writer - CPU wake-up with interrupt mask on event input x
pub type IM65_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM66` reader - CPU wake-up with interrupt mask on event input x
pub type IM66_R = crate::BitReader;
///Field `IM66` writer - CPU wake-up with interrupt mask on event input x
pub type IM66_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM68` reader - CPU wake-up with interrupt mask on event input x
pub type IM68_R = crate::BitReader;
///Field `IM68` writer - CPU wake-up with interrupt mask on event input x
pub type IM68_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM69` reader - CPU wake-up with interrupt mask on event input x
pub type IM69_R = crate::BitReader;
///Field `IM69` writer - CPU wake-up with interrupt mask on event input x
pub type IM69_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM70` reader - CPU wake-up with interrupt mask on event input x
pub type IM70_R = crate::BitReader;
///Field `IM70` writer - CPU wake-up with interrupt mask on event input x
pub type IM70_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM71` reader - CPU wake-up with interrupt mask on event input x
pub type IM71_R = crate::BitReader;
///Field `IM71` writer - CPU wake-up with interrupt mask on event input x
pub type IM71_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM72` reader - CPU wake-up with interrupt mask on event input x
pub type IM72_R = crate::BitReader;
///Field `IM72` writer - CPU wake-up with interrupt mask on event input x
pub type IM72_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM73` reader - CPU wake-up with interrupt mask on event input x
pub type IM73_R = crate::BitReader;
///Field `IM73` writer - CPU wake-up with interrupt mask on event input x
pub type IM73_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM74` reader - CPU wake-up with interrupt mask on event input x
pub type IM74_R = crate::BitReader;
///Field `IM74` writer - CPU wake-up with interrupt mask on event input x
pub type IM74_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM77` reader - CPU wake-up with interrupt mask on event input 77
pub type IM77_R = crate::BitReader;
///Field `IM77` writer - CPU wake-up with interrupt mask on event input 77
pub type IM77_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im64(&self) -> IM64_R {
        IM64_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im65(&self) -> IM65_R {
        IM65_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im66(&self) -> IM66_R {
        IM66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im68(&self) -> IM68_R {
        IM68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im69(&self) -> IM69_R {
        IM69_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im70(&self) -> IM70_R {
        IM70_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im71(&self) -> IM71_R {
        IM71_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im72(&self) -> IM72_R {
        IM72_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im73(&self) -> IM73_R {
        IM73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im74(&self) -> IM74_R {
        IM74_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - CPU wake-up with interrupt mask on event input 77
    #[inline(always)]
    pub fn im77(&self) -> IM77_R {
        IM77_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR3")
            .field("im64", &self.im64())
            .field("im65", &self.im65())
            .field("im66", &self.im66())
            .field("im68", &self.im68())
            .field("im69", &self.im69())
            .field("im70", &self.im70())
            .field("im71", &self.im71())
            .field("im72", &self.im72())
            .field("im73", &self.im73())
            .field("im74", &self.im74())
            .field("im77", &self.im77())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im64(&mut self) -> IM64_W<'_, IMR3rs> {
        IM64_W::new(self, 0)
    }
    ///Bit 1 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im65(&mut self) -> IM65_W<'_, IMR3rs> {
        IM65_W::new(self, 1)
    }
    ///Bit 2 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im66(&mut self) -> IM66_W<'_, IMR3rs> {
        IM66_W::new(self, 2)
    }
    ///Bit 4 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im68(&mut self) -> IM68_W<'_, IMR3rs> {
        IM68_W::new(self, 4)
    }
    ///Bit 5 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im69(&mut self) -> IM69_W<'_, IMR3rs> {
        IM69_W::new(self, 5)
    }
    ///Bit 6 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im70(&mut self) -> IM70_W<'_, IMR3rs> {
        IM70_W::new(self, 6)
    }
    ///Bit 7 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im71(&mut self) -> IM71_W<'_, IMR3rs> {
        IM71_W::new(self, 7)
    }
    ///Bit 8 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im72(&mut self) -> IM72_W<'_, IMR3rs> {
        IM72_W::new(self, 8)
    }
    ///Bit 9 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im73(&mut self) -> IM73_W<'_, IMR3rs> {
        IM73_W::new(self, 9)
    }
    ///Bit 10 - CPU wake-up with interrupt mask on event input x
    #[inline(always)]
    pub fn im74(&mut self) -> IM74_W<'_, IMR3rs> {
        IM74_W::new(self, 10)
    }
    ///Bit 13 - CPU wake-up with interrupt mask on event input 77
    #[inline(always)]
    pub fn im77(&mut self) -> IM77_W<'_, IMR3rs> {
        IM77_W::new(self, 13)
    }
}
/**EXTI CPU wake-up with interrupt mask register 3

You can [`read`](crate::Reg::read) this register and get [`imr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#EXTI:IMR3)*/
pub struct IMR3rs;
impl crate::RegisterSpec for IMR3rs {
    type Ux = u32;
}
///`read()` method returns [`imr3::R`](R) reader structure
impl crate::Readable for IMR3rs {}
///`write(|w| ..)` method takes [`imr3::W`](W) writer structure
impl crate::Writable for IMR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IMR3 to value 0
impl crate::Resettable for IMR3rs {}
