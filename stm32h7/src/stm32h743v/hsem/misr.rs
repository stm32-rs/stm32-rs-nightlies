#[doc = "Reader of register MISR"]
pub type R = crate::R<u32, super::MISR>;
#[doc = "Reader of field `ISEM0`"]
pub type ISEM0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM1`"]
pub type ISEM1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM2`"]
pub type ISEM2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM3`"]
pub type ISEM3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM4`"]
pub type ISEM4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM5`"]
pub type ISEM5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM6`"]
pub type ISEM6_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM7`"]
pub type ISEM7_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM8`"]
pub type ISEM8_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM9`"]
pub type ISEM9_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM10`"]
pub type ISEM10_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM11`"]
pub type ISEM11_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM12`"]
pub type ISEM12_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM13`"]
pub type ISEM13_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM14`"]
pub type ISEM14_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM15`"]
pub type ISEM15_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM16`"]
pub type ISEM16_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM17`"]
pub type ISEM17_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM18`"]
pub type ISEM18_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM19`"]
pub type ISEM19_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM20`"]
pub type ISEM20_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM21`"]
pub type ISEM21_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM22`"]
pub type ISEM22_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM23`"]
pub type ISEM23_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM24`"]
pub type ISEM24_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM25`"]
pub type ISEM25_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM26`"]
pub type ISEM26_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM27`"]
pub type ISEM27_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM28`"]
pub type ISEM28_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM29`"]
pub type ISEM29_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM30`"]
pub type ISEM30_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISEM31`"]
pub type ISEM31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem0(&self) -> ISEM0_R {
        ISEM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem1(&self) -> ISEM1_R {
        ISEM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem2(&self) -> ISEM2_R {
        ISEM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem3(&self) -> ISEM3_R {
        ISEM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem4(&self) -> ISEM4_R {
        ISEM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem5(&self) -> ISEM5_R {
        ISEM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem6(&self) -> ISEM6_R {
        ISEM6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem7(&self) -> ISEM7_R {
        ISEM7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem8(&self) -> ISEM8_R {
        ISEM8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem9(&self) -> ISEM9_R {
        ISEM9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem10(&self) -> ISEM10_R {
        ISEM10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem11(&self) -> ISEM11_R {
        ISEM11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem12(&self) -> ISEM12_R {
        ISEM12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem13(&self) -> ISEM13_R {
        ISEM13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem14(&self) -> ISEM14_R {
        ISEM14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem15(&self) -> ISEM15_R {
        ISEM15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem16(&self) -> ISEM16_R {
        ISEM16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem17(&self) -> ISEM17_R {
        ISEM17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem18(&self) -> ISEM18_R {
        ISEM18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem19(&self) -> ISEM19_R {
        ISEM19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem20(&self) -> ISEM20_R {
        ISEM20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem21(&self) -> ISEM21_R {
        ISEM21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem22(&self) -> ISEM22_R {
        ISEM22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem23(&self) -> ISEM23_R {
        ISEM23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem24(&self) -> ISEM24_R {
        ISEM24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem25(&self) -> ISEM25_R {
        ISEM25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem26(&self) -> ISEM26_R {
        ISEM26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem27(&self) -> ISEM27_R {
        ISEM27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem28(&self) -> ISEM28_R {
        ISEM28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem29(&self) -> ISEM29_R {
        ISEM29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem30(&self) -> ISEM30_R {
        ISEM30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn isem31(&self) -> ISEM31_R {
        ISEM31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
