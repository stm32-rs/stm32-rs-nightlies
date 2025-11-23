///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `CCLR` writer - CCLR
pub type CCLR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TCLR` writer - TCLR
pub type TCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:3 - CCLR
    #[inline(always)]
    pub fn cclr(&mut self) -> CCLR_W<'_, CCRrs> {
        CCLR_W::new(self, 0)
    }
    ///Bit 31 - TCLR
    #[inline(always)]
    pub fn tclr(&mut self) -> TCLR_W<'_, CCRrs> {
        TCLR_W::new(self, 31)
    }
}
/**Write-only register. A read request returns all zeros

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPERFM:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}
