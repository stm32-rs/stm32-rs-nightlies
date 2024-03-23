#[doc = "Register `DDRCTRL_DBGSTAT` reader"]
pub type R = crate::R<DDRCTRL_DBGSTATrs>;
#[doc = "Field `RANK0_REFRESH_BUSY` reader - RANK0_REFRESH_BUSY"]
pub type RANK0_REFRESH_BUSY_R = crate::BitReader;
#[doc = "Field `ZQ_CALIB_SHORT_BUSY` reader - ZQ_CALIB_SHORT_BUSY"]
pub type ZQ_CALIB_SHORT_BUSY_R = crate::BitReader;
#[doc = "Field `CTRLUPD_BUSY` reader - CTRLUPD_BUSY"]
pub type CTRLUPD_BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RANK0_REFRESH_BUSY"]
    #[inline(always)]
    pub fn rank0_refresh_busy(&self) -> RANK0_REFRESH_BUSY_R {
        RANK0_REFRESH_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - ZQ_CALIB_SHORT_BUSY"]
    #[inline(always)]
    pub fn zq_calib_short_busy(&self) -> ZQ_CALIB_SHORT_BUSY_R {
        ZQ_CALIB_SHORT_BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CTRLUPD_BUSY"]
    #[inline(always)]
    pub fn ctrlupd_busy(&self) -> CTRLUPD_BUSY_R {
        CTRLUPD_BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "DDRCTRL status debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dbgstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DBGSTATrs;
impl crate::RegisterSpec for DDRCTRL_DBGSTATrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_dbgstat::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DBGSTATrs {}
#[doc = "`reset()` method sets DDRCTRL_DBGSTAT to value 0"]
impl crate::Resettable for DDRCTRL_DBGSTATrs {
    const RESET_VALUE: u32 = 0;
}
