///Register `CRCAPR` reader
pub type R = crate::R<CRCAPRrs>;
///Register `CRCAPR` writer
pub type W = crate::W<CRCAPRrs>;
///Field `CAPDHOFF` reader - delayed controller-role hand-off
pub type CAPDHOFF_R = crate::BitReader;
///Field `CAPDHOFF` writer - delayed controller-role hand-off
pub type CAPDHOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAPGRP` reader - group management support (when acting as controller)
pub type CAPGRP_R = crate::BitReader;
///Field `CAPGRP` writer - group management support (when acting as controller)
pub type CAPGRP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - delayed controller-role hand-off
    #[inline(always)]
    pub fn capdhoff(&self) -> CAPDHOFF_R {
        CAPDHOFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 9 - group management support (when acting as controller)
    #[inline(always)]
    pub fn capgrp(&self) -> CAPGRP_R {
        CAPGRP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCAPR")
            .field("capdhoff", &self.capdhoff())
            .field("capgrp", &self.capgrp())
            .finish()
    }
}
impl W {
    ///Bit 3 - delayed controller-role hand-off
    #[inline(always)]
    pub fn capdhoff(&mut self) -> CAPDHOFF_W<'_, CRCAPRrs> {
        CAPDHOFF_W::new(self, 3)
    }
    ///Bit 9 - group management support (when acting as controller)
    #[inline(always)]
    pub fn capgrp(&mut self) -> CAPGRP_W<'_, CRCAPRrs> {
        CAPGRP_W::new(self, 9)
    }
}
/**I3C controller-role capability register

You can [`read`](crate::Reg::read) this register and get [`crcapr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcapr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#I3C1:CRCAPR)*/
pub struct CRCAPRrs;
impl crate::RegisterSpec for CRCAPRrs {
    type Ux = u32;
}
///`read()` method returns [`crcapr::R`](R) reader structure
impl crate::Readable for CRCAPRrs {}
///`write(|w| ..)` method takes [`crcapr::W`](W) writer structure
impl crate::Writable for CRCAPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCAPR to value 0
impl crate::Resettable for CRCAPRrs {}
