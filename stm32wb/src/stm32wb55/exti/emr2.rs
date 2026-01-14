///Register `EMR2` reader
pub type R = crate::R<EMR2rs>;
///Register `EMR2` writer
pub type W = crate::W<EMR2rs>;
///Field `EM` reader - CPU(m) Wakeup with event generation Mask on Event input
pub type EM_R = crate::FieldReader;
///Field `EM` writer - CPU(m) Wakeup with event generation Mask on Event input
pub type EM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 8:9 - CPU(m) Wakeup with event generation Mask on Event input
    #[inline(always)]
    pub fn em(&self) -> EM_R {
        EM_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMR2").field("em", &self.em()).finish()
    }
}
impl W {
    ///Bits 8:9 - CPU(m) Wakeup with event generation Mask on Event input
    #[inline(always)]
    pub fn em(&mut self) -> EM_W<'_, EMR2rs> {
        EM_W::new(self, 8)
    }
}
/**CPUm wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`emr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:EMR2)*/
pub struct EMR2rs;
impl crate::RegisterSpec for EMR2rs {
    type Ux = u32;
}
///`read()` method returns [`emr2::R`](R) reader structure
impl crate::Readable for EMR2rs {}
///`write(|w| ..)` method takes [`emr2::W`](W) writer structure
impl crate::Writable for EMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EMR2 to value 0
impl crate::Resettable for EMR2rs {}
