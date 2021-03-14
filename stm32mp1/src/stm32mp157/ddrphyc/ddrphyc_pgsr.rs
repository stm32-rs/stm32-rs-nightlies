#[doc = "Reader of register DDRPHYC_PGSR"]
pub type R = crate::R<u32, super::DDRPHYC_PGSR>;
#[doc = "Reader of field `IDONE`"]
pub type IDONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DLDONE`"]
pub type DLDONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `ZCDDONE`"]
pub type ZCDDONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIDONE`"]
pub type DIDONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTDONE`"]
pub type DTDONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTERR`"]
pub type DTERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTIERR`"]
pub type DTIERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFTERR`"]
pub type DFTERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RVERR`"]
pub type RVERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RVEIRR`"]
pub type RVEIRR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TQ`"]
pub type TQ_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - IDONE"]
    #[inline(always)]
    pub fn idone(&self) -> IDONE_R {
        IDONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DLDONE"]
    #[inline(always)]
    pub fn dldone(&self) -> DLDONE_R {
        DLDONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ZCDDONE"]
    #[inline(always)]
    pub fn zcddone(&self) -> ZCDDONE_R {
        ZCDDONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DIDONE"]
    #[inline(always)]
    pub fn didone(&self) -> DIDONE_R {
        DIDONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DTDONE"]
    #[inline(always)]
    pub fn dtdone(&self) -> DTDONE_R {
        DTDONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DTERR"]
    #[inline(always)]
    pub fn dterr(&self) -> DTERR_R {
        DTERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DTIERR"]
    #[inline(always)]
    pub fn dtierr(&self) -> DTIERR_R {
        DTIERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DFTERR"]
    #[inline(always)]
    pub fn dfterr(&self) -> DFTERR_R {
        DFTERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RVERR"]
    #[inline(always)]
    pub fn rverr(&self) -> RVERR_R {
        RVERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RVEIRR"]
    #[inline(always)]
    pub fn rveirr(&self) -> RVEIRR_R {
        RVEIRR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 31 - TQ"]
    #[inline(always)]
    pub fn tq(&self) -> TQ_R {
        TQ_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
