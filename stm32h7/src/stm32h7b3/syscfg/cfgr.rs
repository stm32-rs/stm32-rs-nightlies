#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `PVDL` reader - PVD lock enable bit."]
pub type PVDL_R = crate::BitReader;
#[doc = "Field `PVDL` writer - PVD lock enable bit."]
pub type PVDL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHL` reader - Flash double ECC error lock bit"]
pub type FLASHL_R = crate::BitReader;
#[doc = "Field `FLASHL` writer - Flash double ECC error lock bit"]
pub type FLASHL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM7L` reader - CortexÃ‚Â®-M7 LOCKUP (HardFault) output enable bit"]
pub type CM7L_R = crate::BitReader;
#[doc = "Field `CM7L` writer - CortexÃ‚Â®-M7 LOCKUP (HardFault) output enable bit"]
pub type CM7L_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCML` reader - D1TCM or D0TCM double ECC error signal lock"]
pub type DTCML_R = crate::BitReader;
#[doc = "Field `DTCML` writer - D1TCM or D0TCM double ECC error signal lock"]
pub type DTCML_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITCML` reader - ITCM double ECC error signal lock"]
pub type ITCML_R = crate::BitReader;
#[doc = "Field `ITCML` writer - ITCM double ECC error signal lock"]
pub type ITCML_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - PVD lock enable bit."]
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flash double ECC error lock bit"]
    #[inline(always)]
    pub fn flashl(&self) -> FLASHL_R {
        FLASHL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - CortexÃ‚Â®-M7 LOCKUP (HardFault) output enable bit"]
    #[inline(always)]
    pub fn cm7l(&self) -> CM7L_R {
        CM7L_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 13 - D1TCM or D0TCM double ECC error signal lock"]
    #[inline(always)]
    pub fn dtcml(&self) -> DTCML_R {
        DTCML_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ITCM double ECC error signal lock"]
    #[inline(always)]
    pub fn itcml(&self) -> ITCML_R {
        ITCML_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - PVD lock enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn pvdl(&mut self) -> PVDL_W<CFGRrs> {
        PVDL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Flash double ECC error lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn flashl(&mut self) -> FLASHL_W<CFGRrs> {
        FLASHL_W::new(self, 3)
    }
    #[doc = "Bit 6 - CortexÃ‚Â®-M7 LOCKUP (HardFault) output enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cm7l(&mut self) -> CM7L_W<CFGRrs> {
        CM7L_W::new(self, 6)
    }
    #[doc = "Bit 13 - D1TCM or D0TCM double ECC error signal lock"]
    #[inline(always)]
    #[must_use]
    pub fn dtcml(&mut self) -> DTCML_W<CFGRrs> {
        DTCML_W::new(self, 13)
    }
    #[doc = "Bit 14 - ITCM double ECC error signal lock"]
    #[inline(always)]
    #[must_use]
    pub fn itcml(&mut self) -> ITCML_W<CFGRrs> {
        ITCML_W::new(self, 14)
    }
}
#[doc = "SYSCFG timer break lockup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
