///Register `_CNT` reader
pub type R = crate::R<_CNTrs>;
///Register `_CNT` writer
pub type W = crate::W<_CNTrs>;
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
        f.debug_struct("_CNT")
            .field("cnt", &self.cnt())
            .field("uifcpy", &self.uifcpy())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - CNT
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<_CNTrs> {
        CNT_W::new(self, 0)
    }
    ///Bit 31 - UIFCPY
    #[inline(always)]
    pub fn uifcpy(&mut self) -> UIFCPY_W<_CNTrs> {
        UIFCPY_W::new(self, 31)
    }
}
/**TIM14 counter

You can [`read`](crate::Reg::read) this register and get [`_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM14:_CNT)*/
pub struct _CNTrs;
impl crate::RegisterSpec for _CNTrs {
    type Ux = u32;
}
///`read()` method returns [`_cnt::R`](R) reader structure
impl crate::Readable for _CNTrs {}
///`write(|w| ..)` method takes [`_cnt::W`](W) writer structure
impl crate::Writable for _CNTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _CNT to value 0
impl crate::Resettable for _CNTrs {}
