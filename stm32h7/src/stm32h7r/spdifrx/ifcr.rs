///Register `IFCR` writer
pub type W = crate::W<IFCRrs>;
///Field `PERRCF` writer - clears the parity error flag Writing 1 in this bit clears the flag PERR in the SPDIFRX_SR register. Reading this bit always returns the value 0.
pub type PERRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRCF` writer - clears the overrun error flag Writing 1 in this bit clears the flag OVR in the SPDIFRX_SR register. Reading this bit always returns the value 0.
pub type OVRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBDCF` writer - clears the synchronization block detected flag Writing 1 in this bit clears the flag SBD in the SPDIFRX_SR register. Reading this bit always returns the value 0.
pub type SBDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNCDCF` writer - clears the synchronization done flag Writing 1 in this bit clears the flag SYNCD in the SPDIFRX_SR register. Reading this bit always returns the value 0.
pub type SYNCDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 2 - clears the parity error flag Writing 1 in this bit clears the flag PERR in the SPDIFRX_SR register. Reading this bit always returns the value 0.
    #[inline(always)]
    pub fn perrcf(&mut self) -> PERRCF_W<IFCRrs> {
        PERRCF_W::new(self, 2)
    }
    ///Bit 3 - clears the overrun error flag Writing 1 in this bit clears the flag OVR in the SPDIFRX_SR register. Reading this bit always returns the value 0.
    #[inline(always)]
    pub fn ovrcf(&mut self) -> OVRCF_W<IFCRrs> {
        OVRCF_W::new(self, 3)
    }
    ///Bit 4 - clears the synchronization block detected flag Writing 1 in this bit clears the flag SBD in the SPDIFRX_SR register. Reading this bit always returns the value 0.
    #[inline(always)]
    pub fn sbdcf(&mut self) -> SBDCF_W<IFCRrs> {
        SBDCF_W::new(self, 4)
    }
    ///Bit 5 - clears the synchronization done flag Writing 1 in this bit clears the flag SYNCD in the SPDIFRX_SR register. Reading this bit always returns the value 0.
    #[inline(always)]
    pub fn syncdcf(&mut self) -> SYNCDCF_W<IFCRrs> {
        SYNCDCF_W::new(self, 5)
    }
}
/**SPDIFRX interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#SPDIFRX:IFCR)*/
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ifcr::W`](W) writer structure
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IFCR to value 0
impl crate::Resettable for IFCRrs {}
