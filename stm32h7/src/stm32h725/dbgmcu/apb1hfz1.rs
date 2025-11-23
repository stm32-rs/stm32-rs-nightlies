///Register `APB1HFZ1` reader
pub type R = crate::R<APB1HFZ1rs>;
///Register `APB1HFZ1` writer
pub type W = crate::W<APB1HFZ1rs>;
///Field `TIM23` reader - TIM23 stop in debug
pub type TIM23_R = crate::BitReader;
///Field `TIM23` writer - TIM23 stop in debug
pub type TIM23_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM24` reader - TIM24 stop in debug
pub type TIM24_R = crate::BitReader;
///Field `TIM24` writer - TIM24 stop in debug
pub type TIM24_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 24 - TIM23 stop in debug
    #[inline(always)]
    pub fn tim23(&self) -> TIM23_R {
        TIM23_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TIM24 stop in debug
    #[inline(always)]
    pub fn tim24(&self) -> TIM24_R {
        TIM24_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1HFZ1")
            .field("tim23", &self.tim23())
            .field("tim24", &self.tim24())
            .finish()
    }
}
impl W {
    ///Bit 24 - TIM23 stop in debug
    #[inline(always)]
    pub fn tim23(&mut self) -> TIM23_W<'_, APB1HFZ1rs> {
        TIM23_W::new(self, 24)
    }
    ///Bit 25 - TIM24 stop in debug
    #[inline(always)]
    pub fn tim24(&mut self) -> TIM24_W<'_, APB1HFZ1rs> {
        TIM24_W::new(self, 25)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`apb1hfz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hfz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#DBGMCU:APB1HFZ1)*/
pub struct APB1HFZ1rs;
impl crate::RegisterSpec for APB1HFZ1rs {
    type Ux = u32;
}
///`read()` method returns [`apb1hfz1::R`](R) reader structure
impl crate::Readable for APB1HFZ1rs {}
///`write(|w| ..)` method takes [`apb1hfz1::W`](W) writer structure
impl crate::Writable for APB1HFZ1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1HFZ1 to value 0
impl crate::Resettable for APB1HFZ1rs {}
