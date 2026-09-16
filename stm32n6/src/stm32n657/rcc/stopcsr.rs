///Register `STOPCSR` writer
pub type W = crate::W<STOPCSRrs>;
///Field `MSISTOPENS` writer - MSISTOPENS
pub type MSISTOPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSISTOPENS` writer - HSISTOPENS
pub type HSISTOPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<STOPCSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - MSISTOPENS
    #[inline(always)]
    pub fn msistopens(&mut self) -> MSISTOPENS_W<'_, STOPCSRrs> {
        MSISTOPENS_W::new(self, 0)
    }
    ///Bit 1 - HSISTOPENS
    #[inline(always)]
    pub fn hsistopens(&mut self) -> HSISTOPENS_W<'_, STOPCSRrs> {
        HSISTOPENS_W::new(self, 1)
    }
}
/**RCC Stop configuration register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stopcsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:STOPCSR)*/
pub struct STOPCSRrs;
impl crate::RegisterSpec for STOPCSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`stopcsr::W`](W) writer structure
impl crate::Writable for STOPCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STOPCSR to value 0
impl crate::Resettable for STOPCSRrs {}
