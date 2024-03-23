#[doc = "Register `DDRPHYC_PGSR` reader"]
pub type R = crate::R<DDRPHYC_PGSRrs>;
#[doc = "Field `IDONE` reader - IDONE"]
pub type IDONE_R = crate::BitReader;
#[doc = "Field `DLDONE` reader - DLDONE"]
pub type DLDONE_R = crate::BitReader;
#[doc = "Field `ZCDDONE` reader - ZCDDONE"]
pub type ZCDDONE_R = crate::BitReader;
#[doc = "Field `DIDONE` reader - DIDONE"]
pub type DIDONE_R = crate::BitReader;
#[doc = "Field `DTDONE` reader - DTDONE"]
pub type DTDONE_R = crate::BitReader;
#[doc = "Field `DTERR` reader - DTERR"]
pub type DTERR_R = crate::BitReader;
#[doc = "Field `DTIERR` reader - DTIERR"]
pub type DTIERR_R = crate::BitReader;
#[doc = "Field `DFTERR` reader - DFTERR"]
pub type DFTERR_R = crate::BitReader;
#[doc = "Field `RVERR` reader - RVERR"]
pub type RVERR_R = crate::BitReader;
#[doc = "Field `RVEIRR` reader - RVEIRR"]
pub type RVEIRR_R = crate::BitReader;
#[doc = "Field `TQ` reader - TQ"]
pub type TQ_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - IDONE"]
    #[inline(always)]
    pub fn idone(&self) -> IDONE_R {
        IDONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DLDONE"]
    #[inline(always)]
    pub fn dldone(&self) -> DLDONE_R {
        DLDONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ZCDDONE"]
    #[inline(always)]
    pub fn zcddone(&self) -> ZCDDONE_R {
        ZCDDONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DIDONE"]
    #[inline(always)]
    pub fn didone(&self) -> DIDONE_R {
        DIDONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DTDONE"]
    #[inline(always)]
    pub fn dtdone(&self) -> DTDONE_R {
        DTDONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DTERR"]
    #[inline(always)]
    pub fn dterr(&self) -> DTERR_R {
        DTERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DTIERR"]
    #[inline(always)]
    pub fn dtierr(&self) -> DTIERR_R {
        DTIERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DFTERR"]
    #[inline(always)]
    pub fn dfterr(&self) -> DFTERR_R {
        DFTERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RVERR"]
    #[inline(always)]
    pub fn rverr(&self) -> RVERR_R {
        RVERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RVEIRR"]
    #[inline(always)]
    pub fn rveirr(&self) -> RVEIRR_R {
        RVEIRR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - TQ"]
    #[inline(always)]
    pub fn tq(&self) -> TQ_R {
        TQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "DDRPHYC PHY global status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_pgsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_PGSRrs;
impl crate::RegisterSpec for DDRPHYC_PGSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_pgsr::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_PGSRrs {}
#[doc = "`reset()` method sets DDRPHYC_PGSR to value 0"]
impl crate::Resettable for DDRPHYC_PGSRrs {
    const RESET_VALUE: u32 = 0;
}
