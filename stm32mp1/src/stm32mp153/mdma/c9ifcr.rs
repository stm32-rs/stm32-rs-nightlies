///Register `C9IFCR` writer
pub type W = crate::W<C9IFCRrs>;
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
impl core::fmt::Debug for crate::generic::Reg<C9IFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - CTEIF
    #[inline(always)]
    pub fn cteif(&mut self) -> CTEIF_W<C9IFCRrs> {
        CTEIF_W::new(self, 0)
    }
    ///Bit 1 - CCTCIF
    #[inline(always)]
    pub fn cctcif(&mut self) -> CCTCIF_W<C9IFCRrs> {
        CCTCIF_W::new(self, 1)
    }
    ///Bit 2 - CBRTIF
    #[inline(always)]
    pub fn cbrtif(&mut self) -> CBRTIF_W<C9IFCRrs> {
        CBRTIF_W::new(self, 2)
    }
    ///Bit 3 - CBTIF
    #[inline(always)]
    pub fn cbtif(&mut self) -> CBTIF_W<C9IFCRrs> {
        CBTIF_W::new(self, 3)
    }
    ///Bit 4 - CLTCIF
    #[inline(always)]
    pub fn cltcif(&mut self) -> CLTCIF_W<C9IFCRrs> {
        CLTCIF_W::new(self, 4)
    }
}
/**MDMA channel 9 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:C9IFCR)*/
pub struct C9IFCRrs;
impl crate::RegisterSpec for C9IFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`c9ifcr::W`](W) writer structure
impl crate::Writable for C9IFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C9IFCR to value 0
impl crate::Resettable for C9IFCRrs {}
