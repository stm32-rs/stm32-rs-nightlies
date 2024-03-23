#[doc = "Register `BCDR` reader"]
pub type R = crate::R<BCDRrs>;
#[doc = "Register `BCDR` writer"]
pub type W = crate::W<BCDRrs>;
#[doc = "Field `BCDEN` reader - Battery charging detector (BCD) enable"]
pub type BCDEN_R = crate::BitReader;
#[doc = "Field `BCDEN` writer - Battery charging detector (BCD) enable"]
pub type BCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDEN` reader - Data contact detection (DCD) mode enable"]
pub type DCDEN_R = crate::BitReader;
#[doc = "Field `DCDEN` writer - Data contact detection (DCD) mode enable"]
pub type DCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDEN` reader - Primary detection (PD) mode enable"]
pub type PDEN_R = crate::BitReader;
#[doc = "Field `PDEN` writer - Primary detection (PD) mode enable"]
pub type PDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDEN` reader - Secondary detection (SD) mode enable"]
pub type SDEN_R = crate::BitReader;
#[doc = "Field `SDEN` writer - Secondary detection (SD) mode enable"]
pub type SDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDET` reader - Data contact detection (DCD) status"]
pub type DCDET_R = crate::BitReader;
#[doc = "Field `PDET` reader - Primary detection (PD) status"]
pub type PDET_R = crate::BitReader;
#[doc = "Field `SDET` reader - Secondary detection (SD) status"]
pub type SDET_R = crate::BitReader;
#[doc = "Field `PS2DET` reader - DM pull-up detection status"]
pub type PS2DET_R = crate::BitReader;
#[doc = "Field `DPPU` reader - DP pull-up control"]
pub type DPPU_R = crate::BitReader;
#[doc = "Field `DPPU` writer - DP pull-up control"]
pub type DPPU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Battery charging detector (BCD) enable"]
    #[inline(always)]
    pub fn bcden(&self) -> BCDEN_R {
        BCDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data contact detection (DCD) mode enable"]
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Primary detection (PD) mode enable"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Secondary detection (SD) mode enable"]
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data contact detection (DCD) status"]
    #[inline(always)]
    pub fn dcdet(&self) -> DCDET_R {
        DCDET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Primary detection (PD) status"]
    #[inline(always)]
    pub fn pdet(&self) -> PDET_R {
        PDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Secondary detection (SD) status"]
    #[inline(always)]
    pub fn sdet(&self) -> SDET_R {
        SDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DM pull-up detection status"]
    #[inline(always)]
    pub fn ps2det(&self) -> PS2DET_R {
        PS2DET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - DP pull-up control"]
    #[inline(always)]
    pub fn dppu(&self) -> DPPU_R {
        DPPU_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Battery charging detector (BCD) enable"]
    #[inline(always)]
    #[must_use]
    pub fn bcden(&mut self) -> BCDEN_W<BCDRrs> {
        BCDEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data contact detection (DCD) mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcden(&mut self) -> DCDEN_W<BCDRrs> {
        DCDEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Primary detection (PD) mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PDEN_W<BCDRrs> {
        PDEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Secondary detection (SD) mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn sden(&mut self) -> SDEN_W<BCDRrs> {
        SDEN_W::new(self, 3)
    }
    #[doc = "Bit 15 - DP pull-up control"]
    #[inline(always)]
    #[must_use]
    pub fn dppu(&mut self) -> DPPU_W<BCDRrs> {
        DPPU_W::new(self, 15)
    }
}
#[doc = "Battery charging detector(\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCDRrs;
impl crate::RegisterSpec for BCDRrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bcdr::R`](R) reader structure"]
impl crate::Readable for BCDRrs {}
#[doc = "`write(|w| ..)` method takes [`bcdr::W`](W) writer structure"]
impl crate::Writable for BCDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BCDR to value 0"]
impl crate::Resettable for BCDRrs {
    const RESET_VALUE: u16 = 0;
}
