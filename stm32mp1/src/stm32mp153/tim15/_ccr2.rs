///Register `_CCR2` reader
pub type R = crate::R<_CCR2rs>;
///Register `_CCR2` writer
pub type W = crate::W<_CCR2rs>;
///Field `CCR2` reader - CCR2
pub type CCR2_R = crate::FieldReader<u16>;
///Field `CCR2` writer - CCR2
pub type CCR2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - CCR2
    #[inline(always)]
    pub fn ccr2(&self) -> CCR2_R {
        CCR2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_CCR2").field("ccr2", &self.ccr2()).finish()
    }
}
impl W {
    ///Bits 0:15 - CCR2
    #[inline(always)]
    pub fn ccr2(&mut self) -> CCR2_W<'_, _CCR2rs> {
        CCR2_W::new(self, 0)
    }
}
/**TIM15 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`_ccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM15:_CCR2)*/
pub struct _CCR2rs;
impl crate::RegisterSpec for _CCR2rs {
    type Ux = u16;
}
///`read()` method returns [`_ccr2::R`](R) reader structure
impl crate::Readable for _CCR2rs {}
///`write(|w| ..)` method takes [`_ccr2::W`](W) writer structure
impl crate::Writable for _CCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _CCR2 to value 0
impl crate::Resettable for _CCR2rs {}
