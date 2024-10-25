///Register `C21IFCR` writer
pub type W = crate::W<C21IFCRrs>;
///Field `CTEIF` writer - CTEIF
pub type CTEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCTCIF` writer - CCTCIF
pub type CCTCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CBRTIF` writer - CBRTIF
pub type CBRTIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CBTIF` writer - CBTIF
pub type CBTIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLTCIF` writer - CLTCIF
pub type CLTCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<C21IFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - CTEIF
    #[inline(always)]
    #[must_use]
    pub fn cteif(&mut self) -> CTEIF_W<C21IFCRrs> {
        CTEIF_W::new(self, 0)
    }
    ///Bit 1 - CCTCIF
    #[inline(always)]
    #[must_use]
    pub fn cctcif(&mut self) -> CCTCIF_W<C21IFCRrs> {
        CCTCIF_W::new(self, 1)
    }
    ///Bit 2 - CBRTIF
    #[inline(always)]
    #[must_use]
    pub fn cbrtif(&mut self) -> CBRTIF_W<C21IFCRrs> {
        CBRTIF_W::new(self, 2)
    }
    ///Bit 3 - CBTIF
    #[inline(always)]
    #[must_use]
    pub fn cbtif(&mut self) -> CBTIF_W<C21IFCRrs> {
        CBTIF_W::new(self, 3)
    }
    ///Bit 4 - CLTCIF
    #[inline(always)]
    #[must_use]
    pub fn cltcif(&mut self) -> CLTCIF_W<C21IFCRrs> {
        CLTCIF_W::new(self, 4)
    }
}
/**MDMA channel 21 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c21ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C21IFCR)*/
pub struct C21IFCRrs;
impl crate::RegisterSpec for C21IFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`c21ifcr::W`](W) writer structure
impl crate::Writable for C21IFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C21IFCR to value 0
impl crate::Resettable for C21IFCRrs {
    const RESET_VALUE: u32 = 0;
}
