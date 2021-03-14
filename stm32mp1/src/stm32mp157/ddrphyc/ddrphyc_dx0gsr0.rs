#[doc = "Reader of register DDRPHYC_DX0GSR0"]
pub type R = crate::R<u16, super::DDRPHYC_DX0GSR0>;
#[doc = "Reader of field `DTDONE`"]
pub type DTDONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTERR`"]
pub type DTERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTIERR`"]
pub type DTIERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTPASS`"]
pub type DTPASS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - DTDONE"]
    #[inline(always)]
    pub fn dtdone(&self) -> DTDONE_R {
        DTDONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - DTERR"]
    #[inline(always)]
    pub fn dterr(&self) -> DTERR_R {
        DTERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DTIERR"]
    #[inline(always)]
    pub fn dtierr(&self) -> DTIERR_R {
        DTIERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - DTPASS"]
    #[inline(always)]
    pub fn dtpass(&self) -> DTPASS_R {
        DTPASS_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
