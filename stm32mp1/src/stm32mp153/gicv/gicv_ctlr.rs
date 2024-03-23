#[doc = "Register `GICV_CTLR` reader"]
pub type R = crate::R<GICV_CTLRrs>;
#[doc = "Register `GICV_CTLR` writer"]
pub type W = crate::W<GICV_CTLRrs>;
#[doc = "Field `ENABLEGRP0` reader - ENABLEGRP0"]
pub type ENABLEGRP0_R = crate::BitReader;
#[doc = "Field `ENABLEGRP0` writer - ENABLEGRP0"]
pub type ENABLEGRP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLEGRP1` reader - ENABLEGRP1"]
pub type ENABLEGRP1_R = crate::BitReader;
#[doc = "Field `ENABLEGRP1` writer - ENABLEGRP1"]
pub type ENABLEGRP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKCTL` reader - ACKCTL"]
pub type ACKCTL_R = crate::BitReader;
#[doc = "Field `ACKCTL` writer - ACKCTL"]
pub type ACKCTL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIQEN` reader - FIQEN"]
pub type FIQEN_R = crate::BitReader;
#[doc = "Field `FIQEN` writer - FIQEN"]
pub type FIQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBPR` reader - CBPR"]
pub type CBPR_R = crate::BitReader;
#[doc = "Field `CBPR` writer - CBPR"]
pub type CBPR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOIMODE` reader - EOIMODE"]
pub type EOIMODE_R = crate::BitReader;
#[doc = "Field `EOIMODE` writer - EOIMODE"]
pub type EOIMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ENABLEGRP0"]
    #[inline(always)]
    pub fn enablegrp0(&self) -> ENABLEGRP0_R {
        ENABLEGRP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ENABLEGRP1"]
    #[inline(always)]
    pub fn enablegrp1(&self) -> ENABLEGRP1_R {
        ENABLEGRP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ACKCTL"]
    #[inline(always)]
    pub fn ackctl(&self) -> ACKCTL_R {
        ACKCTL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIQEN"]
    #[inline(always)]
    pub fn fiqen(&self) -> FIQEN_R {
        FIQEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CBPR"]
    #[inline(always)]
    pub fn cbpr(&self) -> CBPR_R {
        CBPR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - EOIMODE"]
    #[inline(always)]
    pub fn eoimode(&self) -> EOIMODE_R {
        EOIMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ENABLEGRP0"]
    #[inline(always)]
    #[must_use]
    pub fn enablegrp0(&mut self) -> ENABLEGRP0_W<GICV_CTLRrs> {
        ENABLEGRP0_W::new(self, 0)
    }
    #[doc = "Bit 1 - ENABLEGRP1"]
    #[inline(always)]
    #[must_use]
    pub fn enablegrp1(&mut self) -> ENABLEGRP1_W<GICV_CTLRrs> {
        ENABLEGRP1_W::new(self, 1)
    }
    #[doc = "Bit 2 - ACKCTL"]
    #[inline(always)]
    #[must_use]
    pub fn ackctl(&mut self) -> ACKCTL_W<GICV_CTLRrs> {
        ACKCTL_W::new(self, 2)
    }
    #[doc = "Bit 3 - FIQEN"]
    #[inline(always)]
    #[must_use]
    pub fn fiqen(&mut self) -> FIQEN_W<GICV_CTLRrs> {
        FIQEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - CBPR"]
    #[inline(always)]
    #[must_use]
    pub fn cbpr(&mut self) -> CBPR_W<GICV_CTLRrs> {
        CBPR_W::new(self, 4)
    }
    #[doc = "Bit 9 - EOIMODE"]
    #[inline(always)]
    #[must_use]
    pub fn eoimode(&mut self) -> EOIMODE_W<GICV_CTLRrs> {
        EOIMODE_W::new(self, 9)
    }
}
#[doc = "GICV virtual machine control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_ctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicv_ctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICV_CTLRrs;
impl crate::RegisterSpec for GICV_CTLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicv_ctlr::R`](R) reader structure"]
impl crate::Readable for GICV_CTLRrs {}
#[doc = "`write(|w| ..)` method takes [`gicv_ctlr::W`](W) writer structure"]
impl crate::Writable for GICV_CTLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICV_CTLR to value 0"]
impl crate::Resettable for GICV_CTLRrs {
    const RESET_VALUE: u32 = 0;
}
