#[doc = "Register `I3C_CRCAPR` reader"]
pub type R = crate::R<I3C_CRCAPRrs>;
#[doc = "Register `I3C_CRCAPR` writer"]
pub type W = crate::W<I3C_CRCAPRrs>;
#[doc = "Field `CAPDHOFF` reader - delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2."]
pub type CAPDHOFF_R = crate::BitReader;
#[doc = "Field `CAPDHOFF` writer - delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2."]
pub type CAPDHOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPGRP` reader - group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2."]
pub type CAPGRP_R = crate::BitReader;
#[doc = "Field `CAPGRP` writer - group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2."]
pub type CAPGRP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2."]
    #[inline(always)]
    pub fn capdhoff(&self) -> CAPDHOFF_R {
        CAPDHOFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2."]
    #[inline(always)]
    pub fn capgrp(&self) -> CAPGRP_R {
        CAPGRP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2."]
    #[inline(always)]
    #[must_use]
    pub fn capdhoff(&mut self) -> CAPDHOFF_W<I3C_CRCAPRrs> {
        CAPDHOFF_W::new(self, 3)
    }
    #[doc = "Bit 9 - group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2."]
    #[inline(always)]
    #[must_use]
    pub fn capgrp(&mut self) -> CAPGRP_W<I3C_CRCAPRrs> {
        CAPGRP_W::new(self, 9)
    }
}
#[doc = "I3C controller-role capability register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i3c_crcapr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i3c_crcapr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3C_CRCAPRrs;
impl crate::RegisterSpec for I3C_CRCAPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3c_crcapr::R`](R) reader structure"]
impl crate::Readable for I3C_CRCAPRrs {}
#[doc = "`write(|w| ..)` method takes [`i3c_crcapr::W`](W) writer structure"]
impl crate::Writable for I3C_CRCAPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I3C_CRCAPR to value 0"]
impl crate::Resettable for I3C_CRCAPRrs {
    const RESET_VALUE: u32 = 0;
}
