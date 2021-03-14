#[doc = "Reader of register DDRPHYC_ZQ0SR0"]
pub type R = crate::R<u32, super::DDRPHYC_ZQ0SR0>;
#[doc = "Reader of field `ZCTRL`"]
pub type ZCTRL_R = crate::R<u32, u32>;
#[doc = "Reader of field `ZERR`"]
pub type ZERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `ZDONE`"]
pub type ZDONE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:19 - ZCTRL"]
    #[inline(always)]
    pub fn zctrl(&self) -> ZCTRL_R {
        ZCTRL_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 30 - ZERR"]
    #[inline(always)]
    pub fn zerr(&self) -> ZERR_R {
        ZERR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ZDONE"]
    #[inline(always)]
    pub fn zdone(&self) -> ZDONE_R {
        ZDONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
