///Register `DBGCMD` reader
pub type R = crate::R<DBGCMDrs>;
///Register `DBGCMD` writer
pub type W = crate::W<DBGCMDrs>;
///Field `RANK0_REFRESH` reader - RANK0_REFRESH
pub type RANK0_REFRESH_R = crate::BitReader;
///Field `RANK0_REFRESH` writer - RANK0_REFRESH
pub type RANK0_REFRESH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ZQ_CALIB_SHORT` reader - ZQ_CALIB_SHORT
pub type ZQ_CALIB_SHORT_R = crate::BitReader;
///Field `ZQ_CALIB_SHORT` writer - ZQ_CALIB_SHORT
pub type ZQ_CALIB_SHORT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTRLUPD` reader - CTRLUPD
pub type CTRLUPD_R = crate::BitReader;
///Field `CTRLUPD` writer - CTRLUPD
pub type CTRLUPD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RANK0_REFRESH
    #[inline(always)]
    pub fn rank0_refresh(&self) -> RANK0_REFRESH_R {
        RANK0_REFRESH_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - ZQ_CALIB_SHORT
    #[inline(always)]
    pub fn zq_calib_short(&self) -> ZQ_CALIB_SHORT_R {
        ZQ_CALIB_SHORT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CTRLUPD
    #[inline(always)]
    pub fn ctrlupd(&self) -> CTRLUPD_R {
        CTRLUPD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGCMD")
            .field("rank0_refresh", &self.rank0_refresh())
            .field("zq_calib_short", &self.zq_calib_short())
            .field("ctrlupd", &self.ctrlupd())
            .finish()
    }
}
impl W {
    ///Bit 0 - RANK0_REFRESH
    #[inline(always)]
    pub fn rank0_refresh(&mut self) -> RANK0_REFRESH_W<'_, DBGCMDrs> {
        RANK0_REFRESH_W::new(self, 0)
    }
    ///Bit 4 - ZQ_CALIB_SHORT
    #[inline(always)]
    pub fn zq_calib_short(&mut self) -> ZQ_CALIB_SHORT_W<'_, DBGCMDrs> {
        ZQ_CALIB_SHORT_W::new(self, 4)
    }
    ///Bit 5 - CTRLUPD
    #[inline(always)]
    pub fn ctrlupd(&mut self) -> CTRLUPD_W<'_, DBGCMDrs> {
        CTRLUPD_W::new(self, 5)
    }
}
/**DDRCTRL command debug register

You can [`read`](crate::Reg::read) this register and get [`dbgcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DBGCMD)*/
pub struct DBGCMDrs;
impl crate::RegisterSpec for DBGCMDrs {
    type Ux = u32;
}
///`read()` method returns [`dbgcmd::R`](R) reader structure
impl crate::Readable for DBGCMDrs {}
///`write(|w| ..)` method takes [`dbgcmd::W`](W) writer structure
impl crate::Writable for DBGCMDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBGCMD to value 0
impl crate::Resettable for DBGCMDrs {}
