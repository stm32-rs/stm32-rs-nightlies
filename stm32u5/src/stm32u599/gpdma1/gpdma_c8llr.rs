#[doc = "Register `GPDMA_C8LLR` reader"]
pub type R = crate::R<GPDMA_C8LLRrs>;
#[doc = "Register `GPDMA_C8LLR` writer"]
pub type W = crate::W<GPDMA_C8LLRrs>;
#[doc = "Field `LA` reader - pointer (16-bit low significant address) to the next linked-list data structure If UT1=UT2=UB1=USA=UDA=ULL=0 and if LA\\[15:2\\]=0: the current LLI is the last one. The channel transfer is completed without any update of the linked-list DMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure will be automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list DMA internal register file i.e. possibly GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR. Note: The user should program the pointer to be 32-bit aligned. The two low significant bits are write ignored."]
pub type LA_R = crate::FieldReader<u16>;
#[doc = "Field `LA` writer - pointer (16-bit low significant address) to the next linked-list data structure If UT1=UT2=UB1=USA=UDA=ULL=0 and if LA\\[15:2\\]=0: the current LLI is the last one. The channel transfer is completed without any update of the linked-list DMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure will be automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list DMA internal register file i.e. possibly GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR. Note: The user should program the pointer to be 32-bit aligned. The two low significant bits are write ignored."]
pub type LA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `ULL` reader - Update GPDMA_CxLLR from memory This bit controls the update of the GPDMA_CxLLR register from the memory during the link transfer. - 0: no GPDMA_CxLLR update - 1: GPDMA_CxLLR update"]
pub type ULL_R = crate::BitReader;
#[doc = "Field `ULL` writer - Update GPDMA_CxLLR from memory This bit controls the update of the GPDMA_CxLLR register from the memory during the link transfer. - 0: no GPDMA_CxLLR update - 1: GPDMA_CxLLR update"]
pub type ULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDA` reader - Update GPDMA_CxDAR from memory This bit controls the update of the GPDMA_CxDAR register from the memory during the link transfer. - 0: no GPDMA_CxDAR update - 1: GPDMA_CxDAR update"]
pub type UDA_R = crate::BitReader;
#[doc = "Field `UDA` writer - Update GPDMA_CxDAR from memory This bit controls the update of the GPDMA_CxDAR register from the memory during the link transfer. - 0: no GPDMA_CxDAR update - 1: GPDMA_CxDAR update"]
pub type UDA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USA` reader - Update GPDMA_CxSAR from memory This bit controls the update of the GPDMA_CxSAR register from the memory during the link transfer. - 0: no GPDMA_CxSAR update - 1: GPDMA_CxSAR update"]
pub type USA_R = crate::BitReader;
#[doc = "Field `USA` writer - Update GPDMA_CxSAR from memory This bit controls the update of the GPDMA_CxSAR register from the memory during the link transfer. - 0: no GPDMA_CxSAR update - 1: GPDMA_CxSAR update"]
pub type USA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UB1` reader - Update GPDMA_CxBR1 from memory This bit controls the update of the GPDMA_CxBR1 register from the memory during the link transfer. If UB1=0 and if GPDMA_CxLLR != 0, the linked-list is not completed. Then GPDMA_CxBR1.BNDT\\[15:0\\]
is restored to the programmed value after data transfer is completed and before the link transfer. - 0: no GPDMA_CxBR1 update (GPDMA_CxBR1.BNDT\\[15:0\\]
is restored, if any link transfer) - 1: GPDMA_CxBR1 update"]
pub type UB1_R = crate::BitReader;
#[doc = "Field `UB1` writer - Update GPDMA_CxBR1 from memory This bit controls the update of the GPDMA_CxBR1 register from the memory during the link transfer. If UB1=0 and if GPDMA_CxLLR != 0, the linked-list is not completed. Then GPDMA_CxBR1.BNDT\\[15:0\\]
is restored to the programmed value after data transfer is completed and before the link transfer. - 0: no GPDMA_CxBR1 update (GPDMA_CxBR1.BNDT\\[15:0\\]
is restored, if any link transfer) - 1: GPDMA_CxBR1 update"]
pub type UB1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UT2` reader - Update GPDMA_CxTR2 from memory This bit controls the update of the GPDMA_CxTR2 register from the memory during the link transfer. - 0: no GPDMA_CxTR2 update - 1: GPDMA_CxTR2 update"]
pub type UT2_R = crate::BitReader;
#[doc = "Field `UT2` writer - Update GPDMA_CxTR2 from memory This bit controls the update of the GPDMA_CxTR2 register from the memory during the link transfer. - 0: no GPDMA_CxTR2 update - 1: GPDMA_CxTR2 update"]
pub type UT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UT1` reader - Update GPDMA_CxTR1 from memory This bit controls the update of the GPDMA_CxTR1 register from the memory during the link transfer. - 0: no GPDMA_CxTR1 update - 1: GPDMA_CxTR1 update"]
pub type UT1_R = crate::BitReader;
#[doc = "Field `UT1` writer - Update GPDMA_CxTR1 from memory This bit controls the update of the GPDMA_CxTR1 register from the memory during the link transfer. - 0: no GPDMA_CxTR1 update - 1: GPDMA_CxTR1 update"]
pub type UT1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - pointer (16-bit low significant address) to the next linked-list data structure If UT1=UT2=UB1=USA=UDA=ULL=0 and if LA\\[15:2\\]=0: the current LLI is the last one. The channel transfer is completed without any update of the linked-list DMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure will be automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list DMA internal register file i.e. possibly GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR. Note: The user should program the pointer to be 32-bit aligned. The two low significant bits are write ignored."]
    #[inline(always)]
    pub fn la(&self) -> LA_R {
        LA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - Update GPDMA_CxLLR from memory This bit controls the update of the GPDMA_CxLLR register from the memory during the link transfer. - 0: no GPDMA_CxLLR update - 1: GPDMA_CxLLR update"]
    #[inline(always)]
    pub fn ull(&self) -> ULL_R {
        ULL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 27 - Update GPDMA_CxDAR from memory This bit controls the update of the GPDMA_CxDAR register from the memory during the link transfer. - 0: no GPDMA_CxDAR update - 1: GPDMA_CxDAR update"]
    #[inline(always)]
    pub fn uda(&self) -> UDA_R {
        UDA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Update GPDMA_CxSAR from memory This bit controls the update of the GPDMA_CxSAR register from the memory during the link transfer. - 0: no GPDMA_CxSAR update - 1: GPDMA_CxSAR update"]
    #[inline(always)]
    pub fn usa(&self) -> USA_R {
        USA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Update GPDMA_CxBR1 from memory This bit controls the update of the GPDMA_CxBR1 register from the memory during the link transfer. If UB1=0 and if GPDMA_CxLLR != 0, the linked-list is not completed. Then GPDMA_CxBR1.BNDT\\[15:0\\]
is restored to the programmed value after data transfer is completed and before the link transfer. - 0: no GPDMA_CxBR1 update (GPDMA_CxBR1.BNDT\\[15:0\\]
is restored, if any link transfer) - 1: GPDMA_CxBR1 update"]
    #[inline(always)]
    pub fn ub1(&self) -> UB1_R {
        UB1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Update GPDMA_CxTR2 from memory This bit controls the update of the GPDMA_CxTR2 register from the memory during the link transfer. - 0: no GPDMA_CxTR2 update - 1: GPDMA_CxTR2 update"]
    #[inline(always)]
    pub fn ut2(&self) -> UT2_R {
        UT2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Update GPDMA_CxTR1 from memory This bit controls the update of the GPDMA_CxTR1 register from the memory during the link transfer. - 0: no GPDMA_CxTR1 update - 1: GPDMA_CxTR1 update"]
    #[inline(always)]
    pub fn ut1(&self) -> UT1_R {
        UT1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - pointer (16-bit low significant address) to the next linked-list data structure If UT1=UT2=UB1=USA=UDA=ULL=0 and if LA\\[15:2\\]=0: the current LLI is the last one. The channel transfer is completed without any update of the linked-list DMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure will be automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list DMA internal register file i.e. possibly GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR. Note: The user should program the pointer to be 32-bit aligned. The two low significant bits are write ignored."]
    #[inline(always)]
    #[must_use]
    pub fn la(&mut self) -> LA_W<GPDMA_C8LLRrs> {
        LA_W::new(self, 2)
    }
    #[doc = "Bit 16 - Update GPDMA_CxLLR from memory This bit controls the update of the GPDMA_CxLLR register from the memory during the link transfer. - 0: no GPDMA_CxLLR update - 1: GPDMA_CxLLR update"]
    #[inline(always)]
    #[must_use]
    pub fn ull(&mut self) -> ULL_W<GPDMA_C8LLRrs> {
        ULL_W::new(self, 16)
    }
    #[doc = "Bit 27 - Update GPDMA_CxDAR from memory This bit controls the update of the GPDMA_CxDAR register from the memory during the link transfer. - 0: no GPDMA_CxDAR update - 1: GPDMA_CxDAR update"]
    #[inline(always)]
    #[must_use]
    pub fn uda(&mut self) -> UDA_W<GPDMA_C8LLRrs> {
        UDA_W::new(self, 27)
    }
    #[doc = "Bit 28 - Update GPDMA_CxSAR from memory This bit controls the update of the GPDMA_CxSAR register from the memory during the link transfer. - 0: no GPDMA_CxSAR update - 1: GPDMA_CxSAR update"]
    #[inline(always)]
    #[must_use]
    pub fn usa(&mut self) -> USA_W<GPDMA_C8LLRrs> {
        USA_W::new(self, 28)
    }
    #[doc = "Bit 29 - Update GPDMA_CxBR1 from memory This bit controls the update of the GPDMA_CxBR1 register from the memory during the link transfer. If UB1=0 and if GPDMA_CxLLR != 0, the linked-list is not completed. Then GPDMA_CxBR1.BNDT\\[15:0\\]
is restored to the programmed value after data transfer is completed and before the link transfer. - 0: no GPDMA_CxBR1 update (GPDMA_CxBR1.BNDT\\[15:0\\]
is restored, if any link transfer) - 1: GPDMA_CxBR1 update"]
    #[inline(always)]
    #[must_use]
    pub fn ub1(&mut self) -> UB1_W<GPDMA_C8LLRrs> {
        UB1_W::new(self, 29)
    }
    #[doc = "Bit 30 - Update GPDMA_CxTR2 from memory This bit controls the update of the GPDMA_CxTR2 register from the memory during the link transfer. - 0: no GPDMA_CxTR2 update - 1: GPDMA_CxTR2 update"]
    #[inline(always)]
    #[must_use]
    pub fn ut2(&mut self) -> UT2_W<GPDMA_C8LLRrs> {
        UT2_W::new(self, 30)
    }
    #[doc = "Bit 31 - Update GPDMA_CxTR1 from memory This bit controls the update of the GPDMA_CxTR1 register from the memory during the link transfer. - 0: no GPDMA_CxTR1 update - 1: GPDMA_CxTR1 update"]
    #[inline(always)]
    #[must_use]
    pub fn ut1(&mut self) -> UT1_W<GPDMA_C8LLRrs> {
        UT1_W::new(self, 31)
    }
}
#[doc = "GPDMA channel x linked-list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdma_c8llr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdma_c8llr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPDMA_C8LLRrs;
impl crate::RegisterSpec for GPDMA_C8LLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdma_c8llr::R`](R) reader structure"]
impl crate::Readable for GPDMA_C8LLRrs {}
#[doc = "`write(|w| ..)` method takes [`gpdma_c8llr::W`](W) writer structure"]
impl crate::Writable for GPDMA_C8LLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPDMA_C8LLR to value 0"]
impl crate::Resettable for GPDMA_C8LLRrs {
    const RESET_VALUE: u32 = 0;
}
