#[doc = "Reader of register HSEM_ISR"]
pub type R = crate::R<u32, super::HSEM_ISR>;
#[doc = "Reader of field `ISF0`"]
pub type ISF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISF1`"]
pub type ISF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISF2`"]
pub type ISF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISF3`"]
pub type ISF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISF4`"]
pub type ISF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISF5`"]
pub type ISF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISF6`"]
pub type ISF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISF7`"]
pub type ISF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISF8`"]
pub type ISF8_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISF9`"]
pub type ISF9_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISF10`"]
pub type ISF10_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISF11`"]
pub type ISF11_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISF12`"]
pub type ISF12_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISF13`"]
pub type ISF13_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISF14`"]
pub type ISF14_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISF15`"]
pub type ISF15_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf0(&self) -> ISF0_R {
        ISF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf1(&self) -> ISF1_R {
        ISF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf2(&self) -> ISF2_R {
        ISF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf3(&self) -> ISF3_R {
        ISF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf4(&self) -> ISF4_R {
        ISF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf5(&self) -> ISF5_R {
        ISF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf6(&self) -> ISF6_R {
        ISF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf7(&self) -> ISF7_R {
        ISF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf8(&self) -> ISF8_R {
        ISF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf9(&self) -> ISF9_R {
        ISF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf10(&self) -> ISF10_R {
        ISF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf11(&self) -> ISF11_R {
        ISF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf12(&self) -> ISF12_R {
        ISF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf13(&self) -> ISF13_R {
        ISF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf14(&self) -> ISF14_R {
        ISF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf15(&self) -> ISF15_R {
        ISF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
