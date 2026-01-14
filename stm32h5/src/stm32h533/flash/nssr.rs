///Register `NSSR` reader
pub type R = crate::R<NSSRrs>;
///Field `BSY` reader - busy flag
pub type BSY_R = crate::BitReader;
///Field `WBNE` reader - write buffer not empty flag
pub type WBNE_R = crate::BitReader;
///Field `DBNE` reader - data buffer not empty flag
pub type DBNE_R = crate::BitReader;
///Field `EOP` reader - end of operation flag
pub type EOP_R = crate::BitReader;
///Field `WRPERR` reader - write protection error flag
pub type WRPERR_R = crate::BitReader;
///Field `PGSERR` reader - programming sequence error flag
pub type PGSERR_R = crate::BitReader;
///Field `STRBERR` reader - strobe error flag
pub type STRBERR_R = crate::BitReader;
///Field `INCERR` reader - inconsistency error flag
pub type INCERR_R = crate::BitReader;
///Field `OBKERR` reader - OBK general error flag
pub type OBKERR_R = crate::BitReader;
///Field `OBKWERR` reader - OBK write error flag
pub type OBKWERR_R = crate::BitReader;
///Field `OPTCHANGEERR` reader - Option byte change error flag
pub type OPTCHANGEERR_R = crate::BitReader;
impl R {
    ///Bit 0 - busy flag
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - write buffer not empty flag
    #[inline(always)]
    pub fn wbne(&self) -> WBNE_R {
        WBNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - data buffer not empty flag
    #[inline(always)]
    pub fn dbne(&self) -> DBNE_R {
        DBNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 16 - end of operation flag
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - write protection error flag
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - programming sequence error flag
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - strobe error flag
    #[inline(always)]
    pub fn strberr(&self) -> STRBERR_R {
        STRBERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - inconsistency error flag
    #[inline(always)]
    pub fn incerr(&self) -> INCERR_R {
        INCERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - OBK general error flag
    #[inline(always)]
    pub fn obkerr(&self) -> OBKERR_R {
        OBKERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - OBK write error flag
    #[inline(always)]
    pub fn obkwerr(&self) -> OBKWERR_R {
        OBKWERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Option byte change error flag
    #[inline(always)]
    pub fn optchangeerr(&self) -> OPTCHANGEERR_R {
        OPTCHANGEERR_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NSSR")
            .field("bsy", &self.bsy())
            .field("wbne", &self.wbne())
            .field("dbne", &self.dbne())
            .field("eop", &self.eop())
            .field("wrperr", &self.wrperr())
            .field("pgserr", &self.pgserr())
            .field("strberr", &self.strberr())
            .field("incerr", &self.incerr())
            .field("obkerr", &self.obkerr())
            .field("obkwerr", &self.obkwerr())
            .field("optchangeerr", &self.optchangeerr())
            .finish()
    }
}
/**FLASH non-secure status register

You can [`read`](crate::Reg::read) this register and get [`nssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#FLASH:NSSR)*/
pub struct NSSRrs;
impl crate::RegisterSpec for NSSRrs {
    type Ux = u32;
}
///`read()` method returns [`nssr::R`](R) reader structure
impl crate::Readable for NSSRrs {}
///`reset()` method sets NSSR to value 0
impl crate::Resettable for NSSRrs {}
