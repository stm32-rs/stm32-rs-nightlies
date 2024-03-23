#[doc = "Register `RCC_DBGCFGR` reader"]
pub type R = crate::R<RCC_DBGCFGRrs>;
#[doc = "Register `RCC_DBGCFGR` writer"]
pub type W = crate::W<RCC_DBGCFGRrs>;
#[doc = "Field `TRACEDIV` reader - TRACEDIV"]
pub type TRACEDIV_R = crate::FieldReader;
#[doc = "Field `TRACEDIV` writer - TRACEDIV"]
pub type TRACEDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBGCKEN` reader - DBGCKEN"]
pub type DBGCKEN_R = crate::BitReader;
#[doc = "Field `DBGCKEN` writer - DBGCKEN"]
pub type DBGCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACECKEN` reader - TRACECKEN"]
pub type TRACECKEN_R = crate::BitReader;
#[doc = "Field `TRACECKEN` writer - TRACECKEN"]
pub type TRACECKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGRST` reader - DBGRST"]
pub type DBGRST_R = crate::BitReader;
#[doc = "Field `DBGRST` writer - DBGRST"]
pub type DBGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - TRACEDIV"]
    #[inline(always)]
    pub fn tracediv(&self) -> TRACEDIV_R {
        TRACEDIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - DBGCKEN"]
    #[inline(always)]
    pub fn dbgcken(&self) -> DBGCKEN_R {
        DBGCKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TRACECKEN"]
    #[inline(always)]
    pub fn tracecken(&self) -> TRACECKEN_R {
        TRACECKEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - DBGRST"]
    #[inline(always)]
    pub fn dbgrst(&self) -> DBGRST_R {
        DBGRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TRACEDIV"]
    #[inline(always)]
    #[must_use]
    pub fn tracediv(&mut self) -> TRACEDIV_W<RCC_DBGCFGRrs> {
        TRACEDIV_W::new(self, 0)
    }
    #[doc = "Bit 8 - DBGCKEN"]
    #[inline(always)]
    #[must_use]
    pub fn dbgcken(&mut self) -> DBGCKEN_W<RCC_DBGCFGRrs> {
        DBGCKEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - TRACECKEN"]
    #[inline(always)]
    #[must_use]
    pub fn tracecken(&mut self) -> TRACECKEN_W<RCC_DBGCFGRrs> {
        TRACECKEN_W::new(self, 9)
    }
    #[doc = "Bit 12 - DBGRST"]
    #[inline(always)]
    #[must_use]
    pub fn dbgrst(&mut self) -> DBGRST_W<RCC_DBGCFGRrs> {
        DBGRST_W::new(self, 12)
    }
}
#[doc = "This is register contains the enable control of the debug and trace function, and the clock divider for the trace function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_dbgcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_dbgcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_DBGCFGRrs;
impl crate::RegisterSpec for RCC_DBGCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_dbgcfgr::R`](R) reader structure"]
impl crate::Readable for RCC_DBGCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_dbgcfgr::W`](W) writer structure"]
impl crate::Writable for RCC_DBGCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_DBGCFGR to value 0x01"]
impl crate::Resettable for RCC_DBGCFGRrs {
    const RESET_VALUE: u32 = 0x01;
}
