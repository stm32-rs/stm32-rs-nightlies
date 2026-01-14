///Register `IMR3` reader
pub type R = crate::R<IMR3rs>;
///Register `IMR3` writer
pub type W = crate::W<IMR3rs>;
///Field `MR77` reader - CPU interrupt mask on direct event input x+64
pub type MR77_R = crate::BitReader;
///Field `MR77` writer - CPU interrupt mask on direct event input x+64
pub type MR77_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 13 - CPU interrupt mask on direct event input x+64
    #[inline(always)]
    pub fn mr77(&self) -> MR77_R {
        MR77_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR3").field("mr77", &self.mr77()).finish()
    }
}
impl W {
    ///Bit 13 - CPU interrupt mask on direct event input x+64
    #[inline(always)]
    pub fn mr77(&mut self) -> MR77_W<'_, IMR3rs> {
        MR77_W::new(self, 13)
    }
}
/**EXTI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`imr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#EXTI:IMR3)*/
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
///`reset()` method sets IMR3 to value 0x0f8b_ffff
impl crate::Resettable for IMR3rs {
    const RESET_VALUE: u32 = 0x0f8b_ffff;
}
