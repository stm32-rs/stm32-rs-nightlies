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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTLR")
            .field("enablegrp0", &self.enablegrp0())
            .field("enablegrp1", &self.enablegrp1())
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
}
/**GICD control register

You can [`read`](crate::Reg::read) this register and get [`ctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:CTLR)*/
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
