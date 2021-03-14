#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `YEMPTY`"]
pub type YEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `X1FULL`"]
pub type X1FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVFL`"]
pub type OVFL_R = crate::R<bool, bool>;
#[doc = "Reader of field `UNFL`"]
pub type UNFL_R = crate::R<bool, bool>;
#[doc = "Reader of field `SAT`"]
pub type SAT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - YEMPTY"]
    #[inline(always)]
    pub fn yempty(&self) -> YEMPTY_R {
        YEMPTY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - X1FULL"]
    #[inline(always)]
    pub fn x1full(&self) -> X1FULL_R {
        X1FULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OVFL"]
    #[inline(always)]
    pub fn ovfl(&self) -> OVFL_R {
        OVFL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UNFL"]
    #[inline(always)]
    pub fn unfl(&self) -> UNFL_R {
        UNFL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SAT"]
    #[inline(always)]
    pub fn sat(&self) -> SAT_R {
        SAT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
