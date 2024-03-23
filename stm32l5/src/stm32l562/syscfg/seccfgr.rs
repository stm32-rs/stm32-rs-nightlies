#[doc = "Register `SECCFGR` reader"]
pub type R = crate::R<SECCFGRrs>;
#[doc = "Register `SECCFGR` writer"]
pub type W = crate::W<SECCFGRrs>;
#[doc = "Field `SYSCFGSEC` reader - SYSCFG clock control security"]
pub type SYSCFGSEC_R = crate::BitReader;
#[doc = "Field `SYSCFGSEC` writer - SYSCFG clock control security"]
pub type SYSCFGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLASSBSEC` reader - ClassB security"]
pub type CLASSBSEC_R = crate::BitReader;
#[doc = "Field `CLASSBSEC` writer - ClassB security"]
pub type CLASSBSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2SEC` reader - SRAM2 security"]
pub type SRAM2SEC_R = crate::BitReader;
#[doc = "Field `SRAM2SEC` writer - SRAM2 security"]
pub type SRAM2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPUSEC` reader - FPUSEC"]
pub type FPUSEC_R = crate::BitReader;
#[doc = "Field `FPUSEC` writer - FPUSEC"]
pub type FPUSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSCFG clock control security"]
    #[inline(always)]
    pub fn syscfgsec(&self) -> SYSCFGSEC_R {
        SYSCFGSEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ClassB security"]
    #[inline(always)]
    pub fn classbsec(&self) -> CLASSBSEC_R {
        CLASSBSEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM2 security"]
    #[inline(always)]
    pub fn sram2sec(&self) -> SRAM2SEC_R {
        SRAM2SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FPUSEC"]
    #[inline(always)]
    pub fn fpusec(&self) -> FPUSEC_R {
        FPUSEC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG clock control security"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgsec(&mut self) -> SYSCFGSEC_W<SECCFGRrs> {
        SYSCFGSEC_W::new(self, 0)
    }
    #[doc = "Bit 1 - ClassB security"]
    #[inline(always)]
    #[must_use]
    pub fn classbsec(&mut self) -> CLASSBSEC_W<SECCFGRrs> {
        CLASSBSEC_W::new(self, 1)
    }
    #[doc = "Bit 2 - SRAM2 security"]
    #[inline(always)]
    #[must_use]
    pub fn sram2sec(&mut self) -> SRAM2SEC_W<SECCFGRrs> {
        SRAM2SEC_W::new(self, 2)
    }
    #[doc = "Bit 3 - FPUSEC"]
    #[inline(always)]
    #[must_use]
    pub fn fpusec(&mut self) -> FPUSEC_W<SECCFGRrs> {
        FPUSEC_W::new(self, 3)
    }
}
#[doc = "SYSCFG secure configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seccfgr::R`](R) reader structure"]
impl crate::Readable for SECCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure"]
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECCFGR to value 0"]
impl crate::Resettable for SECCFGRrs {
    const RESET_VALUE: u32 = 0;
}
