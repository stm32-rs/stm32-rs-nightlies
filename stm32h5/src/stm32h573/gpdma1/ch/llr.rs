///Register `LLR` reader
pub type R = crate::R<LLRrs>;
///Register `LLR` writer
pub type W = crate::W<LLRrs>;
///Field `LA` reader - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\[15:20\] = 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored.
pub type LA_R = crate::FieldReader<u16>;
///Field `LA` writer - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\[15:20\] = 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored.
pub type LA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16, crate::Safe>;
/**Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULL {
    ///0: No CxLLR update
    NoUpdate = 0,
    ///1: CxLLR updated from memory during link transfer
    Update = 1,
}
impl From<ULL> for bool {
    #[inline(always)]
    fn from(variant: ULL) -> Self {
        variant as u8 != 0
    }
}
///Field `ULL` reader - Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer.
pub type ULL_R = crate::BitReader<ULL>;
impl ULL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ULL {
        match self.bits {
            false => ULL::NoUpdate,
            true => ULL::Update,
        }
    }
    ///No CxLLR update
    #[inline(always)]
    pub fn is_no_update(&self) -> bool {
        *self == ULL::NoUpdate
    }
    ///CxLLR updated from memory during link transfer
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == ULL::Update
    }
}
///Field `ULL` writer - Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer.
pub type ULL_W<'a, REG> = crate::BitWriter<'a, REG, ULL>;
impl<'a, REG> ULL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No CxLLR update
    #[inline(always)]
    pub fn no_update(self) -> &'a mut crate::W<REG> {
        self.variant(ULL::NoUpdate)
    }
    ///CxLLR updated from memory during link transfer
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(ULL::Update)
    }
}
/**Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDA {
    ///0: No CxDAR update
    NoUpdate = 0,
    ///1: CxDAR updated from memory during link transfer
    Update = 1,
}
impl From<UDA> for bool {
    #[inline(always)]
    fn from(variant: UDA) -> Self {
        variant as u8 != 0
    }
}
///Field `UDA` reader - Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer.
pub type UDA_R = crate::BitReader<UDA>;
impl UDA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UDA {
        match self.bits {
            false => UDA::NoUpdate,
            true => UDA::Update,
        }
    }
    ///No CxDAR update
    #[inline(always)]
    pub fn is_no_update(&self) -> bool {
        *self == UDA::NoUpdate
    }
    ///CxDAR updated from memory during link transfer
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == UDA::Update
    }
}
///Field `UDA` writer - Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer.
pub type UDA_W<'a, REG> = crate::BitWriter<'a, REG, UDA>;
impl<'a, REG> UDA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No CxDAR update
    #[inline(always)]
    pub fn no_update(self) -> &'a mut crate::W<REG> {
        self.variant(UDA::NoUpdate)
    }
    ///CxDAR updated from memory during link transfer
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(UDA::Update)
    }
}
/**update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USA {
    ///0: No CxSAR update
    NoUpdate = 0,
    ///1: CxSAR updated from memory during link transfer
    Update = 1,
}
impl From<USA> for bool {
    #[inline(always)]
    fn from(variant: USA) -> Self {
        variant as u8 != 0
    }
}
///Field `USA` reader - update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer.
pub type USA_R = crate::BitReader<USA>;
impl USA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USA {
        match self.bits {
            false => USA::NoUpdate,
            true => USA::Update,
        }
    }
    ///No CxSAR update
    #[inline(always)]
    pub fn is_no_update(&self) -> bool {
        *self == USA::NoUpdate
    }
    ///CxSAR updated from memory during link transfer
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == USA::Update
    }
}
///Field `USA` writer - update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer.
pub type USA_W<'a, REG> = crate::BitWriter<'a, REG, USA>;
impl<'a, REG> USA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No CxSAR update
    #[inline(always)]
    pub fn no_update(self) -> &'a mut crate::W<REG> {
        self.variant(USA::NoUpdate)
    }
    ///CxSAR updated from memory during link transfer
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(USA::Update)
    }
}
/**Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR different 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\[15:0\] is then restored to the programmed value after data transfer is completed and before the link transfer.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UB1 {
    ///0: No CxBR1 update
    NoUpdate = 0,
    ///1: CxBR1 updated from memory during link transfer
    Update = 1,
}
impl From<UB1> for bool {
    #[inline(always)]
    fn from(variant: UB1) -> Self {
        variant as u8 != 0
    }
}
///Field `UB1` reader - Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR different 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\[15:0\] is then restored to the programmed value after data transfer is completed and before the link transfer.
pub type UB1_R = crate::BitReader<UB1>;
impl UB1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UB1 {
        match self.bits {
            false => UB1::NoUpdate,
            true => UB1::Update,
        }
    }
    ///No CxBR1 update
    #[inline(always)]
    pub fn is_no_update(&self) -> bool {
        *self == UB1::NoUpdate
    }
    ///CxBR1 updated from memory during link transfer
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == UB1::Update
    }
}
///Field `UB1` writer - Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR different 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\[15:0\] is then restored to the programmed value after data transfer is completed and before the link transfer.
pub type UB1_W<'a, REG> = crate::BitWriter<'a, REG, UB1>;
impl<'a, REG> UB1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No CxBR1 update
    #[inline(always)]
    pub fn no_update(self) -> &'a mut crate::W<REG> {
        self.variant(UB1::NoUpdate)
    }
    ///CxBR1 updated from memory during link transfer
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(UB1::Update)
    }
}
/**Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UT2 {
    ///0: No CxTR2 update
    NoUpdate = 0,
    ///1: CxTR2 updated from memory during link transfer
    Update = 1,
}
impl From<UT2> for bool {
    #[inline(always)]
    fn from(variant: UT2) -> Self {
        variant as u8 != 0
    }
}
///Field `UT2` reader - Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer.
pub type UT2_R = crate::BitReader<UT2>;
impl UT2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UT2 {
        match self.bits {
            false => UT2::NoUpdate,
            true => UT2::Update,
        }
    }
    ///No CxTR2 update
    #[inline(always)]
    pub fn is_no_update(&self) -> bool {
        *self == UT2::NoUpdate
    }
    ///CxTR2 updated from memory during link transfer
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == UT2::Update
    }
}
///Field `UT2` writer - Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer.
pub type UT2_W<'a, REG> = crate::BitWriter<'a, REG, UT2>;
impl<'a, REG> UT2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No CxTR2 update
    #[inline(always)]
    pub fn no_update(self) -> &'a mut crate::W<REG> {
        self.variant(UT2::NoUpdate)
    }
    ///CxTR2 updated from memory during link transfer
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(UT2::Update)
    }
}
/**Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UT1 {
    ///0: No CxTR1 update
    NoUpdate = 0,
    ///1: CxTR1 updated from memory during link transfer
    Update = 1,
}
impl From<UT1> for bool {
    #[inline(always)]
    fn from(variant: UT1) -> Self {
        variant as u8 != 0
    }
}
///Field `UT1` reader - Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer.
pub type UT1_R = crate::BitReader<UT1>;
impl UT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UT1 {
        match self.bits {
            false => UT1::NoUpdate,
            true => UT1::Update,
        }
    }
    ///No CxTR1 update
    #[inline(always)]
    pub fn is_no_update(&self) -> bool {
        *self == UT1::NoUpdate
    }
    ///CxTR1 updated from memory during link transfer
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == UT1::Update
    }
}
///Field `UT1` writer - Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer.
pub type UT1_W<'a, REG> = crate::BitWriter<'a, REG, UT1>;
impl<'a, REG> UT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No CxTR1 update
    #[inline(always)]
    pub fn no_update(self) -> &'a mut crate::W<REG> {
        self.variant(UT1::NoUpdate)
    }
    ///CxTR1 updated from memory during link transfer
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(UT1::Update)
    }
}
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
/**GPDMA channel 0 linked-list address register

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
