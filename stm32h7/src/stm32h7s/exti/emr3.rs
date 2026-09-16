///Register `EMR3` reader
pub type R = crate::R<EMR3rs>;
///Register `EMR3` writer
pub type W = crate::W<EMR3rs>;
///Field `MR77` reader - CPU event mask on event input x+64
pub type MR77_R = crate::BitReader;
///Field `MR77` writer - CPU event mask on event input x+64
pub type MR77_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 13 - CPU event mask on event input x+64
    #[inline(always)]
    pub fn mr77(&self) -> MR77_R {
        MR77_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMR3").field("mr77", &self.mr77()).finish()
    }
}
impl W {
    ///Bit 13 - CPU event mask on event input x+64
    #[inline(always)]
    pub fn mr77(&mut self) -> MR77_W<'_, EMR3rs> {
        MR77_W::new(self, 13)
    }
}
/**EXTI event mask register

You can [`read`](crate::Reg::read) this register and get [`emr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#EXTI:EMR3)*/
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
