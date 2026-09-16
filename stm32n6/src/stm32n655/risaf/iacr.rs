///Register `IACR` writer
pub type W = crate::W<IACRrs>;
///Field `CAEF` writer - configuration access error flag
pub type CAEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAEF` writer - illegal access error flag
pub type IAEF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IACRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - configuration access error flag
    #[inline(always)]
    pub fn caef(&mut self) -> CAEF_W<'_, IACRrs> {
        CAEF_W::new(self, 0)
    }
    ///Bit 1 - illegal access error flag
    #[inline(always)]
    pub fn iaef(&mut self) -> IAEF_W<'_, IACRrs> {
        IAEF_W::new(self, 1)
    }
}
/**RISAF illegal access clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iacr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RISAF:IACR)*/
pub struct IACRrs;
impl crate::RegisterSpec for IACRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`iacr::W`](W) writer structure
impl crate::Writable for IACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IACR to value 0
impl crate::Resettable for IACRrs {}
