///Register `CRR6` reader
pub type R = crate::R<CRR6rs>;
///Register `CRR6` writer
pub type W = crate::W<CRR6rs>;
///Field `CCR6` reader - Capture/Compare 6 value
pub type CCR6_R = crate::FieldReader<u16>;
///Field `CCR6` writer - Capture/Compare 6 value
pub type CCR6_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Capture/Compare 6 value
    #[inline(always)]
    pub fn ccr6(&self) -> CCR6_R {
        CCR6_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRR6").field("ccr6", &self.ccr6()).finish()
    }
}
impl W {
    ///Bits 0:15 - Capture/Compare 6 value
    #[inline(always)]
    pub fn ccr6(&mut self) -> CCR6_W<CRR6rs> {
        CCR6_W::new(self, 0)
    }
}
/**capture/compare register 6

You can [`read`](crate::Reg::read) this register and get [`crr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#TIM8:CRR6)*/
pub struct CRR6rs;
impl crate::RegisterSpec for CRR6rs {
    type Ux = u32;
}
///`read()` method returns [`crr6::R`](R) reader structure
impl crate::Readable for CRR6rs {}
///`write(|w| ..)` method takes [`crr6::W`](W) writer structure
impl crate::Writable for CRR6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRR6 to value 0
impl crate::Resettable for CRR6rs {}
