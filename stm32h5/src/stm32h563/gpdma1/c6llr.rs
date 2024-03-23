#[doc = "Register `C6LLR` reader"]
pub type R = crate::R<C6LLRrs>;
#[doc = "Register `C6LLR` writer"]
pub type W = crate::W<C6LLRrs>;
#[doc = "Field `LA` reader - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\\[15:20\\]
= 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored."]
pub type LA_R = crate::FieldReader<u16>;
#[doc = "Field `LA` writer - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\\[15:20\\]
= 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored."]
pub type LA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 14, u16>;
#[doc = "Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULL {
    #[doc = "0: No CxLLR update"]
    NoUpdate = 0,
    #[doc = "1: CxLLR updated from memory during link transfer"]
    Update = 1,
}
impl From<ULL> for bool {
    #[inline(always)]
    fn from(variant: ULL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULL` reader - Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer."]
pub type ULL_R = crate::BitReader<ULL>;
impl ULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ULL {
        match self.bits {
            false => ULL::NoUpdate,
            true => ULL::Update,
        }
    }
    #[doc = "No CxLLR update"]
    #[inline(always)]
    pub fn is_no_update(&self) -> bool {
        *self == ULL::NoUpdate
    }
    #[doc = "CxLLR updated from memory during link transfer"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == ULL::Update
    }
}
#[doc = "Field `ULL` writer - Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer."]
pub type ULL_W<'a, REG> = crate::BitWriter<'a, REG, ULL>;
impl<'a, REG> ULL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No CxLLR update"]
    #[inline(always)]
    pub fn no_update(self) -> &'a mut crate::W<REG> {
        self.variant(ULL::NoUpdate)
    }
    #[doc = "CxLLR updated from memory during link transfer"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(ULL::Update)
    }
}
#[doc = "Update GPDMA_CxBR2 from memory This bit controls the update of GPDMA_CxBR2 from the memory during the link transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UB2 {
    #[doc = "0: No CxBR2 update"]
    NoUpdate = 0,
    #[doc = "1: CxBR2 updated from memory during link transfer"]
    Update = 1,
}
impl From<UB2> for bool {
    #[inline(always)]
    fn from(variant: UB2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UB2` reader - Update GPDMA_CxBR2 from memory This bit controls the update of GPDMA_CxBR2 from the memory during the link transfer."]
pub type UB2_R = crate::BitReader<UB2>;
impl UB2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UB2 {
        match self.bits {
            false => UB2::NoUpdate,
            true => UB2::Update,
        }
    }
    #[doc = "No CxBR2 update"]
    #[inline(always)]
    pub fn is_no_update(&self) -> bool {
        *self == UB2::NoUpdate
    }
    #[doc = "CxBR2 updated from memory during link transfer"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == UB2::Update
    }
}
#[doc = "Field `UB2` writer - Update GPDMA_CxBR2 from memory This bit controls the update of GPDMA_CxBR2 from the memory during the link transfer."]
pub type UB2_W<'a, REG> = crate::BitWriter<'a, REG, UB2>;
impl<'a, REG> UB2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No CxBR2 update"]
    #[inline(always)]
    pub fn no_update(self) -> &'a mut crate::W<REG> {
        self.variant(UB2::NoUpdate)
    }
    #[doc = "CxBR2 updated from memory during link transfer"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(UB2::Update)
    }
}
#[doc = "Update GPDMA_CxTR3 from memory This bit controls the update of GPDMA_CxTR3 from the memory during the link transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UT3 {
    #[doc = "0: No CxTR3 update"]
    NoUpdate = 0,
    #[doc = "1: CxTR3 updated from memory during link transfer"]
    Update = 1,
}
impl From<UT3> for bool {
    #[inline(always)]
    fn from(variant: UT3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UT3` reader - Update GPDMA_CxTR3 from memory This bit controls the update of GPDMA_CxTR3 from the memory during the link transfer."]
pub type UT3_R = crate::BitReader<UT3>;
impl UT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UT3 {
        match self.bits {
            false => UT3::NoUpdate,
            true => UT3::Update,
        }
    }
    #[doc = "No CxTR3 update"]
    #[inline(always)]
    pub fn is_no_update(&self) -> bool {
        *self == UT3::NoUpdate
    }
    #[doc = "CxTR3 updated from memory during link transfer"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == UT3::Update
    }
}
#[doc = "Field `UT3` writer - Update GPDMA_CxTR3 from memory This bit controls the update of GPDMA_CxTR3 from the memory during the link transfer."]
pub type UT3_W<'a, REG> = crate::BitWriter<'a, REG, UT3>;
impl<'a, REG> UT3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No CxTR3 update"]
    #[inline(always)]
    pub fn no_update(self) -> &'a mut crate::W<REG> {
        self.variant(UT3::NoUpdate)
    }
    #[doc = "CxTR3 updated from memory during link transfer"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(UT3::Update)
    }
}
#[doc = "Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDA {
    #[doc = "0: No CxDAR update"]
    NoUpdate = 0,
    #[doc = "1: CxDAR updated from memory during link transfer"]
    Update = 1,
}
impl From<UDA> for bool {
    #[inline(always)]
    fn from(variant: UDA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDA` reader - Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer."]
pub type UDA_R = crate::BitReader<UDA>;
impl UDA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UDA {
        match self.bits {
            false => UDA::NoUpdate,
            true => UDA::Update,
        }
    }
    #[doc = "No CxDAR update"]
    #[inline(always)]
    pub fn is_no_update(&self) -> bool {
        *self == UDA::NoUpdate
    }
    #[doc = "CxDAR updated from memory during link transfer"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == UDA::Update
    }
}
#[doc = "Field `UDA` writer - Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer."]
pub type UDA_W<'a, REG> = crate::BitWriter<'a, REG, UDA>;
impl<'a, REG> UDA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No CxDAR update"]
    #[inline(always)]
    pub fn no_update(self) -> &'a mut crate::W<REG> {
        self.variant(UDA::NoUpdate)
    }
    #[doc = "CxDAR updated from memory during link transfer"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(UDA::Update)
    }
}
#[doc = "update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USA {
    #[doc = "0: No CxSAR update"]
    NoUpdate = 0,
    #[doc = "1: CxSAR updated from memory during link transfer"]
    Update = 1,
}
impl From<USA> for bool {
    #[inline(always)]
    fn from(variant: USA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USA` reader - update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer."]
pub type USA_R = crate::BitReader<USA>;
impl USA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USA {
        match self.bits {
            false => USA::NoUpdate,
            true => USA::Update,
        }
    }
    #[doc = "No CxSAR update"]
    #[inline(always)]
    pub fn is_no_update(&self) -> bool {
        *self == USA::NoUpdate
    }
    #[doc = "CxSAR updated from memory during link transfer"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == USA::Update
    }
}
#[doc = "Field `USA` writer - update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer."]
pub type USA_W<'a, REG> = crate::BitWriter<'a, REG, USA>;
impl<'a, REG> USA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No CxSAR update"]
    #[inline(always)]
    pub fn no_update(self) -> &'a mut crate::W<REG> {
        self.variant(USA::NoUpdate)
    }
    #[doc = "CxSAR updated from memory during link transfer"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(USA::Update)
    }
}
#[doc = "Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR ≠ 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\\[15:0\\]
is then restored to the programmed value after data transfer is completed and before the link transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UB1 {
    #[doc = "0: No CxBR1 update"]
    NoUpdate = 0,
    #[doc = "1: CxBR1 updated from memory during link transfer"]
    Update = 1,
}
impl From<UB1> for bool {
    #[inline(always)]
    fn from(variant: UB1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UB1` reader - Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR ≠ 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\\[15:0\\]
is then restored to the programmed value after data transfer is completed and before the link transfer."]
pub type UB1_R = crate::BitReader<UB1>;
impl UB1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UB1 {
        match self.bits {
            false => UB1::NoUpdate,
            true => UB1::Update,
        }
    }
    #[doc = "No CxBR1 update"]
    #[inline(always)]
    pub fn is_no_update(&self) -> bool {
        *self == UB1::NoUpdate
    }
    #[doc = "CxBR1 updated from memory during link transfer"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == UB1::Update
    }
}
#[doc = "Field `UB1` writer - Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR ≠ 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\\[15:0\\]
is then restored to the programmed value after data transfer is completed and before the link transfer."]
pub type UB1_W<'a, REG> = crate::BitWriter<'a, REG, UB1>;
impl<'a, REG> UB1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No CxBR1 update"]
    #[inline(always)]
    pub fn no_update(self) -> &'a mut crate::W<REG> {
        self.variant(UB1::NoUpdate)
    }
    #[doc = "CxBR1 updated from memory during link transfer"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(UB1::Update)
    }
}
#[doc = "Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UT2 {
    #[doc = "0: No CxTR2 update"]
    NoUpdate = 0,
    #[doc = "1: CxTR2 updated from memory during link transfer"]
    Update = 1,
}
impl From<UT2> for bool {
    #[inline(always)]
    fn from(variant: UT2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UT2` reader - Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer."]
pub type UT2_R = crate::BitReader<UT2>;
impl UT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UT2 {
        match self.bits {
            false => UT2::NoUpdate,
            true => UT2::Update,
        }
    }
    #[doc = "No CxTR2 update"]
    #[inline(always)]
    pub fn is_no_update(&self) -> bool {
        *self == UT2::NoUpdate
    }
    #[doc = "CxTR2 updated from memory during link transfer"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == UT2::Update
    }
}
#[doc = "Field `UT2` writer - Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer."]
pub type UT2_W<'a, REG> = crate::BitWriter<'a, REG, UT2>;
impl<'a, REG> UT2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No CxTR2 update"]
    #[inline(always)]
    pub fn no_update(self) -> &'a mut crate::W<REG> {
        self.variant(UT2::NoUpdate)
    }
    #[doc = "CxTR2 updated from memory during link transfer"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(UT2::Update)
    }
}
#[doc = "Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UT1 {
    #[doc = "0: No CxTR1 update"]
    NoUpdate = 0,
    #[doc = "1: CxTR1 updated from memory during link transfer"]
    Update = 1,
}
impl From<UT1> for bool {
    #[inline(always)]
    fn from(variant: UT1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UT1` reader - Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer."]
pub type UT1_R = crate::BitReader<UT1>;
impl UT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UT1 {
        match self.bits {
            false => UT1::NoUpdate,
            true => UT1::Update,
        }
    }
    #[doc = "No CxTR1 update"]
    #[inline(always)]
    pub fn is_no_update(&self) -> bool {
        *self == UT1::NoUpdate
    }
    #[doc = "CxTR1 updated from memory during link transfer"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == UT1::Update
    }
}
#[doc = "Field `UT1` writer - Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer."]
pub type UT1_W<'a, REG> = crate::BitWriter<'a, REG, UT1>;
impl<'a, REG> UT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No CxTR1 update"]
    #[inline(always)]
    pub fn no_update(self) -> &'a mut crate::W<REG> {
        self.variant(UT1::NoUpdate)
    }
    #[doc = "CxTR1 updated from memory during link transfer"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(UT1::Update)
    }
}
impl R {
    #[doc = "Bits 2:15 - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\\[15:20\\]
= 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored."]
    #[inline(always)]
    pub fn la(&self) -> LA_R {
        LA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer."]
    #[inline(always)]
    pub fn ull(&self) -> ULL_R {
        ULL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 25 - Update GPDMA_CxBR2 from memory This bit controls the update of GPDMA_CxBR2 from the memory during the link transfer."]
    #[inline(always)]
    pub fn ub2(&self) -> UB2_R {
        UB2_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Update GPDMA_CxTR3 from memory This bit controls the update of GPDMA_CxTR3 from the memory during the link transfer."]
    #[inline(always)]
    pub fn ut3(&self) -> UT3_R {
        UT3_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer."]
    #[inline(always)]
    pub fn uda(&self) -> UDA_R {
        UDA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer."]
    #[inline(always)]
    pub fn usa(&self) -> USA_R {
        USA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR ≠ 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\\[15:0\\]
is then restored to the programmed value after data transfer is completed and before the link transfer."]
    #[inline(always)]
    pub fn ub1(&self) -> UB1_R {
        UB1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer."]
    #[inline(always)]
    pub fn ut2(&self) -> UT2_R {
        UT2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer."]
    #[inline(always)]
    pub fn ut1(&self) -> UT1_R {
        UT1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\\[15:20\\]
= 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (GPDMA_CxCTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and GPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored."]
    #[inline(always)]
    #[must_use]
    pub fn la(&mut self) -> LA_W<C6LLRrs> {
        LA_W::new(self, 2)
    }
    #[doc = "Bit 16 - Update GPDMA_CxLLR register from memory This bit is used to control the update of GPDMA_CxLLR from the memory during the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn ull(&mut self) -> ULL_W<C6LLRrs> {
        ULL_W::new(self, 16)
    }
    #[doc = "Bit 25 - Update GPDMA_CxBR2 from memory This bit controls the update of GPDMA_CxBR2 from the memory during the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn ub2(&mut self) -> UB2_W<C6LLRrs> {
        UB2_W::new(self, 25)
    }
    #[doc = "Bit 26 - Update GPDMA_CxTR3 from memory This bit controls the update of GPDMA_CxTR3 from the memory during the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn ut3(&mut self) -> UT3_W<C6LLRrs> {
        UT3_W::new(self, 26)
    }
    #[doc = "Bit 27 - Update GPDMA_CxDAR register from memory This bit is used to control the update of GPDMA_CxDAR from the memory during the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn uda(&mut self) -> UDA_W<C6LLRrs> {
        UDA_W::new(self, 27)
    }
    #[doc = "Bit 28 - update GPDMA_CxSAR from memory This bit controls the update of GPDMA_CxSAR from the memory during the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn usa(&mut self) -> USA_W<C6LLRrs> {
        USA_W::new(self, 28)
    }
    #[doc = "Bit 29 - Update GPDMA_CxBR1 from memory This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer. If UB1 = 0 and if GPDMA_CxLLR ≠ 0, the linked-list is not completed. GPDMA_CxBR1.BNDT\\[15:0\\]
is then restored to the programmed value after data transfer is completed and before the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn ub1(&mut self) -> UB1_W<C6LLRrs> {
        UB1_W::new(self, 29)
    }
    #[doc = "Bit 30 - Update GPDMA_CxTR2 from memory This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn ut2(&mut self) -> UT2_W<C6LLRrs> {
        UT2_W::new(self, 30)
    }
    #[doc = "Bit 31 - Update GPDMA_CxTR1 from memory This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer."]
    #[inline(always)]
    #[must_use]
    pub fn ut1(&mut self) -> UT1_W<C6LLRrs> {
        UT1_W::new(self, 31)
    }
}
#[doc = "GPDMA channel 6 alternate linked-list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c6llr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c6llr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C6LLRrs;
impl crate::RegisterSpec for C6LLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c6llr::R`](R) reader structure"]
impl crate::Readable for C6LLRrs {}
#[doc = "`write(|w| ..)` method takes [`c6llr::W`](W) writer structure"]
impl crate::Writable for C6LLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C6LLR to value 0"]
impl crate::Resettable for C6LLRrs {
    const RESET_VALUE: u32 = 0;
}
