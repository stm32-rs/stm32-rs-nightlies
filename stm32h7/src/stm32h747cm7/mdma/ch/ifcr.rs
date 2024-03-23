#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IFCRrs>;
#[doc = "Field `CTEIF` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type CTEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type CCTCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRTIF` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type CBRTIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTIF` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type CBTIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTCIF` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type CLTCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cteif(&mut self) -> CTEIF_W<IFCRrs> {
        CTEIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cctcif(&mut self) -> CCTCIF_W<IFCRrs> {
        CCTCIF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cbrtif(&mut self) -> CBRTIF_W<IFCRrs> {
        CBRTIF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cbtif(&mut self) -> CBTIF_W<IFCRrs> {
        CBTIF_W::new(self, 3)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cltcif(&mut self) -> CLTCIF_W<IFCRrs> {
        CLTCIF_W::new(self, 4)
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCRrs {
    const RESET_VALUE: u32 = 0;
}
