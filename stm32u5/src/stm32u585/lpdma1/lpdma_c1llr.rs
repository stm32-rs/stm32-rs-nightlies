#[doc = "Register `LPDMA_C1LLR` reader"]
pub type R = crate::R<LPDMA_C1LLRrs>;
#[doc = "Register `LPDMA_C1LLR` writer"]
pub type W = crate::W<LPDMA_C1LLRrs>;
#[doc = "Field `LA` reader - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\\[15:20\\]
= 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list DMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list DMA internal register file (LPDMA_CxCTR1, LPDMA_CxTR2, LPDMA_CxBR1, LPDMA_CxSAR, LPDMA_CxDAR and LPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored."]
pub type LA_R = crate::FieldReader<u16>;
#[doc = "Field `LA` writer - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\\[15:20\\]
= 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list DMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list DMA internal register file (LPDMA_CxCTR1, LPDMA_CxTR2, LPDMA_CxBR1, LPDMA_CxSAR, LPDMA_CxDAR and LPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored."]
pub type LA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `ULL` reader - Update LPDMA_CxLLR register from memory This bit is used to control the update of the LPDMA_CxLLR register from the memory during the link transfer."]
pub type ULL_R = crate::BitReader;
#[doc = "Field `ULL` writer - Update LPDMA_CxLLR register from memory This bit is used to control the update of the LPDMA_CxLLR register from the memory during the link transfer."]
pub type ULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDA` reader - Update LPDMA_CxDAR register from memory This bit is used to control the update of the LPDMA_CxDAR register from the memory during the link transfer."]
pub type UDA_R = crate::BitReader;
#[doc = "Field `UDA` writer - Update LPDMA_CxDAR register from memory This bit is used to control the update of the LPDMA_CxDAR register from the memory during the link transfer."]
pub type UDA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USA` reader - update LPDMA_CxSAR from memory This bit controls the update of the LPDMA_CxSAR register from the memory during the link transfer."]
pub type USA_R = crate::BitReader;
#[doc = "Field `USA` writer - update LPDMA_CxSAR from memory This bit controls the update of the LPDMA_CxSAR register from the memory during the link transfer."]
pub type USA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UB1` reader - Update LPDMA_CxBR1 from memory This bit controls the update of the LPDMA_CxBR1 register from the memory during the link transfer."]
pub type UB1_R = crate::BitReader;
#[doc = "Field `UB1` writer - Update LPDMA_CxBR1 from memory This bit controls the update of the LPDMA_CxBR1 register from the memory during the link transfer."]
pub type UB1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UT2` reader - Update LPDMA_CxTR2 from memory This bit controls the update of the LPDMA_CxTR2 register from the memory during the link transfer."]
pub type UT2_R = crate::BitReader;
#[doc = "Field `UT2` writer - Update LPDMA_CxTR2 from memory This bit controls the update of the LPDMA_CxTR2 register from the memory during the link transfer."]
pub type UT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UT1` reader - Update LPDMA_CxTR1 from memory This bit controls the update of the LPDMA_CxTR1 register from the memory during the link transfer."]
pub type UT1_R = crate::BitReader;
#[doc = "Field `UT1` writer - Update LPDMA_CxTR1 from memory This bit controls the update of the LPDMA_CxTR1 register from the memory during the link transfer."]
pub type UT1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\\[15:20\\]
= 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list DMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list DMA internal register file (LPDMA_CxCTR1, LPDMA_CxTR2, LPDMA_CxBR1, LPDMA_CxSAR, LPDMA_CxDAR and LPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored."]
    #[inline(always)]
    pub fn la(&self) -> LA_R {
        LA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - Update LPDMA_CxLLR register from memory This bit is used to control the update of the LPDMA_CxLLR register from the memory during the link transfer."]
    #[inline(always)]
    pub fn ull(&self) -> ULL_R {
        ULL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 27 - Update LPDMA_CxDAR register from memory This bit is used to control the update of the LPDMA_CxDAR register from the memory during the link transfer."]
    #[inline(always)]
    pub fn uda(&self) -> UDA_R {
        UDA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - update LPDMA_CxSAR from memory This bit controls the update of the LPDMA_CxSAR register from the memory during the link transfer."]
    #[inline(always)]
    pub fn usa(&self) -> USA_R {
        USA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Update LPDMA_CxBR1 from memory This bit controls the update of the LPDMA_CxBR1 register from the memory during the link transfer."]
    #[inline(always)]
    pub fn ub1(&self) -> UB1_R {
        UB1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Update LPDMA_CxTR2 from memory This bit controls the update of the LPDMA_CxTR2 register from the memory during the link transfer."]
    #[inline(always)]
    pub fn ut2(&self) -> UT2_R {
        UT2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Update LPDMA_CxTR1 from memory This bit controls the update of the LPDMA_CxTR1 register from the memory during the link transfer."]
    #[inline(always)]
    pub fn ut1(&self) -> UT1_R {
        UT1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\\[15:20\\]
= 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list DMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list DMA internal register file (LPDMA_CxCTR1, LPDMA_CxTR2, LPDMA_CxBR1, LPDMA_CxSAR, LPDMA_CxDAR and LPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored."]
    #[inline(always)]
    #[must_use]
    pub fn la(&mut self) -> LA_W<LPDMA_C1LLRrs> {
        LA_W::new(self, 2)
    }
    #[doc = "Bit 16 - Update LPDMA_CxLLR register from memory This bit is used to control the update of the LPDMA_CxLLR register from the memory during the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn ull(&mut self) -> ULL_W<LPDMA_C1LLRrs> {
        ULL_W::new(self, 16)
    }
    #[doc = "Bit 27 - Update LPDMA_CxDAR register from memory This bit is used to control the update of the LPDMA_CxDAR register from the memory during the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn uda(&mut self) -> UDA_W<LPDMA_C1LLRrs> {
        UDA_W::new(self, 27)
    }
    #[doc = "Bit 28 - update LPDMA_CxSAR from memory This bit controls the update of the LPDMA_CxSAR register from the memory during the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn usa(&mut self) -> USA_W<LPDMA_C1LLRrs> {
        USA_W::new(self, 28)
    }
    #[doc = "Bit 29 - Update LPDMA_CxBR1 from memory This bit controls the update of the LPDMA_CxBR1 register from the memory during the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn ub1(&mut self) -> UB1_W<LPDMA_C1LLRrs> {
        UB1_W::new(self, 29)
    }
    #[doc = "Bit 30 - Update LPDMA_CxTR2 from memory This bit controls the update of the LPDMA_CxTR2 register from the memory during the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn ut2(&mut self) -> UT2_W<LPDMA_C1LLRrs> {
        UT2_W::new(self, 30)
    }
    #[doc = "Bit 31 - Update LPDMA_CxTR1 from memory This bit controls the update of the LPDMA_CxTR1 register from the memory during the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn ut1(&mut self) -> UT1_W<LPDMA_C1LLRrs> {
        UT1_W::new(self, 31)
    }
}
#[doc = "LPDMA channel 1 linked-list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c1llr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c1llr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPDMA_C1LLRrs;
impl crate::RegisterSpec for LPDMA_C1LLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpdma_c1llr::R`](R) reader structure"]
impl crate::Readable for LPDMA_C1LLRrs {}
#[doc = "`write(|w| ..)` method takes [`lpdma_c1llr::W`](W) writer structure"]
impl crate::Writable for LPDMA_C1LLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPDMA_C1LLR to value 0"]
impl crate::Resettable for LPDMA_C1LLRrs {
    const RESET_VALUE: u32 = 0;
}
