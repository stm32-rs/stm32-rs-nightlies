///Register `DBGSTAT` reader
pub type R = crate::R<DBGSTATrs>;
///Field `RANK0_REFRESH_BUSY` reader - RANK0_REFRESH_BUSY
pub type RANK0_REFRESH_BUSY_R = crate::BitReader;
///Field `ZQ_CALIB_SHORT_BUSY` reader - ZQ_CALIB_SHORT_BUSY
pub type ZQ_CALIB_SHORT_BUSY_R = crate::BitReader;
///Field `CTRLUPD_BUSY` reader - CTRLUPD_BUSY
pub type CTRLUPD_BUSY_R = crate::BitReader;
impl R {
    ///Bit 0 - RANK0_REFRESH_BUSY
    #[inline(always)]
    pub fn rank0_refresh_busy(&self) -> RANK0_REFRESH_BUSY_R {
        RANK0_REFRESH_BUSY_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - ZQ_CALIB_SHORT_BUSY
    #[inline(always)]
    pub fn zq_calib_short_busy(&self) -> ZQ_CALIB_SHORT_BUSY_R {
        ZQ_CALIB_SHORT_BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CTRLUPD_BUSY
    #[inline(always)]
    pub fn ctrlupd_busy(&self) -> CTRLUPD_BUSY_R {
        CTRLUPD_BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGSTAT")
            .field("rank0_refresh_busy", &self.rank0_refresh_busy())
            .field("zq_calib_short_busy", &self.zq_calib_short_busy())
            .field("ctrlupd_busy", &self.ctrlupd_busy())
            .finish()
    }
}
/**DDRCTRL status debug register

You can [`read`](crate::Reg::read) this register and get [`dbgstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DBGSTAT)*/
pub struct DBGSTATrs;
impl crate::RegisterSpec for DBGSTATrs {
    type Ux = u32;
}
///`read()` method returns [`dbgstat::R`](R) reader structure
impl crate::Readable for DBGSTATrs {}
///`reset()` method sets DBGSTAT to value 0
impl crate::Resettable for DBGSTATrs {}
