///Register `IFCR` writer
pub type W = crate::W<IFCRrs>;
///Field `CTEIF` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
pub type CTEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCTCIF` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register
pub type CCTCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CBRTIF` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register
pub type CBRTIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CBTIF` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register
pub type CBTIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLTCIF` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register
pub type CLTCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
    #[inline(always)]
    pub fn cteif(&mut self) -> CTEIF_W<'_, IFCRrs> {
        CTEIF_W::new(self, 0)
    }
    ///Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register
    #[inline(always)]
    pub fn cctcif(&mut self) -> CCTCIF_W<'_, IFCRrs> {
        CCTCIF_W::new(self, 1)
    }
    ///Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register
    #[inline(always)]
    pub fn cbrtif(&mut self) -> CBRTIF_W<'_, IFCRrs> {
        CBRTIF_W::new(self, 2)
    }
    ///Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register
    #[inline(always)]
    pub fn cbtif(&mut self) -> CBTIF_W<'_, IFCRrs> {
        CBTIF_W::new(self, 3)
    }
    ///Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register
    #[inline(always)]
    pub fn cltcif(&mut self) -> CLTCIF_W<'_, IFCRrs> {
        CLTCIF_W::new(self, 4)
    }
}
/**MDMA channel x interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
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
