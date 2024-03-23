#[doc = "Register `ERCFGR` reader"]
pub type R = crate::R<ERCFGRrs>;
#[doc = "Register `ERCFGR` writer"]
pub type W = crate::W<ERCFGRrs>;
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
    pub fn ercfg0(&mut self) -> ERCFG0_W<ERCFGRrs> {
        ERCFG0_W::new(self, 0)
    }
}
#[doc = "TAMP erase configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ercfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ercfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERCFGRrs;
impl crate::RegisterSpec for ERCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ercfgr::R`](R) reader structure"]
impl crate::Readable for ERCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`ercfgr::W`](W) writer structure"]
impl crate::Writable for ERCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERCFGR to value 0"]
impl crate::Resettable for ERCFGRrs {
    const RESET_VALUE: u32 = 0;
}
