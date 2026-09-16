///Register `CCR6` reader
pub type R = crate::R<CCR6rs>;
///Register `CCR6` writer
pub type W = crate::W<CCR6rs>;
///Field `CCR6` reader - CCR6
pub type CCR6_R = crate::FieldReader<u16>;
///Field `CCR6` writer - CCR6
pub type CCR6_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - CCR6
    #[inline(always)]
    pub fn ccr6(&self) -> CCR6_R {
        CCR6_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR6").field("ccr6", &self.ccr6()).finish()
    }
}
impl W {
    ///Bits 0:15 - CCR6
    #[inline(always)]
    pub fn ccr6(&mut self) -> CCR6_W<'_, CCR6rs> {
        CCR6_W::new(self, 0)
    }
}
/**TIM1 capture/compare register 6

You can [`read`](crate::Reg::read) this register and get [`ccr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM1:CCR6)*/
pub struct CCR6rs;
impl crate::RegisterSpec for CCR6rs {
    type Ux = u16;
}
///`read()` method returns [`ccr6::R`](R) reader structure
impl crate::Readable for CCR6rs {}
///`write(|w| ..)` method takes [`ccr6::W`](W) writer structure
impl crate::Writable for CCR6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR6 to value 0
impl crate::Resettable for CCR6rs {}
