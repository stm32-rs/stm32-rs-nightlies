#[doc = "Register `TAMP_ERCFGR` reader"]
pub type R = crate::R<TAMP_ERCFGRrs>;
#[doc = "Register `TAMP_ERCFGR` writer"]
pub type W = crate::W<TAMP_ERCFGRrs>;
#[doc = "Field `ERCFG0` reader - Configurable device secrets configuration"]
pub type ERCFG0_R = crate::BitReader;
#[doc = "Field `ERCFG0` writer - Configurable device secrets configuration"]
pub type ERCFG0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configurable device secrets configuration"]
    #[inline(always)]
    pub fn ercfg0(&self) -> ERCFG0_R {
        ERCFG0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configurable device secrets configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ercfg0(&mut self) -> ERCFG0_W<TAMP_ERCFGRrs> {
        ERCFG0_W::new(self, 0)
    }
}
#[doc = "TAMP erase configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_ercfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_ercfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAMP_ERCFGRrs;
impl crate::RegisterSpec for TAMP_ERCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp_ercfgr::R`](R) reader structure"]
impl crate::Readable for TAMP_ERCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`tamp_ercfgr::W`](W) writer structure"]
impl crate::Writable for TAMP_ERCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMP_ERCFGR to value 0"]
impl crate::Resettable for TAMP_ERCFGRrs {
    const RESET_VALUE: u32 = 0;
}
