///Register `APB2FZ` reader
pub type R = crate::R<APB2FZrs>;
///Register `APB2FZ` writer
pub type W = crate::W<APB2FZrs>;
///Field `TIM1` reader - TIM1 stop in debug
pub type TIM1_R = crate::BitReader;
///Field `TIM1` writer - TIM1 stop in debug
pub type TIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15` reader - TIM15 stop in debug
pub type TIM15_R = crate::BitReader;
///Field `TIM15` writer - TIM15 stop in debug
pub type TIM15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16` reader - TIM16 stop in debug
pub type TIM16_R = crate::BitReader;
///Field `TIM16` writer - TIM16 stop in debug
pub type TIM16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17` reader - TIM17 stop in debug
pub type TIM17_R = crate::BitReader;
///Field `TIM17` writer - TIM17 stop in debug
pub type TIM17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM9` reader - TIM9 stop in debug
pub type TIM9_R = crate::BitReader;
///Field `TIM9` writer - TIM9 stop in debug
pub type TIM9_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM1 stop in debug
    #[inline(always)]
    pub fn tim1(&self) -> TIM1_R {
        TIM1_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - TIM15 stop in debug
    #[inline(always)]
    pub fn tim15(&self) -> TIM15_R {
        TIM15_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 stop in debug
    #[inline(always)]
    pub fn tim16(&self) -> TIM16_R {
        TIM16_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 stop in debug
    #[inline(always)]
    pub fn tim17(&self) -> TIM17_R {
        TIM17_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TIM9 stop in debug
    #[inline(always)]
    pub fn tim9(&self) -> TIM9_R {
        TIM9_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2FZ")
            .field("tim1", &self.tim1())
            .field("tim15", &self.tim15())
            .field("tim16", &self.tim16())
            .field("tim17", &self.tim17())
            .field("tim9", &self.tim9())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM1 stop in debug
    #[inline(always)]
    pub fn tim1(&mut self) -> TIM1_W<'_, APB2FZrs> {
        TIM1_W::new(self, 0)
    }
    ///Bit 16 - TIM15 stop in debug
    #[inline(always)]
    pub fn tim15(&mut self) -> TIM15_W<'_, APB2FZrs> {
        TIM15_W::new(self, 16)
    }
    ///Bit 17 - TIM16 stop in debug
    #[inline(always)]
    pub fn tim16(&mut self) -> TIM16_W<'_, APB2FZrs> {
        TIM16_W::new(self, 17)
    }
    ///Bit 18 - TIM17 stop in debug
    #[inline(always)]
    pub fn tim17(&mut self) -> TIM17_W<'_, APB2FZrs> {
        TIM17_W::new(self, 18)
    }
    ///Bit 19 - TIM9 stop in debug
    #[inline(always)]
    pub fn tim9(&mut self) -> TIM9_W<'_, APB2FZrs> {
        TIM9_W::new(self, 19)
    }
}
/**DBGMCU APB2 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb2fz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DBGMCU:APB2FZ)*/
pub struct APB2FZrs;
impl crate::RegisterSpec for APB2FZrs {
    type Ux = u32;
}
///`read()` method returns [`apb2fz::R`](R) reader structure
impl crate::Readable for APB2FZrs {}
///`write(|w| ..)` method takes [`apb2fz::W`](W) writer structure
impl crate::Writable for APB2FZrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2FZ to value 0
impl crate::Resettable for APB2FZrs {}
