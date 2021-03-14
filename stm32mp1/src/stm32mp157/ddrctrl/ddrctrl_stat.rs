#[doc = "Reader of register DDRCTRL_STAT"]
pub type R = crate::R<u32, super::DDRCTRL_STAT>;
#[doc = "Reader of field `OPERATING_MODE`"]
pub type OPERATING_MODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SELFREF_TYPE`"]
pub type SELFREF_TYPE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SELFREF_CAM_NOT_EMPTY`"]
pub type SELFREF_CAM_NOT_EMPTY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:2 - OPERATING_MODE"]
    #[inline(always)]
    pub fn operating_mode(&self) -> OPERATING_MODE_R {
        OPERATING_MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - SELFREF_TYPE"]
    #[inline(always)]
    pub fn selfref_type(&self) -> SELFREF_TYPE_R {
        SELFREF_TYPE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 12 - SELFREF_CAM_NOT_EMPTY"]
    #[inline(always)]
    pub fn selfref_cam_not_empty(&self) -> SELFREF_CAM_NOT_EMPTY_R {
        SELFREF_CAM_NOT_EMPTY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
