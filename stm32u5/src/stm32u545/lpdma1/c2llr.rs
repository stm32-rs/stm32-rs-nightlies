///Register `C2LLR` reader
pub type R = crate::R<C2LLRrs>;
///Register `C2LLR` writer
pub type W = crate::W<C2LLRrs>;
///Field `LA` reader - pointer (16-bit low significant address) to the next linked-list data structure If UT1=UT2=UB1=USA=UDA=ULL=0 and if LA\[15:2\]=0: the current LLI is the last one. The channel transfer is completed without any update of the linked-list DMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure will be automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list DMA internal register file i.e. possibly LPDMA_CxCTR1, LPDMA_CxTR2, LPDMA_CxBR1, LPDMA_CxSAR, LPDMA_CxDAR and LPDMA_CxLLR. Note: The user should program the pointer to be 32-bit aligned. The two low significant bits are write ignored.
pub type LA_R = crate::FieldReader<u16>;
///Field `LA` writer - pointer (16-bit low significant address) to the next linked-list data structure If UT1=UT2=UB1=USA=UDA=ULL=0 and if LA\[15:2\]=0: the current LLI is the last one. The channel transfer is completed without any update of the linked-list DMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure will be automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list DMA internal register file i.e. possibly LPDMA_CxCTR1, LPDMA_CxTR2, LPDMA_CxBR1, LPDMA_CxSAR, LPDMA_CxDAR and LPDMA_CxLLR. Note: The user should program the pointer to be 32-bit aligned. The two low significant bits are write ignored.
pub type LA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `ULL` reader - Update LPDMA_CxLLR from memory This bit controls the update of the LPDMA_CxLLR register from the memory during the link transfer. - 0: no LPDMA_CxLLR update - 1: LPDMA_CxLLR update
pub type ULL_R = crate::BitReader;
///Field `ULL` writer - Update LPDMA_CxLLR from memory This bit controls the update of the LPDMA_CxLLR register from the memory during the link transfer. - 0: no LPDMA_CxLLR update - 1: LPDMA_CxLLR update
pub type ULL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDA` reader - Update LPDMA_CxDAR from memory This bit controls the update of the LPDMA_CxDAR register from the memory during the link transfer. - 0: no LPDMA_CxDAR update - 1: LPDMA_CxDAR update
pub type UDA_R = crate::BitReader;
///Field `UDA` writer - Update LPDMA_CxDAR from memory This bit controls the update of the LPDMA_CxDAR register from the memory during the link transfer. - 0: no LPDMA_CxDAR update - 1: LPDMA_CxDAR update
pub type UDA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USA` reader - Update LPDMA_CxSAR from memory This bit controls the update of the LPDMA_CxSAR register from the memory during the link transfer. - 0: no LPDMA_CxSAR update - 1: LPDMA_CxSAR update
pub type USA_R = crate::BitReader;
///Field `USA` writer - Update LPDMA_CxSAR from memory This bit controls the update of the LPDMA_CxSAR register from the memory during the link transfer. - 0: no LPDMA_CxSAR update - 1: LPDMA_CxSAR update
pub type USA_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `UB1` reader - Update LPDMA_CxBR1 from memory This bit controls the update of the LPDMA_CxBR1 register from the memory during the link transfer. If UB1=0 and if LPDMA_CxLLR != 0, the linked-list is not completed. Then LPDMA_CxBR1.BNDT\[15:0\]
is restored to the programmed value after data transfer is completed and before the link transfer. - 0: no LPDMA_CxBR1 update (LPDMA_CxBR1.BNDT\[15:0\]
is restored, if any link transfer) - 1: LPDMA_CxBR1 update*/
pub type UB1_R = crate::BitReader;
/**Field `UB1` writer - Update LPDMA_CxBR1 from memory This bit controls the update of the LPDMA_CxBR1 register from the memory during the link transfer. If UB1=0 and if LPDMA_CxLLR != 0, the linked-list is not completed. Then LPDMA_CxBR1.BNDT\[15:0\]
is restored to the programmed value after data transfer is completed and before the link transfer. - 0: no LPDMA_CxBR1 update (LPDMA_CxBR1.BNDT\[15:0\]
is restored, if any link transfer) - 1: LPDMA_CxBR1 update*/
pub type UB1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UT2` reader - Update LPDMA_CxTR2 from memory This bit controls the update of the LPDMA_CxTR2 register from the memory during the link transfer. - 0: no LPDMA_CxTR2 update - 1: LPDMA_CxTR2 update
pub type UT2_R = crate::BitReader;
///Field `UT2` writer - Update LPDMA_CxTR2 from memory This bit controls the update of the LPDMA_CxTR2 register from the memory during the link transfer. - 0: no LPDMA_CxTR2 update - 1: LPDMA_CxTR2 update
pub type UT2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UT1` reader - Update LPDMA_CxTR1 from memory This bit controls the update of the LPDMA_CxTR1 register from the memory during the link transfer. - 0: no LPDMA_CxTR1 update - 1: LPDMA_CxTR1 update
pub type UT1_R = crate::BitReader;
///Field `UT1` writer - Update LPDMA_CxTR1 from memory This bit controls the update of the LPDMA_CxTR1 register from the memory during the link transfer. - 0: no LPDMA_CxTR1 update - 1: LPDMA_CxTR1 update
pub type UT1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 2:15 - pointer (16-bit low significant address) to the next linked-list data structure If UT1=UT2=UB1=USA=UDA=ULL=0 and if LA\[15:2\]=0: the current LLI is the last one. The channel transfer is completed without any update of the linked-list DMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure will be automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list DMA internal register file i.e. possibly LPDMA_CxCTR1, LPDMA_CxTR2, LPDMA_CxBR1, LPDMA_CxSAR, LPDMA_CxDAR and LPDMA_CxLLR. Note: The user should program the pointer to be 32-bit aligned. The two low significant bits are write ignored.
    #[inline(always)]
    pub fn la(&self) -> LA_R {
        LA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bit 16 - Update LPDMA_CxLLR from memory This bit controls the update of the LPDMA_CxLLR register from the memory during the link transfer. - 0: no LPDMA_CxLLR update - 1: LPDMA_CxLLR update
    #[inline(always)]
    pub fn ull(&self) -> ULL_R {
        ULL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 27 - Update LPDMA_CxDAR from memory This bit controls the update of the LPDMA_CxDAR register from the memory during the link transfer. - 0: no LPDMA_CxDAR update - 1: LPDMA_CxDAR update
    #[inline(always)]
    pub fn uda(&self) -> UDA_R {
        UDA_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Update LPDMA_CxSAR from memory This bit controls the update of the LPDMA_CxSAR register from the memory during the link transfer. - 0: no LPDMA_CxSAR update - 1: LPDMA_CxSAR update
    #[inline(always)]
    pub fn usa(&self) -> USA_R {
        USA_R::new(((self.bits >> 28) & 1) != 0)
    }
    /**Bit 29 - Update LPDMA_CxBR1 from memory This bit controls the update of the LPDMA_CxBR1 register from the memory during the link transfer. If UB1=0 and if LPDMA_CxLLR != 0, the linked-list is not completed. Then LPDMA_CxBR1.BNDT\[15:0\]
    is restored to the programmed value after data transfer is completed and before the link transfer. - 0: no LPDMA_CxBR1 update (LPDMA_CxBR1.BNDT\[15:0\]
    is restored, if any link transfer) - 1: LPDMA_CxBR1 update*/
    #[inline(always)]
    pub fn ub1(&self) -> UB1_R {
        UB1_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Update LPDMA_CxTR2 from memory This bit controls the update of the LPDMA_CxTR2 register from the memory during the link transfer. - 0: no LPDMA_CxTR2 update - 1: LPDMA_CxTR2 update
    #[inline(always)]
    pub fn ut2(&self) -> UT2_R {
        UT2_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Update LPDMA_CxTR1 from memory This bit controls the update of the LPDMA_CxTR1 register from the memory during the link transfer. - 0: no LPDMA_CxTR1 update - 1: LPDMA_CxTR1 update
    #[inline(always)]
    pub fn ut1(&self) -> UT1_R {
        UT1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2LLR")
            .field("la", &self.la())
            .field("ull", &self.ull())
            .field("uda", &self.uda())
            .field("usa", &self.usa())
            .field("ub1", &self.ub1())
            .field("ut2", &self.ut2())
            .field("ut1", &self.ut1())
            .finish()
    }
}
impl W {
    ///Bits 2:15 - pointer (16-bit low significant address) to the next linked-list data structure If UT1=UT2=UB1=USA=UDA=ULL=0 and if LA\[15:2\]=0: the current LLI is the last one. The channel transfer is completed without any update of the linked-list DMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure will be automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list DMA internal register file i.e. possibly LPDMA_CxCTR1, LPDMA_CxTR2, LPDMA_CxBR1, LPDMA_CxSAR, LPDMA_CxDAR and LPDMA_CxLLR. Note: The user should program the pointer to be 32-bit aligned. The two low significant bits are write ignored.
    #[inline(always)]
    pub fn la(&mut self) -> LA_W<C2LLRrs> {
        LA_W::new(self, 2)
    }
    ///Bit 16 - Update LPDMA_CxLLR from memory This bit controls the update of the LPDMA_CxLLR register from the memory during the link transfer. - 0: no LPDMA_CxLLR update - 1: LPDMA_CxLLR update
    #[inline(always)]
    pub fn ull(&mut self) -> ULL_W<C2LLRrs> {
        ULL_W::new(self, 16)
    }
    ///Bit 27 - Update LPDMA_CxDAR from memory This bit controls the update of the LPDMA_CxDAR register from the memory during the link transfer. - 0: no LPDMA_CxDAR update - 1: LPDMA_CxDAR update
    #[inline(always)]
    pub fn uda(&mut self) -> UDA_W<C2LLRrs> {
        UDA_W::new(self, 27)
    }
    ///Bit 28 - Update LPDMA_CxSAR from memory This bit controls the update of the LPDMA_CxSAR register from the memory during the link transfer. - 0: no LPDMA_CxSAR update - 1: LPDMA_CxSAR update
    #[inline(always)]
    pub fn usa(&mut self) -> USA_W<C2LLRrs> {
        USA_W::new(self, 28)
    }
    /**Bit 29 - Update LPDMA_CxBR1 from memory This bit controls the update of the LPDMA_CxBR1 register from the memory during the link transfer. If UB1=0 and if LPDMA_CxLLR != 0, the linked-list is not completed. Then LPDMA_CxBR1.BNDT\[15:0\]
    is restored to the programmed value after data transfer is completed and before the link transfer. - 0: no LPDMA_CxBR1 update (LPDMA_CxBR1.BNDT\[15:0\]
    is restored, if any link transfer) - 1: LPDMA_CxBR1 update*/
    #[inline(always)]
    pub fn ub1(&mut self) -> UB1_W<C2LLRrs> {
        UB1_W::new(self, 29)
    }
    ///Bit 30 - Update LPDMA_CxTR2 from memory This bit controls the update of the LPDMA_CxTR2 register from the memory during the link transfer. - 0: no LPDMA_CxTR2 update - 1: LPDMA_CxTR2 update
    #[inline(always)]
    pub fn ut2(&mut self) -> UT2_W<C2LLRrs> {
        UT2_W::new(self, 30)
    }
    ///Bit 31 - Update LPDMA_CxTR1 from memory This bit controls the update of the LPDMA_CxTR1 register from the memory during the link transfer. - 0: no LPDMA_CxTR1 update - 1: LPDMA_CxTR1 update
    #[inline(always)]
    pub fn ut1(&mut self) -> UT1_W<C2LLRrs> {
        UT1_W::new(self, 31)
    }
}
/**LPDMA channel x linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c2llr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2llr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#LPDMA1:C2LLR)*/
pub struct C2LLRrs;
impl crate::RegisterSpec for C2LLRrs {
    type Ux = u32;
}
///`read()` method returns [`c2llr::R`](R) reader structure
impl crate::Readable for C2LLRrs {}
///`write(|w| ..)` method takes [`c2llr::W`](W) writer structure
impl crate::Writable for C2LLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C2LLR to value 0
impl crate::Resettable for C2LLRrs {
    const RESET_VALUE: u32 = 0;
}