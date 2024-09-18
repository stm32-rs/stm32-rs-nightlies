///Register `CCR6` reader
pub type R = crate::R<CCR6rs>;
///Register `CCR6` writer
pub type W = crate::W<CCR6rs>;
///Field `CCR` reader - Capture/Compare value
pub type CCR_R = crate::FieldReader<u32>;
///Field `CCR` writer - Capture/Compare value
pub type CCR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Capture/Compare value
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR6").field("ccr", &self.ccr()).finish()
    }
}
impl W {
    ///Bits 0:31 - Capture/Compare value
    #[inline(always)]
    #[must_use]
    pub fn ccr(&mut self) -> CCR_W<CCR6rs> {
        CCR_W::new(self, 0)
    }
}
/**capture/compare register

You can [`read`](crate::Reg::read) this register and get [`ccr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483xx.html#TIM2:CCR6)*/
pub struct CCR6rs;
impl crate::RegisterSpec for CCR6rs {
    type Ux = u32;
}
///`read()` method returns [`ccr6::R`](R) reader structure
impl crate::Readable for CCR6rs {}
///`write(|w| ..)` method takes [`ccr6::W`](W) writer structure
impl crate::Writable for CCR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCR6 to value 0
impl crate::Resettable for CCR6rs {
    const RESET_VALUE: u32 = 0;
}
