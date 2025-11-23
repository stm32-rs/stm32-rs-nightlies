///Register `CNT` reader
pub type R = crate::R<CNTrs>;
///Register `CNT` writer
pub type W = crate::W<CNTrs>;
///Field `CNT` reader - Counter value
pub type CNT_R = crate::FieldReader<u32>;
///Field `CNT` writer - Counter value
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
///Field `UIFREMAP_CNT` reader - Counter value when CR1.UIFREMAP=1
pub type UIFREMAP_CNT_R = crate::FieldReader<u32>;
///Field `UIFREMAP_CNT` writer - Counter value when CR1.UIFREMAP=1
pub type UIFREMAP_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
///Field `UIFCPY` reader - Copy of ISR.UIF when CR1.UIFREMAP=1
pub type UIFCPY_R = crate::BitReader;
impl R {
    ///Bits 0:31 - Counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
    ///Bits 0:30 - Counter value when CR1.UIFREMAP=1
    #[inline(always)]
    pub fn uifremap_cnt(&self) -> UIFREMAP_CNT_R {
        UIFREMAP_CNT_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - Copy of ISR.UIF when CR1.UIFREMAP=1
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT")
            .field("cnt", &self.cnt())
            .field("uifcpy", &self.uifcpy())
            .field("uifremap_cnt", &self.uifremap_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Counter value
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<'_, CNTrs> {
        CNT_W::new(self, 0)
    }
    ///Bits 0:30 - Counter value when CR1.UIFREMAP=1
    #[inline(always)]
    pub fn uifremap_cnt(&mut self) -> UIFREMAP_CNT_W<'_, CNTrs> {
        UIFREMAP_CNT_W::new(self, 0)
    }
}
/**counter

You can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#TIM2:CNT)*/
pub struct CNTrs;
impl crate::RegisterSpec for CNTrs {
    type Ux = u32;
}
///`read()` method returns [`cnt::R`](R) reader structure
impl crate::Readable for CNTrs {}
///`write(|w| ..)` method takes [`cnt::W`](W) writer structure
impl crate::Writable for CNTrs {
    type Safety = crate::Safe;
}
///`reset()` method sets CNT to value 0
impl crate::Resettable for CNTrs {}
