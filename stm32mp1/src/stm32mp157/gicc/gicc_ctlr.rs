#[doc = "Register `GICC_CTLR` reader"]
pub type R = crate::R<GICC_CTLRrs>;
#[doc = "Register `GICC_CTLR` writer"]
pub type W = crate::W<GICC_CTLRrs>;
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
#[doc = "Field `FIQBYPDISGRP0` reader - FIQBYPDISGRP0"]
pub type FIQBYPDISGRP0_R = crate::BitReader;
#[doc = "Field `FIQBYPDISGRP0` writer - FIQBYPDISGRP0"]
pub type FIQBYPDISGRP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQBYPDISGRP0` reader - IRQBYPDISGRP0"]
pub type IRQBYPDISGRP0_R = crate::BitReader;
#[doc = "Field `IRQBYPDISGRP0` writer - IRQBYPDISGRP0"]
pub type IRQBYPDISGRP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIQBYPDISGRP1` reader - FIQBYPDISGRP1"]
pub type FIQBYPDISGRP1_R = crate::BitReader;
#[doc = "Field `FIQBYPDISGRP1` writer - FIQBYPDISGRP1"]
pub type FIQBYPDISGRP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQBYPDISGRP1` reader - IRQBYPDISGRP1"]
pub type IRQBYPDISGRP1_R = crate::BitReader;
#[doc = "Field `IRQBYPDISGRP1` writer - IRQBYPDISGRP1"]
pub type IRQBYPDISGRP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOIMODES` reader - EOIMODES"]
pub type EOIMODES_R = crate::BitReader;
#[doc = "Field `EOIMODES` writer - EOIMODES"]
pub type EOIMODES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOIMODENS` reader - EOIMODENS"]
pub type EOIMODENS_R = crate::BitReader;
#[doc = "Field `EOIMODENS` writer - EOIMODENS"]
pub type EOIMODENS_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 5 - FIQBYPDISGRP0"]
    #[inline(always)]
    pub fn fiqbypdisgrp0(&self) -> FIQBYPDISGRP0_R {
        FIQBYPDISGRP0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRQBYPDISGRP0"]
    #[inline(always)]
    pub fn irqbypdisgrp0(&self) -> IRQBYPDISGRP0_R {
        IRQBYPDISGRP0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FIQBYPDISGRP1"]
    #[inline(always)]
    pub fn fiqbypdisgrp1(&self) -> FIQBYPDISGRP1_R {
        FIQBYPDISGRP1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IRQBYPDISGRP1"]
    #[inline(always)]
    pub fn irqbypdisgrp1(&self) -> IRQBYPDISGRP1_R {
        IRQBYPDISGRP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EOIMODES"]
    #[inline(always)]
    pub fn eoimodes(&self) -> EOIMODES_R {
        EOIMODES_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EOIMODENS"]
    #[inline(always)]
    pub fn eoimodens(&self) -> EOIMODENS_R {
        EOIMODENS_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ENABLEGRP0"]
    #[inline(always)]
    #[must_use]
    pub fn enablegrp0(&mut self) -> ENABLEGRP0_W<GICC_CTLRrs> {
        ENABLEGRP0_W::new(self, 0)
    }
    #[doc = "Bit 1 - ENABLEGRP1"]
    #[inline(always)]
    #[must_use]
    pub fn enablegrp1(&mut self) -> ENABLEGRP1_W<GICC_CTLRrs> {
        ENABLEGRP1_W::new(self, 1)
    }
    #[doc = "Bit 2 - ACKCTL"]
    #[inline(always)]
    #[must_use]
    pub fn ackctl(&mut self) -> ACKCTL_W<GICC_CTLRrs> {
        ACKCTL_W::new(self, 2)
    }
    #[doc = "Bit 3 - FIQEN"]
    #[inline(always)]
    #[must_use]
    pub fn fiqen(&mut self) -> FIQEN_W<GICC_CTLRrs> {
        FIQEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - CBPR"]
    #[inline(always)]
    #[must_use]
    pub fn cbpr(&mut self) -> CBPR_W<GICC_CTLRrs> {
        CBPR_W::new(self, 4)
    }
    #[doc = "Bit 5 - FIQBYPDISGRP0"]
    #[inline(always)]
    #[must_use]
    pub fn fiqbypdisgrp0(&mut self) -> FIQBYPDISGRP0_W<GICC_CTLRrs> {
        FIQBYPDISGRP0_W::new(self, 5)
    }
    #[doc = "Bit 6 - IRQBYPDISGRP0"]
    #[inline(always)]
    #[must_use]
    pub fn irqbypdisgrp0(&mut self) -> IRQBYPDISGRP0_W<GICC_CTLRrs> {
        IRQBYPDISGRP0_W::new(self, 6)
    }
    #[doc = "Bit 7 - FIQBYPDISGRP1"]
    #[inline(always)]
    #[must_use]
    pub fn fiqbypdisgrp1(&mut self) -> FIQBYPDISGRP1_W<GICC_CTLRrs> {
        FIQBYPDISGRP1_W::new(self, 7)
    }
    #[doc = "Bit 8 - IRQBYPDISGRP1"]
    #[inline(always)]
    #[must_use]
    pub fn irqbypdisgrp1(&mut self) -> IRQBYPDISGRP1_W<GICC_CTLRrs> {
        IRQBYPDISGRP1_W::new(self, 8)
    }
    #[doc = "Bit 9 - EOIMODES"]
    #[inline(always)]
    #[must_use]
    pub fn eoimodes(&mut self) -> EOIMODES_W<GICC_CTLRrs> {
        EOIMODES_W::new(self, 9)
    }
    #[doc = "Bit 10 - EOIMODENS"]
    #[inline(always)]
    #[must_use]
    pub fn eoimodens(&mut self) -> EOIMODENS_W<GICC_CTLRrs> {
        EOIMODENS_W::new(self, 10)
    }
}
#[doc = "GICC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_ctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_ctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICC_CTLRrs;
impl crate::RegisterSpec for GICC_CTLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_ctlr::R`](R) reader structure"]
impl crate::Readable for GICC_CTLRrs {}
#[doc = "`write(|w| ..)` method takes [`gicc_ctlr::W`](W) writer structure"]
impl crate::Writable for GICC_CTLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICC_CTLR to value 0"]
impl crate::Resettable for GICC_CTLRrs {
    const RESET_VALUE: u32 = 0;
}
