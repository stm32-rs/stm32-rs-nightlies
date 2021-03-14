#[doc = "Reader of register RGCFR"]
pub type R = crate::R<u32, super::RGCFR>;
#[doc = "Reader of field `CSOF0`"]
pub type CSOF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSOF1`"]
pub type CSOF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSOF2`"]
pub type CSOF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSOF3`"]
pub type CSOF3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Generator Clear Overrun Flag 0"]
    #[inline(always)]
    pub fn csof0(&self) -> CSOF0_R {
        CSOF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Generator Clear Overrun Flag 1"]
    #[inline(always)]
    pub fn csof1(&self) -> CSOF1_R {
        CSOF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Generator Clear Overrun Flag 2"]
    #[inline(always)]
    pub fn csof2(&self) -> CSOF2_R {
        CSOF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Generator Clear Overrun Flag 3"]
    #[inline(always)]
    pub fn csof3(&self) -> CSOF3_R {
        CSOF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
