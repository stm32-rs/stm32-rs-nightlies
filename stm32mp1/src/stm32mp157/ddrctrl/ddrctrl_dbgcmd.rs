#[doc = "Register `DDRCTRL_DBGCMD` reader"]
pub type R = crate::R<DDRCTRL_DBGCMDrs>;
#[doc = "Register `DDRCTRL_DBGCMD` writer"]
pub type W = crate::W<DDRCTRL_DBGCMDrs>;
#[doc = "Field `RANK0_REFRESH` reader - RANK0_REFRESH"]
pub type RANK0_REFRESH_R = crate::BitReader;
#[doc = "Field `RANK0_REFRESH` writer - RANK0_REFRESH"]
pub type RANK0_REFRESH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZQ_CALIB_SHORT` reader - ZQ_CALIB_SHORT"]
pub type ZQ_CALIB_SHORT_R = crate::BitReader;
#[doc = "Field `ZQ_CALIB_SHORT` writer - ZQ_CALIB_SHORT"]
pub type ZQ_CALIB_SHORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRLUPD` reader - CTRLUPD"]
pub type CTRLUPD_R = crate::BitReader;
#[doc = "Field `CTRLUPD` writer - CTRLUPD"]
pub type CTRLUPD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RANK0_REFRESH"]
    #[inline(always)]
    pub fn rank0_refresh(&self) -> RANK0_REFRESH_R {
        RANK0_REFRESH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - ZQ_CALIB_SHORT"]
    #[inline(always)]
    pub fn zq_calib_short(&self) -> ZQ_CALIB_SHORT_R {
        ZQ_CALIB_SHORT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CTRLUPD"]
    #[inline(always)]
    pub fn ctrlupd(&self) -> CTRLUPD_R {
        CTRLUPD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RANK0_REFRESH"]
    #[inline(always)]
    #[must_use]
    pub fn rank0_refresh(&mut self) -> RANK0_REFRESH_W<DDRCTRL_DBGCMDrs> {
        RANK0_REFRESH_W::new(self, 0)
    }
    #[doc = "Bit 4 - ZQ_CALIB_SHORT"]
    #[inline(always)]
    #[must_use]
    pub fn zq_calib_short(&mut self) -> ZQ_CALIB_SHORT_W<DDRCTRL_DBGCMDrs> {
        ZQ_CALIB_SHORT_W::new(self, 4)
    }
    #[doc = "Bit 5 - CTRLUPD"]
    #[inline(always)]
    #[must_use]
    pub fn ctrlupd(&mut self) -> CTRLUPD_W<DDRCTRL_DBGCMDrs> {
        CTRLUPD_W::new(self, 5)
    }
}
#[doc = "DDRCTRL command debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dbgcmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dbgcmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DBGCMDrs;
impl crate::RegisterSpec for DDRCTRL_DBGCMDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_dbgcmd::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DBGCMDrs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_dbgcmd::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_DBGCMDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DBGCMD to value 0"]
impl crate::Resettable for DDRCTRL_DBGCMDrs {
    const RESET_VALUE: u32 = 0;
}
