#[doc = "Register `EXTCFGR` reader"]
pub type R = crate::R<EXTCFGRrs>;
#[doc = "Register `EXTCFGR` writer"]
pub type W = crate::W<EXTCFGRrs>;
#[doc = "Field `SHDHPRE` reader - Shared AHB prescaler"]
pub type SHDHPRE_R = crate::FieldReader;
#[doc = "Field `SHDHPRE` writer - Shared AHB prescaler"]
pub type SHDHPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `C2HPRE` reader - CPU2 AHB prescaler"]
pub type C2HPRE_R = crate::FieldReader;
#[doc = "Field `C2HPRE` writer - CPU2 AHB prescaler"]
pub type C2HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SHDHPREF` reader - Shared AHB prescaler flag"]
pub type SHDHPREF_R = crate::BitReader;
#[doc = "Field `C2HPREF` reader - CPU2 AHB prescaler flag"]
pub type C2HPREF_R = crate::BitReader;
#[doc = "Field `RFCSS` reader - RF clock source selected"]
pub type RFCSS_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Shared AHB prescaler"]
    #[inline(always)]
    pub fn shdhpre(&self) -> SHDHPRE_R {
        SHDHPRE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CPU2 AHB prescaler"]
    #[inline(always)]
    pub fn c2hpre(&self) -> C2HPRE_R {
        C2HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Shared AHB prescaler flag"]
    #[inline(always)]
    pub fn shdhpref(&self) -> SHDHPREF_R {
        SHDHPREF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU2 AHB prescaler flag"]
    #[inline(always)]
    pub fn c2hpref(&self) -> C2HPREF_R {
        C2HPREF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - RF clock source selected"]
    #[inline(always)]
    pub fn rfcss(&self) -> RFCSS_R {
        RFCSS_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shared AHB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn shdhpre(&mut self) -> SHDHPRE_W<EXTCFGRrs> {
        SHDHPRE_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - CPU2 AHB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn c2hpre(&mut self) -> C2HPRE_W<EXTCFGRrs> {
        C2HPRE_W::new(self, 4)
    }
}
#[doc = "Extended clock recovery register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTCFGRrs;
impl crate::RegisterSpec for EXTCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extcfgr::R`](R) reader structure"]
impl crate::Readable for EXTCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`extcfgr::W`](W) writer structure"]
impl crate::Writable for EXTCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTCFGR to value 0x0003_0000"]
impl crate::Resettable for EXTCFGRrs {
    const RESET_VALUE: u32 = 0x0003_0000;
}
