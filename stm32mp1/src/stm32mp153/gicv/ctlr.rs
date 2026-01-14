///Register `CTLR` reader
pub type R = crate::R<CTLRrs>;
///Register `CTLR` writer
pub type W = crate::W<CTLRrs>;
///Field `ENABLEGRP0` reader - ENABLEGRP0
pub type ENABLEGRP0_R = crate::BitReader;
///Field `ENABLEGRP0` writer - ENABLEGRP0
pub type ENABLEGRP0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENABLEGRP1` reader - ENABLEGRP1
pub type ENABLEGRP1_R = crate::BitReader;
///Field `ENABLEGRP1` writer - ENABLEGRP1
pub type ENABLEGRP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACKCTL` reader - ACKCTL
pub type ACKCTL_R = crate::BitReader;
///Field `ACKCTL` writer - ACKCTL
pub type ACKCTL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIQEN` reader - FIQEN
pub type FIQEN_R = crate::BitReader;
///Field `FIQEN` writer - FIQEN
pub type FIQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CBPR` reader - CBPR
pub type CBPR_R = crate::BitReader;
///Field `CBPR` writer - CBPR
pub type CBPR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOIMODE` reader - EOIMODE
pub type EOIMODE_R = crate::BitReader;
///Field `EOIMODE` writer - EOIMODE
pub type EOIMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ENABLEGRP0
    #[inline(always)]
    pub fn enablegrp0(&self) -> ENABLEGRP0_R {
        ENABLEGRP0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ENABLEGRP1
    #[inline(always)]
    pub fn enablegrp1(&self) -> ENABLEGRP1_R {
        ENABLEGRP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ACKCTL
    #[inline(always)]
    pub fn ackctl(&self) -> ACKCTL_R {
        ACKCTL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FIQEN
    #[inline(always)]
    pub fn fiqen(&self) -> FIQEN_R {
        FIQEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CBPR
    #[inline(always)]
    pub fn cbpr(&self) -> CBPR_R {
        CBPR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 9 - EOIMODE
    #[inline(always)]
    pub fn eoimode(&self) -> EOIMODE_R {
        EOIMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTLR")
            .field("enablegrp0", &self.enablegrp0())
            .field("enablegrp1", &self.enablegrp1())
            .field("ackctl", &self.ackctl())
            .field("fiqen", &self.fiqen())
            .field("cbpr", &self.cbpr())
            .field("eoimode", &self.eoimode())
            .finish()
    }
}
impl W {
    ///Bit 0 - ENABLEGRP0
    #[inline(always)]
    pub fn enablegrp0(&mut self) -> ENABLEGRP0_W<'_, CTLRrs> {
        ENABLEGRP0_W::new(self, 0)
    }
    ///Bit 1 - ENABLEGRP1
    #[inline(always)]
    pub fn enablegrp1(&mut self) -> ENABLEGRP1_W<'_, CTLRrs> {
        ENABLEGRP1_W::new(self, 1)
    }
    ///Bit 2 - ACKCTL
    #[inline(always)]
    pub fn ackctl(&mut self) -> ACKCTL_W<'_, CTLRrs> {
        ACKCTL_W::new(self, 2)
    }
    ///Bit 3 - FIQEN
    #[inline(always)]
    pub fn fiqen(&mut self) -> FIQEN_W<'_, CTLRrs> {
        FIQEN_W::new(self, 3)
    }
    ///Bit 4 - CBPR
    #[inline(always)]
    pub fn cbpr(&mut self) -> CBPR_W<'_, CTLRrs> {
        CBPR_W::new(self, 4)
    }
    ///Bit 9 - EOIMODE
    #[inline(always)]
    pub fn eoimode(&mut self) -> EOIMODE_W<'_, CTLRrs> {
        EOIMODE_W::new(self, 9)
    }
}
/**GICV virtual machine control register

You can [`read`](crate::Reg::read) this register and get [`ctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:CTLR)*/
pub struct CTLRrs;
impl crate::RegisterSpec for CTLRrs {
    type Ux = u32;
}
///`read()` method returns [`ctlr::R`](R) reader structure
impl crate::Readable for CTLRrs {}
///`write(|w| ..)` method takes [`ctlr::W`](W) writer structure
impl crate::Writable for CTLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTLR to value 0
impl crate::Resettable for CTLRrs {}
