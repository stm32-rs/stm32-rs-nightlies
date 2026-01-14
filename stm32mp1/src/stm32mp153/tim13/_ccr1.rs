///Register `_CCR1` reader
pub type R = crate::R<_CCR1rs>;
///Register `_CCR1` writer
pub type W = crate::W<_CCR1rs>;
///Field `CCR1` reader - CCR1
pub type CCR1_R = crate::FieldReader<u16>;
///Field `CCR1` writer - CCR1
pub type CCR1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - CCR1
    #[inline(always)]
    pub fn ccr1(&self) -> CCR1_R {
        CCR1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_CCR1").field("ccr1", &self.ccr1()).finish()
    }
}
impl W {
    ///Bits 0:15 - CCR1
    #[inline(always)]
    pub fn ccr1(&mut self) -> CCR1_W<'_, _CCR1rs> {
        CCR1_W::new(self, 0)
    }
}
/**TIM13 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`_ccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM13:_CCR1)*/
pub struct _CCR1rs;
impl crate::RegisterSpec for _CCR1rs {
    type Ux = u16;
}
///`read()` method returns [`_ccr1::R`](R) reader structure
impl crate::Readable for _CCR1rs {}
///`write(|w| ..)` method takes [`_ccr1::W`](W) writer structure
impl crate::Writable for _CCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _CCR1 to value 0
impl crate::Resettable for _CCR1rs {}
