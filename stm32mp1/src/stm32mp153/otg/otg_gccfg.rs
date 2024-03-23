#[doc = "Register `OTG_GCCFG` reader"]
pub type R = crate::R<OTG_GCCFGrs>;
#[doc = "Register `OTG_GCCFG` writer"]
pub type W = crate::W<OTG_GCCFGrs>;
#[doc = "Field `PDET` reader - PDET"]
pub type PDET_R = crate::BitReader;
#[doc = "Field `SDET` reader - SDET"]
pub type SDET_R = crate::BitReader;
#[doc = "Field `PS2DET` reader - PS2DET"]
pub type PS2DET_R = crate::BitReader;
#[doc = "Field `PWRDWN` reader - PWRDWN"]
pub type PWRDWN_R = crate::BitReader;
#[doc = "Field `PWRDWN` writer - PWRDWN"]
pub type PWRDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCDEN` reader - BCDEN"]
pub type BCDEN_R = crate::BitReader;
#[doc = "Field `BCDEN` writer - BCDEN"]
pub type BCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDEN` reader - PDEN"]
pub type PDEN_R = crate::BitReader;
#[doc = "Field `PDEN` writer - PDEN"]
pub type PDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDEN` reader - SDEN"]
pub type SDEN_R = crate::BitReader;
#[doc = "Field `SDEN` writer - SDEN"]
pub type SDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBDEN` reader - VBDEN"]
pub type VBDEN_R = crate::BitReader;
#[doc = "Field `VBDEN` writer - VBDEN"]
pub type VBDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDEN` reader - IDEN"]
pub type IDEN_R = crate::BitReader;
#[doc = "Field `IDEN` writer - IDEN"]
pub type IDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - PDET"]
    #[inline(always)]
    pub fn pdet(&self) -> PDET_R {
        PDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDET"]
    #[inline(always)]
    pub fn sdet(&self) -> SDET_R {
        SDET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PS2DET"]
    #[inline(always)]
    pub fn ps2det(&self) -> PS2DET_R {
        PS2DET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - PWRDWN"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BCDEN"]
    #[inline(always)]
    pub fn bcden(&self) -> BCDEN_R {
        BCDEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - PDEN"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SDEN"]
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - VBDEN"]
    #[inline(always)]
    pub fn vbden(&self) -> VBDEN_R {
        VBDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - IDEN"]
    #[inline(always)]
    pub fn iden(&self) -> IDEN_R {
        IDEN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - PWRDWN"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<OTG_GCCFGrs> {
        PWRDWN_W::new(self, 16)
    }
    #[doc = "Bit 17 - BCDEN"]
    #[inline(always)]
    #[must_use]
    pub fn bcden(&mut self) -> BCDEN_W<OTG_GCCFGrs> {
        BCDEN_W::new(self, 17)
    }
    #[doc = "Bit 19 - PDEN"]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PDEN_W<OTG_GCCFGrs> {
        PDEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - SDEN"]
    #[inline(always)]
    #[must_use]
    pub fn sden(&mut self) -> SDEN_W<OTG_GCCFGrs> {
        SDEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - VBDEN"]
    #[inline(always)]
    #[must_use]
    pub fn vbden(&mut self) -> VBDEN_W<OTG_GCCFGrs> {
        VBDEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - IDEN"]
    #[inline(always)]
    #[must_use]
    pub fn iden(&mut self) -> IDEN_W<OTG_GCCFGrs> {
        IDEN_W::new(self, 22)
    }
}
#[doc = "OTG general core configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_gccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_gccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_GCCFGrs;
impl crate::RegisterSpec for OTG_GCCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_gccfg::R`](R) reader structure"]
impl crate::Readable for OTG_GCCFGrs {}
#[doc = "`write(|w| ..)` method takes [`otg_gccfg::W`](W) writer structure"]
impl crate::Writable for OTG_GCCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_GCCFG to value 0"]
impl crate::Resettable for OTG_GCCFGrs {
    const RESET_VALUE: u32 = 0;
}
