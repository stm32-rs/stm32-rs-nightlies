#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `TMONEN` reader - TMONEN"]
pub type TMONEN_R = crate::BitReader;
#[doc = "Field `TMONEN` writer - TMONEN"]
pub type TMONEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONEN` reader - VMONEN"]
pub type VMONEN_R = crate::BitReader;
#[doc = "Field `VMONEN` writer - VMONEN"]
pub type VMONEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTMONEN` reader - WUTMONEN"]
pub type WUTMONEN_R = crate::BitReader;
#[doc = "Field `WUTMONEN` writer - WUTMONEN"]
pub type WUTMONEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - TMONEN"]
    #[inline(always)]
    pub fn tmonen(&self) -> TMONEN_R {
        TMONEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VMONEN"]
    #[inline(always)]
    pub fn vmonen(&self) -> VMONEN_R {
        VMONEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WUTMONEN"]
    #[inline(always)]
    pub fn wutmonen(&self) -> WUTMONEN_R {
        WUTMONEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TMONEN"]
    #[inline(always)]
    #[must_use]
    pub fn tmonen(&mut self) -> TMONEN_W<CFGRrs> {
        TMONEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - VMONEN"]
    #[inline(always)]
    #[must_use]
    pub fn vmonen(&mut self) -> VMONEN_W<CFGRrs> {
        VMONEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - WUTMONEN"]
    #[inline(always)]
    #[must_use]
    pub fn wutmonen(&mut self) -> WUTMONEN_W<CFGRrs> {
        WUTMONEN_W::new(self, 3)
    }
}
#[doc = "TAMP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
