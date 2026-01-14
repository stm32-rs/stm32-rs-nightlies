///Register `CRCAPR` reader
pub type R = crate::R<CRCAPRrs>;
///Register `CRCAPR` writer
pub type W = crate::W<CRCAPRrs>;
///Field `CAPDHOFF` reader - delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2.
pub type CAPDHOFF_R = crate::BitReader;
///Field `CAPDHOFF` writer - delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2.
pub type CAPDHOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAPGRP` reader - group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2.
pub type CAPGRP_R = crate::BitReader;
///Field `CAPGRP` writer - group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2.
pub type CAPGRP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2.
    #[inline(always)]
    pub fn capdhoff(&self) -> CAPDHOFF_R {
        CAPDHOFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 9 - group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2.
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
    ///Bit 3 - delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2.
    #[inline(always)]
    pub fn capdhoff(&mut self) -> CAPDHOFF_W<'_, CRCAPRrs> {
        CAPDHOFF_W::new(self, 3)
    }
    ///Bit 9 - group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2.
    #[inline(always)]
    pub fn capgrp(&mut self) -> CAPGRP_W<'_, CRCAPRrs> {
        CAPGRP_W::new(self, 9)
    }
}
/**I3C controller-role capability register

You can [`read`](crate::Reg::read) this register and get [`crcapr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcapr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#I3C:CRCAPR)*/
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
