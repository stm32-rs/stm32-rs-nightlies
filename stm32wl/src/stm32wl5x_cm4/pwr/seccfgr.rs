#[doc = "Register `SECCFGR` reader"]
pub type R = crate::R<SECCFGRrs>;
#[doc = "Register `SECCFGR` writer"]
pub type W = crate::W<SECCFGRrs>;
#[doc = "Field `C2EWILA` reader - wakeup on CPU2 illegal access interrupt enable"]
pub type C2EWILA_R = crate::BitReader;
#[doc = "Field `C2EWILA` writer - wakeup on CPU2 illegal access interrupt enable"]
pub type C2EWILA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 15 - wakeup on CPU2 illegal access interrupt enable"]
    #[inline(always)]
    pub fn c2ewila(&self) -> C2EWILA_R {
        C2EWILA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - wakeup on CPU2 illegal access interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2ewila(&mut self) -> C2EWILA_W<SECCFGRrs> {
        C2EWILA_W::new(self, 15)
    }
}
#[doc = "Power security configuration register \\[dual core device only\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seccfgr::R`](R) reader structure"]
impl crate::Readable for SECCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure"]
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECCFGR to value 0x8000"]
impl crate::Resettable for SECCFGRrs {
    const RESET_VALUE: u32 = 0x8000;
}
