#[doc = "Reader of register DDRCTRL_POISONSTAT"]
pub type R = crate::R<u32, super::DDRCTRL_POISONSTAT>;
#[doc = "Reader of field `WR_POISON_INTR_0`"]
pub type WR_POISON_INTR_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `WR_POISON_INTR_1`"]
pub type WR_POISON_INTR_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `RD_POISON_INTR_0`"]
pub type RD_POISON_INTR_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `RD_POISON_INTR_1`"]
pub type RD_POISON_INTR_1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - WR_POISON_INTR_0"]
    #[inline(always)]
    pub fn wr_poison_intr_0(&self) -> WR_POISON_INTR_0_R {
        WR_POISON_INTR_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WR_POISON_INTR_1"]
    #[inline(always)]
    pub fn wr_poison_intr_1(&self) -> WR_POISON_INTR_1_R {
        WR_POISON_INTR_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RD_POISON_INTR_0"]
    #[inline(always)]
    pub fn rd_poison_intr_0(&self) -> RD_POISON_INTR_0_R {
        RD_POISON_INTR_0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RD_POISON_INTR_1"]
    #[inline(always)]
    pub fn rd_poison_intr_1(&self) -> RD_POISON_INTR_1_R {
        RD_POISON_INTR_1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
