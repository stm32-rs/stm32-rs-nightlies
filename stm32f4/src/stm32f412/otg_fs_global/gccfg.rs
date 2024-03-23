#[doc = "Register `GCCFG` reader"]
pub type R = crate::R<GCCFGrs>;
#[doc = "Register `GCCFG` writer"]
pub type W = crate::W<GCCFGrs>;
#[doc = "Field `DCDET` reader - DCDET"]
pub type DCDET_R = crate::BitReader;
#[doc = "Field `DCDET` writer - DCDET"]
pub type DCDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDET` reader - PDET"]
pub type PDET_R = crate::BitReader;
#[doc = "Field `PDET` writer - PDET"]
pub type PDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDET` reader - SDET"]
pub type SDET_R = crate::BitReader;
#[doc = "Field `SDET` writer - SDET"]
pub type SDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2DET` reader - PS2DET"]
pub type PS2DET_R = crate::BitReader;
#[doc = "Field `PS2DET` writer - PS2DET"]
pub type PS2DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRDWN` reader - PWRDWN"]
pub type PWRDWN_R = crate::BitReader;
#[doc = "Field `PWRDWN` writer - PWRDWN"]
pub type PWRDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCDEN` reader - BCDEN"]
pub type BCDEN_R = crate::BitReader;
#[doc = "Field `BCDEN` writer - BCDEN"]
pub type BCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDEN` reader - DCDEN"]
pub type DCDEN_R = crate::BitReader;
#[doc = "Field `DCDEN` writer - DCDEN"]
pub type DCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl R {
    #[doc = "Bit 0 - DCDET"]
    #[inline(always)]
    pub fn dcdet(&self) -> DCDET_R {
        DCDET_R::new((self.bits & 1) != 0)
    }
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
    #[doc = "Bit 18 - DCDEN"]
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 18) & 1) != 0)
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
}
impl W {
    #[doc = "Bit 0 - DCDET"]
    #[inline(always)]
    #[must_use]
    pub fn dcdet(&mut self) -> DCDET_W<GCCFGrs> {
        DCDET_W::new(self, 0)
    }
    #[doc = "Bit 1 - PDET"]
    #[inline(always)]
    #[must_use]
    pub fn pdet(&mut self) -> PDET_W<GCCFGrs> {
        PDET_W::new(self, 1)
    }
    #[doc = "Bit 2 - SDET"]
    #[inline(always)]
    #[must_use]
    pub fn sdet(&mut self) -> SDET_W<GCCFGrs> {
        SDET_W::new(self, 2)
    }
    #[doc = "Bit 3 - PS2DET"]
    #[inline(always)]
    #[must_use]
    pub fn ps2det(&mut self) -> PS2DET_W<GCCFGrs> {
        PS2DET_W::new(self, 3)
    }
    #[doc = "Bit 16 - PWRDWN"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<GCCFGrs> {
        PWRDWN_W::new(self, 16)
    }
    #[doc = "Bit 17 - BCDEN"]
    #[inline(always)]
    #[must_use]
    pub fn bcden(&mut self) -> BCDEN_W<GCCFGrs> {
        BCDEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - DCDEN"]
    #[inline(always)]
    #[must_use]
    pub fn dcden(&mut self) -> DCDEN_W<GCCFGrs> {
        DCDEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - PDEN"]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PDEN_W<GCCFGrs> {
        PDEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - SDEN"]
    #[inline(always)]
    #[must_use]
    pub fn sden(&mut self) -> SDEN_W<GCCFGrs> {
        SDEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - VBDEN"]
    #[inline(always)]
    #[must_use]
    pub fn vbden(&mut self) -> VBDEN_W<GCCFGrs> {
        VBDEN_W::new(self, 21)
    }
}
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCCFGrs;
impl crate::RegisterSpec for GCCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gccfg::R`](R) reader structure"]
impl crate::Readable for GCCFGrs {}
#[doc = "`write(|w| ..)` method takes [`gccfg::W`](W) writer structure"]
impl crate::Writable for GCCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCCFG to value 0"]
impl crate::Resettable for GCCFGrs {
    const RESET_VALUE: u32 = 0;
}
