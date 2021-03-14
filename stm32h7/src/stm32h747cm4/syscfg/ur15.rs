#[doc = "Reader of register UR15"]
pub type R = crate::R<u32, super::UR15>;
#[doc = "Reader of field `FZIWDGSTB`"]
pub type FZIWDGSTB_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 16 - Freeze independent watchdog in Standby mode"]
    #[inline(always)]
    pub fn fziwdgstb(&self) -> FZIWDGSTB_R {
        FZIWDGSTB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
