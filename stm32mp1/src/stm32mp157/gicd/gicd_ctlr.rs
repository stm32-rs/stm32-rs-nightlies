#[doc = "Register `GICD_CTLR` reader"]
pub type R = crate::R<GICD_CTLRrs>;
#[doc = "Register `GICD_CTLR` writer"]
pub type W = crate::W<GICD_CTLRrs>;
#[doc = "Field `ENABLEGRP0` reader - ENABLEGRP0"]
pub type ENABLEGRP0_R = crate::BitReader;
#[doc = "Field `ENABLEGRP0` writer - ENABLEGRP0"]
pub type ENABLEGRP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLEGRP1` reader - ENABLEGRP1"]
pub type ENABLEGRP1_R = crate::BitReader;
#[doc = "Field `ENABLEGRP1` writer - ENABLEGRP1"]
pub type ENABLEGRP1_W<'a, REG> = crate::BitWriter<'a, REG>;
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
}
impl W {
    #[doc = "Bit 0 - ENABLEGRP0"]
    #[inline(always)]
    #[must_use]
    pub fn enablegrp0(&mut self) -> ENABLEGRP0_W<GICD_CTLRrs> {
        ENABLEGRP0_W::new(self, 0)
    }
    #[doc = "Bit 1 - ENABLEGRP1"]
    #[inline(always)]
    #[must_use]
    pub fn enablegrp1(&mut self) -> ENABLEGRP1_W<GICD_CTLRrs> {
        ENABLEGRP1_W::new(self, 1)
    }
}
#[doc = "GICD control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_CTLRrs;
impl crate::RegisterSpec for GICD_CTLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ctlr::R`](R) reader structure"]
impl crate::Readable for GICD_CTLRrs {}
#[doc = "`write(|w| ..)` method takes [`gicd_ctlr::W`](W) writer structure"]
impl crate::Writable for GICD_CTLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_CTLR to value 0"]
impl crate::Resettable for GICD_CTLRrs {
    const RESET_VALUE: u32 = 0;
}
