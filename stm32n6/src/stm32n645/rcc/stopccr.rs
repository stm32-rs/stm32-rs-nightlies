///Register `STOPCCR` writer
pub type W = crate::W<STOPCCRrs>;
///Field `LSISTOPENC` writer - LSI oscillator enable in Run/Sleep mode.
pub type LSISTOPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSESTOPENC` writer - LSE oscillator enable in Run/Sleep mode.
pub type LSESTOPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSISTOPENC` writer - MSI oscillator enable in Run/Sleep mode.
pub type MSISTOPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSISTOPENC` writer - HSI oscillator enable in Run/Sleep mode.
pub type HSISTOPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<STOPCCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - LSI oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn lsistopenc(&mut self) -> LSISTOPENC_W<'_, STOPCCRrs> {
        LSISTOPENC_W::new(self, 0)
    }
    ///Bit 1 - LSE oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn lsestopenc(&mut self) -> LSESTOPENC_W<'_, STOPCCRrs> {
        LSESTOPENC_W::new(self, 1)
    }
    ///Bit 2 - MSI oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn msistopenc(&mut self) -> MSISTOPENC_W<'_, STOPCCRrs> {
        MSISTOPENC_W::new(self, 2)
    }
    ///Bit 3 - HSI oscillator enable in Run/Sleep mode.
    #[inline(always)]
    pub fn hsistopenc(&mut self) -> HSISTOPENC_W<'_, STOPCCRrs> {
        HSISTOPENC_W::new(self, 3)
    }
}
/**RCC StopCCR configuration register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stopccr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:STOPCCR)*/
pub struct STOPCCRrs;
impl crate::RegisterSpec for STOPCCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`stopccr::W`](W) writer structure
impl crate::Writable for STOPCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STOPCCR to value 0
impl crate::Resettable for STOPCCRrs {}
