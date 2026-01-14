///Register `LLR` reader
pub type R = crate::R<LLRrs>;
///Register `LLR` writer
pub type W = crate::W<LLRrs>;
///Field `LA` reader - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\[15:20\] = 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored.
pub type LA_R = crate::FieldReader<u16>;
///Field `LA` writer - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\[15:20\] = 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored.
pub type LA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16, crate::Safe>;
///Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer.
pub use crate::stm32h503::gpdma1::ch::llr::ULL;
///Field `ULL` reader - Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer.
pub use crate::stm32h503::gpdma1::ch::llr::ULL_R;
///Field `ULL` writer - Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer.
pub use crate::stm32h503::gpdma1::ch::llr::ULL_W;
/**Update GPDMA_CxBR2 from memory This bit controls the update of GPDMA_CxBR2 from the memory during the link transfer.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UB2 {
    ///0: No CxBR2 update
    NoUpdate = 0,
    ///1: CxBR2 updated from memory during link transfer
    Update = 1,
}
impl From<UB2> for bool {
    #[inline(always)]
    fn from(variant: UB2) -> Self {
        variant as u8 != 0
    }
}
///Field `UB2` reader - Update GPDMA_CxBR2 from memory This bit controls the update of GPDMA_CxBR2 from the memory during the link transfer.
pub type UB2_R = crate::BitReader<UB2>;
impl UB2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UB2 {
        match self.bits {
            false => UB2::NoUpdate,
            true => UB2::Update,
        }
    }
    ///No CxBR2 update
    #[inline(always)]
    pub fn is_no_update(&self) -> bool {
        *self == UB2::NoUpdate
    }
    ///CxBR2 updated from memory during link transfer
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == UB2::Update
    }
}
///Field `UB2` writer - Update GPDMA_CxBR2 from memory This bit controls the update of GPDMA_CxBR2 from the memory during the link transfer.
pub type UB2_W<'a, REG> = crate::BitWriter<'a, REG, UB2>;
impl<'a, REG> UB2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No CxBR2 update
    #[inline(always)]
    pub fn no_update(self) -> &'a mut crate::W<REG> {
        self.variant(UB2::NoUpdate)
    }
    ///CxBR2 updated from memory during link transfer
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(UB2::Update)
    }
}
/**Update GPDMA_CxTR3 from memory This bit controls the update of GPDMA_CxTR3 from the memory during the link transfer.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UT3 {
    ///0: No CxTR3 update
    NoUpdate = 0,
    ///1: CxTR3 updated from memory during link transfer
    Update = 1,
}
impl From<UT3> for bool {
    #[inline(always)]
    fn from(variant: UT3) -> Self {
        variant as u8 != 0
    }
}
///Field `UT3` reader - Update GPDMA_CxTR3 from memory This bit controls the update of GPDMA_CxTR3 from the memory during the link transfer.
pub type UT3_R = crate::BitReader<UT3>;
impl UT3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UT3 {
        match self.bits {
            false => UT3::NoUpdate,
            true => UT3::Update,
        }
    }
    ///No CxTR3 update
    #[inline(always)]
    pub fn is_no_update(&self) -> bool {
        *self == UT3::NoUpdate
    }
    ///CxTR3 updated from memory during link transfer
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == UT3::Update
    }
}
///Field `UT3` writer - Update GPDMA_CxTR3 from memory This bit controls the update of GPDMA_CxTR3 from the memory during the link transfer.
pub type UT3_W<'a, REG> = crate::BitWriter<'a, REG, UT3>;
impl<'a, REG> UT3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No CxTR3 update
    #[inline(always)]
    pub fn no_update(self) -> &'a mut crate::W<REG> {
        self.variant(UT3::NoUpdate)
    }
    ///CxTR3 updated from memory during link transfer
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(UT3::Update)
    }
}
///Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR different 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\[15:0\] is then restored to the programmed value after data transfer is completed and before the link transfer.
pub use crate::stm32h503::gpdma1::ch::llr::UB1;
///Field `UB1` reader - Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR different 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\[15:0\] is then restored to the programmed value after data transfer is completed and before the link transfer.
pub use crate::stm32h503::gpdma1::ch::llr::UB1_R;
///Field `UB1` writer - Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR different 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\[15:0\] is then restored to the programmed value after data transfer is completed and before the link transfer.
pub use crate::stm32h503::gpdma1::ch::llr::UB1_W;
///Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer.
pub use crate::stm32h503::gpdma1::ch::llr::UDA;
///Field `UDA` reader - Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer.
pub use crate::stm32h503::gpdma1::ch::llr::UDA_R;
///Field `UDA` writer - Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer.
pub use crate::stm32h503::gpdma1::ch::llr::UDA_W;
///update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer.
pub use crate::stm32h503::gpdma1::ch::llr::USA;
///Field `USA` reader - update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer.
pub use crate::stm32h503::gpdma1::ch::llr::USA_R;
///Field `USA` writer - update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer.
pub use crate::stm32h503::gpdma1::ch::llr::USA_W;
///Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer.
pub use crate::stm32h503::gpdma1::ch::llr::UT1;
///Field `UT1` reader - Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer.
pub use crate::stm32h503::gpdma1::ch::llr::UT1_R;
///Field `UT1` writer - Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer.
pub use crate::stm32h503::gpdma1::ch::llr::UT1_W;
///Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer.
pub use crate::stm32h503::gpdma1::ch::llr::UT2;
///Field `UT2` reader - Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer.
pub use crate::stm32h503::gpdma1::ch::llr::UT2_R;
///Field `UT2` writer - Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer.
pub use crate::stm32h503::gpdma1::ch::llr::UT2_W;
impl R {
    ///Bits 2:15 - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\[15:20\] = 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored.
    #[inline(always)]
    pub fn la(&self) -> LA_R {
        LA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bit 16 - Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer.
    #[inline(always)]
    pub fn ull(&self) -> ULL_R {
        ULL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 25 - Update GPDMA_CxBR2 from memory This bit controls the update of GPDMA_CxBR2 from the memory during the link transfer.
    #[inline(always)]
    pub fn ub2(&self) -> UB2_R {
        UB2_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Update GPDMA_CxTR3 from memory This bit controls the update of GPDMA_CxTR3 from the memory during the link transfer.
    #[inline(always)]
    pub fn ut3(&self) -> UT3_R {
        UT3_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer.
    #[inline(always)]
    pub fn uda(&self) -> UDA_R {
        UDA_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer.
    #[inline(always)]
    pub fn usa(&self) -> USA_R {
        USA_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR different 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\[15:0\] is then restored to the programmed value after data transfer is completed and before the link transfer.
    #[inline(always)]
    pub fn ub1(&self) -> UB1_R {
        UB1_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer.
    #[inline(always)]
    pub fn ut2(&self) -> UT2_R {
        UT2_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer.
    #[inline(always)]
    pub fn ut1(&self) -> UT1_R {
        UT1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LLR")
            .field("la", &self.la())
            .field("ull", &self.ull())
            .field("ub2", &self.ub2())
            .field("ut3", &self.ut3())
            .field("uda", &self.uda())
            .field("usa", &self.usa())
            .field("ub1", &self.ub1())
            .field("ut2", &self.ut2())
            .field("ut1", &self.ut1())
            .finish()
    }
}
impl W {
    ///Bits 2:15 - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\[15:20\] = 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored.
    #[inline(always)]
    pub fn la(&mut self) -> LA_W<'_, LLRrs> {
        LA_W::new(self, 2)
    }
    ///Bit 16 - Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer.
    #[inline(always)]
    pub fn ull(&mut self) -> ULL_W<'_, LLRrs> {
        ULL_W::new(self, 16)
    }
    ///Bit 25 - Update GPDMA_CxBR2 from memory This bit controls the update of GPDMA_CxBR2 from the memory during the link transfer.
    #[inline(always)]
    pub fn ub2(&mut self) -> UB2_W<'_, LLRrs> {
        UB2_W::new(self, 25)
    }
    ///Bit 26 - Update GPDMA_CxTR3 from memory This bit controls the update of GPDMA_CxTR3 from the memory during the link transfer.
    #[inline(always)]
    pub fn ut3(&mut self) -> UT3_W<'_, LLRrs> {
        UT3_W::new(self, 26)
    }
    ///Bit 27 - Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer.
    #[inline(always)]
    pub fn uda(&mut self) -> UDA_W<'_, LLRrs> {
        UDA_W::new(self, 27)
    }
    ///Bit 28 - update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer.
    #[inline(always)]
    pub fn usa(&mut self) -> USA_W<'_, LLRrs> {
        USA_W::new(self, 28)
    }
    ///Bit 29 - Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR different 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\[15:0\] is then restored to the programmed value after data transfer is completed and before the link transfer.
    #[inline(always)]
    pub fn ub1(&mut self) -> UB1_W<'_, LLRrs> {
        UB1_W::new(self, 29)
    }
    ///Bit 30 - Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer.
    #[inline(always)]
    pub fn ut2(&mut self) -> UT2_W<'_, LLRrs> {
        UT2_W::new(self, 30)
    }
    ///Bit 31 - Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer.
    #[inline(always)]
    pub fn ut1(&mut self) -> UT1_W<'_, LLRrs> {
        UT1_W::new(self, 31)
    }
}
/**GPDMA channel 6 alternate linked-list address register

You can [`read`](crate::Reg::read) this register and get [`llr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LLRrs;
impl crate::RegisterSpec for LLRrs {
    type Ux = u32;
}
///`read()` method returns [`llr::R`](R) reader structure
impl crate::Readable for LLRrs {}
///`write(|w| ..)` method takes [`llr::W`](W) writer structure
impl crate::Writable for LLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LLR to value 0
impl crate::Resettable for LLRrs {}
