///Register `TIM8_CNT` reader
pub type R = crate::R<TIM8_CNTrs>;
///Register `TIM8_CNT` writer
pub type W = crate::W<TIM8_CNTrs>;
///Field `CNT` reader - CNT
pub type CNT_R = crate::FieldReader<u16>;
///Field `CNT` writer - CNT
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `UIFCPY` reader - UIFCPY
pub type UIFCPY_R = crate::BitReader;
impl R {
    ///Bits 0:15 - CNT
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 31 - UIFCPY
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM8_CNT")
            .field("cnt", &self.cnt())
            .field("uifcpy", &self.uifcpy())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - CNT
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<TIM8_CNTrs> {
        CNT_W::new(self, 0)
    }
}
/**TIM8 counter

You can [`read`](crate::Reg::read) this register and get [`tim8_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM8:TIM8_CNT)*/
pub struct TIM8_CNTrs;
impl crate::RegisterSpec for TIM8_CNTrs {
    type Ux = u32;
}
///`read()` method returns [`tim8_cnt::R`](R) reader structure
impl crate::Readable for TIM8_CNTrs {}
///`write(|w| ..)` method takes [`tim8_cnt::W`](W) writer structure
impl crate::Writable for TIM8_CNTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIM8_CNT to value 0
impl crate::Resettable for TIM8_CNTrs {
    const RESET_VALUE: u32 = 0;
}
