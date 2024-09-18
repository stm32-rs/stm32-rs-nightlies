///Register `TIM13_CNT` reader
pub type R = crate::R<TIM13_CNTrs>;
///Register `TIM13_CNT` writer
pub type W = crate::W<TIM13_CNTrs>;
///Field `CNT` reader - CNT
pub type CNT_R = crate::FieldReader<u16>;
///Field `CNT` writer - CNT
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `UIFCPY` reader - UIFCPY
pub type UIFCPY_R = crate::BitReader;
///Field `UIFCPY` writer - UIFCPY
pub type UIFCPY_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        f.debug_struct("TIM13_CNT")
            .field("cnt", &self.cnt())
            .field("uifcpy", &self.uifcpy())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - CNT
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<TIM13_CNTrs> {
        CNT_W::new(self, 0)
    }
    ///Bit 31 - UIFCPY
    #[inline(always)]
    #[must_use]
    pub fn uifcpy(&mut self) -> UIFCPY_W<TIM13_CNTrs> {
        UIFCPY_W::new(self, 31)
    }
}
/**TIM13 counter

You can [`read`](crate::Reg::read) this register and get [`tim13_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim13_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM13:TIM13_CNT)*/
pub struct TIM13_CNTrs;
impl crate::RegisterSpec for TIM13_CNTrs {
    type Ux = u32;
}
///`read()` method returns [`tim13_cnt::R`](R) reader structure
impl crate::Readable for TIM13_CNTrs {}
///`write(|w| ..)` method takes [`tim13_cnt::W`](W) writer structure
impl crate::Writable for TIM13_CNTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIM13_CNT to value 0
impl crate::Resettable for TIM13_CNTrs {
    const RESET_VALUE: u32 = 0;
}
