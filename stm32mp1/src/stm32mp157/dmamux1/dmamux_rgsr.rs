#[doc = "Reader of register DMAMUX_RGSR"]
pub type R = crate::R<u32, super::DMAMUX_RGSR>;
#[doc = "Reader of field `OF0`"]
pub type OF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `OF1`"]
pub type OF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `OF2`"]
pub type OF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `OF3`"]
pub type OF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `OF4`"]
pub type OF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `OF5`"]
pub type OF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `OF6`"]
pub type OF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `OF7`"]
pub type OF7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - OF0"]
    #[inline(always)]
    pub fn of0(&self) -> OF0_R {
        OF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OF1"]
    #[inline(always)]
    pub fn of1(&self) -> OF1_R {
        OF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OF2"]
    #[inline(always)]
    pub fn of2(&self) -> OF2_R {
        OF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OF3"]
    #[inline(always)]
    pub fn of3(&self) -> OF3_R {
        OF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OF4"]
    #[inline(always)]
    pub fn of4(&self) -> OF4_R {
        OF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - OF5"]
    #[inline(always)]
    pub fn of5(&self) -> OF5_R {
        OF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OF6"]
    #[inline(always)]
    pub fn of6(&self) -> OF6_R {
        OF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - OF7"]
    #[inline(always)]
    pub fn of7(&self) -> OF7_R {
        OF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
