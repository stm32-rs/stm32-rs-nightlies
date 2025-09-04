///Register `CNT_ALTERNATE5` reader
pub type R = crate::R<CNT_ALTERNATE5rs>;
///Register `CNT_ALTERNATE5` writer
pub type W = crate::W<CNT_ALTERNATE5rs>;
///Field `CNT` reader - Most significant part counter value (TIM2) nullLeast significant part of counter value
pub type CNT_R = crate::FieldReader<u32>;
///Field `CNT` writer - Most significant part counter value (TIM2) nullLeast significant part of counter value
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
///Field `UIFCPY` reader - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register
pub type UIFCPY_R = crate::BitReader;
///Field `UIFCPY` writer - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register
pub type UIFCPY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:30 - Most significant part counter value (TIM2) nullLeast significant part of counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT_ALTERNATE5")
            .field("cnt", &self.cnt())
            .field("uifcpy", &self.uifcpy())
            .finish()
    }
}
impl W {
    ///Bits 0:30 - Most significant part counter value (TIM2) nullLeast significant part of counter value
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<CNT_ALTERNATE5rs> {
        CNT_W::new(self, 0)
    }
    ///Bit 31 - UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register
    #[inline(always)]
    pub fn uifcpy(&mut self) -> UIFCPY_W<CNT_ALTERNATE5rs> {
        UIFCPY_W::new(self, 31)
    }
}
/**counter

You can [`read`](crate::Reg::read) this register and get [`cnt_alternate5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt_alternate5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#TIM3:CNT_ALTERNATE5)*/
pub struct CNT_ALTERNATE5rs;
impl crate::RegisterSpec for CNT_ALTERNATE5rs {
    type Ux = u32;
}
///`read()` method returns [`cnt_alternate5::R`](R) reader structure
impl crate::Readable for CNT_ALTERNATE5rs {}
///`write(|w| ..)` method takes [`cnt_alternate5::W`](W) writer structure
impl crate::Writable for CNT_ALTERNATE5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CNT_ALTERNATE5 to value 0
impl crate::Resettable for CNT_ALTERNATE5rs {}
