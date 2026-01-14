///Register `PGSR` reader
pub type R = crate::R<PGSRrs>;
///Field `IDONE` reader - IDONE
pub type IDONE_R = crate::BitReader;
///Field `DLDONE` reader - DLDONE
pub type DLDONE_R = crate::BitReader;
///Field `ZCDDONE` reader - ZCDDONE
pub type ZCDDONE_R = crate::BitReader;
///Field `DIDONE` reader - DIDONE
pub type DIDONE_R = crate::BitReader;
///Field `DTDONE` reader - DTDONE
pub type DTDONE_R = crate::BitReader;
///Field `DTERR` reader - DTERR
pub type DTERR_R = crate::BitReader;
///Field `DTIERR` reader - DTIERR
pub type DTIERR_R = crate::BitReader;
///Field `DFTERR` reader - DFTERR
pub type DFTERR_R = crate::BitReader;
///Field `RVERR` reader - RVERR
pub type RVERR_R = crate::BitReader;
///Field `RVEIRR` reader - RVEIRR
pub type RVEIRR_R = crate::BitReader;
///Field `TQ` reader - TQ
pub type TQ_R = crate::BitReader;
impl R {
    ///Bit 0 - IDONE
    #[inline(always)]
    pub fn idone(&self) -> IDONE_R {
        IDONE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DLDONE
    #[inline(always)]
    pub fn dldone(&self) -> DLDONE_R {
        DLDONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ZCDDONE
    #[inline(always)]
    pub fn zcddone(&self) -> ZCDDONE_R {
        ZCDDONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DIDONE
    #[inline(always)]
    pub fn didone(&self) -> DIDONE_R {
        DIDONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DTDONE
    #[inline(always)]
    pub fn dtdone(&self) -> DTDONE_R {
        DTDONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DTERR
    #[inline(always)]
    pub fn dterr(&self) -> DTERR_R {
        DTERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DTIERR
    #[inline(always)]
    pub fn dtierr(&self) -> DTIERR_R {
        DTIERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DFTERR
    #[inline(always)]
    pub fn dfterr(&self) -> DFTERR_R {
        DFTERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RVERR
    #[inline(always)]
    pub fn rverr(&self) -> RVERR_R {
        RVERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - RVEIRR
    #[inline(always)]
    pub fn rveirr(&self) -> RVEIRR_R {
        RVEIRR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 31 - TQ
    #[inline(always)]
    pub fn tq(&self) -> TQ_R {
        TQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PGSR")
            .field("idone", &self.idone())
            .field("dldone", &self.dldone())
            .field("zcddone", &self.zcddone())
            .field("didone", &self.didone())
            .field("dtdone", &self.dtdone())
            .field("dterr", &self.dterr())
            .field("dtierr", &self.dtierr())
            .field("dfterr", &self.dfterr())
            .field("rverr", &self.rverr())
            .field("rveirr", &self.rveirr())
            .field("tq", &self.tq())
            .finish()
    }
}
/**DDRPHYC PHY global status register

You can [`read`](crate::Reg::read) this register and get [`pgsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPHYC:PGSR)*/
pub struct PGSRrs;
impl crate::RegisterSpec for PGSRrs {
    type Ux = u32;
}
///`read()` method returns [`pgsr::R`](R) reader structure
impl crate::Readable for PGSRrs {}
///`reset()` method sets PGSR to value 0
impl crate::Resettable for PGSRrs {}
