#[doc = "Reader of register RGSR"]
pub type R = crate::R<u32, super::RGSR>;
#[doc = "Reader of field `OF0`"]
pub type OF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `OF1`"]
pub type OF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `OF2`"]
pub type OF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `OF3`"]
pub type OF3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Generator Overrun Flag 0"]
    #[inline(always)]
    pub fn of0(&self) -> OF0_R {
        OF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Generator Overrun Flag 1"]
    #[inline(always)]
    pub fn of1(&self) -> OF1_R {
        OF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Generator Overrun Flag 2"]
    #[inline(always)]
    pub fn of2(&self) -> OF2_R {
        OF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Generator Overrun Flag 3"]
    #[inline(always)]
    pub fn of3(&self) -> OF3_R {
        OF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
