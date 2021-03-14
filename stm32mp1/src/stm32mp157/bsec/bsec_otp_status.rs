#[doc = "Reader of register BSEC_OTP_STATUS"]
pub type R = crate::R<u32, super::BSEC_OTP_STATUS>;
#[doc = "Reader of field `SECURE`"]
pub type SECURE_R = crate::R<bool, bool>;
#[doc = "Reader of field `FULLDBG`"]
pub type FULLDBG_R = crate::R<bool, bool>;
#[doc = "Reader of field `INVALID`"]
pub type INVALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `PROGFAIL`"]
pub type PROGFAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWRON`"]
pub type PWRON_R = crate::R<bool, bool>;
#[doc = "Reader of field `BIST1LOCK`"]
pub type BIST1LOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `BIST2LOCK`"]
pub type BIST2LOCK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SECURE"]
    #[inline(always)]
    pub fn secure(&self) -> SECURE_R {
        SECURE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FULLDBG"]
    #[inline(always)]
    pub fn fulldbg(&self) -> FULLDBG_R {
        FULLDBG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - INVALID"]
    #[inline(always)]
    pub fn invalid(&self) -> INVALID_R {
        INVALID_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PROGFAIL"]
    #[inline(always)]
    pub fn progfail(&self) -> PROGFAIL_R {
        PROGFAIL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWRON"]
    #[inline(always)]
    pub fn pwron(&self) -> PWRON_R {
        PWRON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BIST1LOCK"]
    #[inline(always)]
    pub fn bist1lock(&self) -> BIST1LOCK_R {
        BIST1LOCK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BIST2LOCK"]
    #[inline(always)]
    pub fn bist2lock(&self) -> BIST2LOCK_R {
        BIST2LOCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
