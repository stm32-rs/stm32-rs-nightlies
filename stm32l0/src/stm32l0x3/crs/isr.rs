#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Reader of field `FECAP`"]
pub type FECAP_R = crate::R<u16, u16>;
#[doc = "Reader of field `FEDIR`"]
pub type FEDIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRIMOVF`"]
pub type TRIMOVF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYNCMISS`"]
pub type SYNCMISS_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYNCERR`"]
pub type SYNCERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `ESYNCF`"]
pub type ESYNCF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRF`"]
pub type ERRF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYNCWARNF`"]
pub type SYNCWARNF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYNCOKF`"]
pub type SYNCOKF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 16:31 - Frequency error capture"]
    #[inline(always)]
    pub fn fecap(&self) -> FECAP_R {
        FECAP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 15 - Frequency error direction"]
    #[inline(always)]
    pub fn fedir(&self) -> FEDIR_R {
        FEDIR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Trimming overflow or underflow"]
    #[inline(always)]
    pub fn trimovf(&self) -> TRIMOVF_R {
        TRIMOVF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SYNC missed"]
    #[inline(always)]
    pub fn syncmiss(&self) -> SYNCMISS_R {
        SYNCMISS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SYNC error"]
    #[inline(always)]
    pub fn syncerr(&self) -> SYNCERR_R {
        SYNCERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Expected SYNC flag"]
    #[inline(always)]
    pub fn esyncf(&self) -> ESYNCF_R {
        ESYNCF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Error flag"]
    #[inline(always)]
    pub fn errf(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SYNC warning flag"]
    #[inline(always)]
    pub fn syncwarnf(&self) -> SYNCWARNF_R {
        SYNCWARNF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SYNC event OK flag"]
    #[inline(always)]
    pub fn syncokf(&self) -> SYNCOKF_R {
        SYNCOKF_R::new((self.bits & 0x01) != 0)
    }
}
