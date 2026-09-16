///Register `EMR3` reader
pub type R = crate::R<EMR3rs>;
///Register `EMR3` writer
pub type W = crate::W<EMR3rs>;
///Field `EM66` reader - EM66
pub type EM66_R = crate::BitReader;
///Field `EM66` writer - EM66
pub type EM66_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - EM66
    #[inline(always)]
    pub fn em66(&self) -> EM66_R {
        EM66_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMR3").field("em66", &self.em66()).finish()
    }
}
impl W {
    ///Bit 2 - EM66
    #[inline(always)]
    pub fn em66(&mut self) -> EM66_W<'_, EMR3rs> {
        EM66_W::new(self, 2)
    }
}
/**EXTI CPU wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`emr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:EMR3)*/
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
