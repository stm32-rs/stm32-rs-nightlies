///Register `CNT` reader
pub type R = crate::R<CNTrs>;
///Register `CNT` writer
pub type W = crate::W<CNTrs>;
///Field `CNT` reader - Least significant part of counter value
pub type CNT_R = crate::FieldReader<u32>;
///Field `CNT` writer - Least significant part of counter value
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
///Field `UIFCPY_CNT` reader - Value depends on IUFREMAP in TIMx_CR1.
pub type UIFCPY_CNT_R = crate::BitReader;
///Field `UIFCPY_CNT` writer - Value depends on IUFREMAP in TIMx_CR1.
pub type UIFCPY_CNT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:30 - Least significant part of counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - Value depends on IUFREMAP in TIMx_CR1.
    #[inline(always)]
    pub fn uifcpy_cnt(&self) -> UIFCPY_CNT_R {
        UIFCPY_CNT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT")
            .field("cnt", &self.cnt())
            .field("uifcpy_cnt", &self.uifcpy_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:30 - Least significant part of counter value
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<'_, CNTrs> {
        CNT_W::new(self, 0)
    }
    ///Bit 31 - Value depends on IUFREMAP in TIMx_CR1.
    #[inline(always)]
    pub fn uifcpy_cnt(&mut self) -> UIFCPY_CNT_W<'_, CNTrs> {
        UIFCPY_CNT_W::new(self, 31)
    }
}
/**TIM3 counter

You can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM3:CNT)*/
pub struct CNTrs;
impl crate::RegisterSpec for CNTrs {
    type Ux = u32;
}
///`read()` method returns [`cnt::R`](R) reader structure
impl crate::Readable for CNTrs {}
///`write(|w| ..)` method takes [`cnt::W`](W) writer structure
impl crate::Writable for CNTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CNT to value 0
impl crate::Resettable for CNTrs {}
