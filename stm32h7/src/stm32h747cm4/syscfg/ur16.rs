#[doc = "Reader of register UR16"]
pub type R = crate::R<u32, super::UR16>;
#[doc = "Reader of field `FZIWDGSTP`"]
pub type FZIWDGSTP_R = crate::R<bool, bool>;
#[doc = "Reader of field `PKP`"]
pub type PKP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Freeze independent watchdog in Stop mode"]
    #[inline(always)]
    pub fn fziwdgstp(&self) -> FZIWDGSTP_R {
        FZIWDGSTP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Private key programmed"]
    #[inline(always)]
    pub fn pkp(&self) -> PKP_R {
        PKP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
