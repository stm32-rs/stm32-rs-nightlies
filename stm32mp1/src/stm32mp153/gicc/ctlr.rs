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
///Field `FIQBYPDISGRP0` reader - FIQBYPDISGRP0
pub type FIQBYPDISGRP0_R = crate::BitReader;
///Field `FIQBYPDISGRP0` writer - FIQBYPDISGRP0
pub type FIQBYPDISGRP0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IRQBYPDISGRP0` reader - IRQBYPDISGRP0
pub type IRQBYPDISGRP0_R = crate::BitReader;
///Field `IRQBYPDISGRP0` writer - IRQBYPDISGRP0
pub type IRQBYPDISGRP0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIQBYPDISGRP1` reader - FIQBYPDISGRP1
pub type FIQBYPDISGRP1_R = crate::BitReader;
///Field `FIQBYPDISGRP1` writer - FIQBYPDISGRP1
pub type FIQBYPDISGRP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IRQBYPDISGRP1` reader - IRQBYPDISGRP1
pub type IRQBYPDISGRP1_R = crate::BitReader;
///Field `IRQBYPDISGRP1` writer - IRQBYPDISGRP1
pub type IRQBYPDISGRP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOIMODES` reader - EOIMODES
pub type EOIMODES_R = crate::BitReader;
///Field `EOIMODES` writer - EOIMODES
pub type EOIMODES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOIMODENS` reader - EOIMODENS
pub type EOIMODENS_R = crate::BitReader;
///Field `EOIMODENS` writer - EOIMODENS
pub type EOIMODENS_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 5 - FIQBYPDISGRP0
    #[inline(always)]
    pub fn fiqbypdisgrp0(&self) -> FIQBYPDISGRP0_R {
        FIQBYPDISGRP0_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IRQBYPDISGRP0
    #[inline(always)]
    pub fn irqbypdisgrp0(&self) -> IRQBYPDISGRP0_R {
        IRQBYPDISGRP0_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - FIQBYPDISGRP1
    #[inline(always)]
    pub fn fiqbypdisgrp1(&self) -> FIQBYPDISGRP1_R {
        FIQBYPDISGRP1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IRQBYPDISGRP1
    #[inline(always)]
    pub fn irqbypdisgrp1(&self) -> IRQBYPDISGRP1_R {
        IRQBYPDISGRP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - EOIMODES
    #[inline(always)]
    pub fn eoimodes(&self) -> EOIMODES_R {
        EOIMODES_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - EOIMODENS
    #[inline(always)]
    pub fn eoimodens(&self) -> EOIMODENS_R {
        EOIMODENS_R::new(((self.bits >> 10) & 1) != 0)
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
            .field("fiqbypdisgrp0", &self.fiqbypdisgrp0())
            .field("irqbypdisgrp0", &self.irqbypdisgrp0())
            .field("fiqbypdisgrp1", &self.fiqbypdisgrp1())
            .field("irqbypdisgrp1", &self.irqbypdisgrp1())
            .field("eoimodes", &self.eoimodes())
            .field("eoimodens", &self.eoimodens())
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
    ///Bit 5 - FIQBYPDISGRP0
    #[inline(always)]
    pub fn fiqbypdisgrp0(&mut self) -> FIQBYPDISGRP0_W<'_, CTLRrs> {
        FIQBYPDISGRP0_W::new(self, 5)
    }
    ///Bit 6 - IRQBYPDISGRP0
    #[inline(always)]
    pub fn irqbypdisgrp0(&mut self) -> IRQBYPDISGRP0_W<'_, CTLRrs> {
        IRQBYPDISGRP0_W::new(self, 6)
    }
    ///Bit 7 - FIQBYPDISGRP1
    #[inline(always)]
    pub fn fiqbypdisgrp1(&mut self) -> FIQBYPDISGRP1_W<'_, CTLRrs> {
        FIQBYPDISGRP1_W::new(self, 7)
    }
    ///Bit 8 - IRQBYPDISGRP1
    #[inline(always)]
    pub fn irqbypdisgrp1(&mut self) -> IRQBYPDISGRP1_W<'_, CTLRrs> {
        IRQBYPDISGRP1_W::new(self, 8)
    }
    ///Bit 9 - EOIMODES
    #[inline(always)]
    pub fn eoimodes(&mut self) -> EOIMODES_W<'_, CTLRrs> {
        EOIMODES_W::new(self, 9)
    }
    ///Bit 10 - EOIMODENS
    #[inline(always)]
    pub fn eoimodens(&mut self) -> EOIMODENS_W<'_, CTLRrs> {
        EOIMODENS_W::new(self, 10)
    }
}
/**GICC control register

You can [`read`](crate::Reg::read) this register and get [`ctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:CTLR)*/
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
