#[doc = "Register `ADF_SADCR` reader"]
pub type R = crate::R<ADF_SADCRrs>;
#[doc = "Register `ADF_SADCR` writer"]
pub type W = crate::W<ADF_SADCRrs>;
#[doc = "Field `SADEN` reader - Sound activity detector enable"]
pub type SADEN_R = crate::BitReader;
#[doc = "Field `SADEN` writer - Sound activity detector enable"]
pub type SADEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATCAP` reader - Data capture mode"]
pub type DATCAP_R = crate::FieldReader;
#[doc = "Field `DATCAP` writer - Data capture mode"]
pub type DATCAP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DETCFG` reader - Sound trigger event configuration"]
pub type DETCFG_R = crate::BitReader;
#[doc = "Field `DETCFG` writer - Sound trigger event configuration"]
pub type DETCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADST` reader - SAD state"]
pub type SADST_R = crate::FieldReader;
#[doc = "Field `HYSTEN` reader - Hysteresis enable"]
pub type HYSTEN_R = crate::BitReader;
#[doc = "Field `HYSTEN` writer - Hysteresis enable"]
pub type HYSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRSIZE` reader - Frame size"]
pub type FRSIZE_R = crate::FieldReader;
#[doc = "Field `FRSIZE` writer - Frame size"]
pub type FRSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SADMOD` reader - SAD working mode"]
pub type SADMOD_R = crate::FieldReader;
#[doc = "Field `SADMOD` writer - SAD working mode"]
pub type SADMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SADACTIVE` reader - SAD Active flag"]
pub type SADACTIVE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Sound activity detector enable"]
    #[inline(always)]
    pub fn saden(&self) -> SADEN_R {
        SADEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data capture mode"]
    #[inline(always)]
    pub fn datcap(&self) -> DATCAP_R {
        DATCAP_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Sound trigger event configuration"]
    #[inline(always)]
    pub fn detcfg(&self) -> DETCFG_R {
        DETCFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SAD state"]
    #[inline(always)]
    pub fn sadst(&self) -> SADST_R {
        SADST_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Hysteresis enable"]
    #[inline(always)]
    pub fn hysten(&self) -> HYSTEN_R {
        HYSTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Frame size"]
    #[inline(always)]
    pub fn frsize(&self) -> FRSIZE_R {
        FRSIZE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - SAD working mode"]
    #[inline(always)]
    pub fn sadmod(&self) -> SADMOD_R {
        SADMOD_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 31 - SAD Active flag"]
    #[inline(always)]
    pub fn sadactive(&self) -> SADACTIVE_R {
        SADACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sound activity detector enable"]
    #[inline(always)]
    #[must_use]
    pub fn saden(&mut self) -> SADEN_W<ADF_SADCRrs> {
        SADEN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Data capture mode"]
    #[inline(always)]
    #[must_use]
    pub fn datcap(&mut self) -> DATCAP_W<ADF_SADCRrs> {
        DATCAP_W::new(self, 1)
    }
    #[doc = "Bit 3 - Sound trigger event configuration"]
    #[inline(always)]
    #[must_use]
    pub fn detcfg(&mut self) -> DETCFG_W<ADF_SADCRrs> {
        DETCFG_W::new(self, 3)
    }
    #[doc = "Bit 7 - Hysteresis enable"]
    #[inline(always)]
    #[must_use]
    pub fn hysten(&mut self) -> HYSTEN_W<ADF_SADCRrs> {
        HYSTEN_W::new(self, 7)
    }
    #[doc = "Bits 8:10 - Frame size"]
    #[inline(always)]
    #[must_use]
    pub fn frsize(&mut self) -> FRSIZE_W<ADF_SADCRrs> {
        FRSIZE_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - SAD working mode"]
    #[inline(always)]
    #[must_use]
    pub fn sadmod(&mut self) -> SADMOD_W<ADF_SADCRrs> {
        SADMOD_W::new(self, 12)
    }
}
#[doc = "ADF SAD control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adf_sadcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adf_sadcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADF_SADCRrs;
impl crate::RegisterSpec for ADF_SADCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adf_sadcr::R`](R) reader structure"]
impl crate::Readable for ADF_SADCRrs {}
#[doc = "`write(|w| ..)` method takes [`adf_sadcr::W`](W) writer structure"]
impl crate::Writable for ADF_SADCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADF_SADCR to value 0"]
impl crate::Resettable for ADF_SADCRrs {
    const RESET_VALUE: u32 = 0;
}
