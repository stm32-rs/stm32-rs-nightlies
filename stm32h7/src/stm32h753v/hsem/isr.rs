#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ISEM0` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM0_R(crate::FieldReader<bool, bool>);
impl ISEM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM1` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM1_R(crate::FieldReader<bool, bool>);
impl ISEM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM2` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM2_R(crate::FieldReader<bool, bool>);
impl ISEM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM3` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM3_R(crate::FieldReader<bool, bool>);
impl ISEM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM4` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM4_R(crate::FieldReader<bool, bool>);
impl ISEM4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM5` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM5_R(crate::FieldReader<bool, bool>);
impl ISEM5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM6` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM6_R(crate::FieldReader<bool, bool>);
impl ISEM6_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM7` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM7_R(crate::FieldReader<bool, bool>);
impl ISEM7_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM8` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM8_R(crate::FieldReader<bool, bool>);
impl ISEM8_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM9` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM9_R(crate::FieldReader<bool, bool>);
impl ISEM9_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM10` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM10_R(crate::FieldReader<bool, bool>);
impl ISEM10_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM11` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM11_R(crate::FieldReader<bool, bool>);
impl ISEM11_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM12` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM12_R(crate::FieldReader<bool, bool>);
impl ISEM12_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM13` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM13_R(crate::FieldReader<bool, bool>);
impl ISEM13_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM14` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM14_R(crate::FieldReader<bool, bool>);
impl ISEM14_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM15` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM15_R(crate::FieldReader<bool, bool>);
impl ISEM15_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM16` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM16_R(crate::FieldReader<bool, bool>);
impl ISEM16_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM17` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM17_R(crate::FieldReader<bool, bool>);
impl ISEM17_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM18` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM18_R(crate::FieldReader<bool, bool>);
impl ISEM18_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM19` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM19_R(crate::FieldReader<bool, bool>);
impl ISEM19_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM20` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM20_R(crate::FieldReader<bool, bool>);
impl ISEM20_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM21` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM21_R(crate::FieldReader<bool, bool>);
impl ISEM21_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM22` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM22_R(crate::FieldReader<bool, bool>);
impl ISEM22_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM23` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM23_R(crate::FieldReader<bool, bool>);
impl ISEM23_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM24` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM24_R(crate::FieldReader<bool, bool>);
impl ISEM24_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM25` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM25_R(crate::FieldReader<bool, bool>);
impl ISEM25_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM26` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM26_R(crate::FieldReader<bool, bool>);
impl ISEM26_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM27` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM27_R(crate::FieldReader<bool, bool>);
impl ISEM27_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM28` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM28_R(crate::FieldReader<bool, bool>);
impl ISEM28_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM29` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM29_R(crate::FieldReader<bool, bool>);
impl ISEM29_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM30` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM30_R(crate::FieldReader<bool, bool>);
impl ISEM30_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM31` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub struct ISEM31_R(crate::FieldReader<bool, bool>);
impl ISEM31_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem0(&self) -> ISEM0_R {
        ISEM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem1(&self) -> ISEM1_R {
        ISEM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem2(&self) -> ISEM2_R {
        ISEM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem3(&self) -> ISEM3_R {
        ISEM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem4(&self) -> ISEM4_R {
        ISEM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem5(&self) -> ISEM5_R {
        ISEM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem6(&self) -> ISEM6_R {
        ISEM6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem7(&self) -> ISEM7_R {
        ISEM7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem8(&self) -> ISEM8_R {
        ISEM8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem9(&self) -> ISEM9_R {
        ISEM9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem10(&self) -> ISEM10_R {
        ISEM10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem11(&self) -> ISEM11_R {
        ISEM11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem12(&self) -> ISEM12_R {
        ISEM12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem13(&self) -> ISEM13_R {
        ISEM13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem14(&self) -> ISEM14_R {
        ISEM14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem15(&self) -> ISEM15_R {
        ISEM15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem16(&self) -> ISEM16_R {
        ISEM16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem17(&self) -> ISEM17_R {
        ISEM17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem18(&self) -> ISEM18_R {
        ISEM18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem19(&self) -> ISEM19_R {
        ISEM19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem20(&self) -> ISEM20_R {
        ISEM20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem21(&self) -> ISEM21_R {
        ISEM21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem22(&self) -> ISEM22_R {
        ISEM22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem23(&self) -> ISEM23_R {
        ISEM23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem24(&self) -> ISEM24_R {
        ISEM24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem25(&self) -> ISEM25_R {
        ISEM25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem26(&self) -> ISEM26_R {
        ISEM26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem27(&self) -> ISEM27_R {
        ISEM27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem28(&self) -> ISEM28_R {
        ISEM28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem29(&self) -> ISEM29_R {
        ISEM29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem30(&self) -> ISEM30_R {
        ISEM30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem31(&self) -> ISEM31_R {
        ISEM31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "HSEM Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
