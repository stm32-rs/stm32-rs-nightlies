#[doc = "Register `OTG_HCFG` reader"]
pub type R = crate::R<OTG_HCFGrs>;
#[doc = "Register `OTG_HCFG` writer"]
pub type W = crate::W<OTG_HCFGrs>;
#[doc = "Field `FSLSPCS` reader - FSLSPCS"]
pub type FSLSPCS_R = crate::FieldReader;
#[doc = "Field `FSLSPCS` writer - FSLSPCS"]
pub type FSLSPCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FSLSS` reader - FSLSS"]
pub type FSLSS_R = crate::BitReader;
#[doc = "Field `DESCDMA` reader - DESCDMA"]
pub type DESCDMA_R = crate::BitReader;
#[doc = "Field `DESCDMA` writer - DESCDMA"]
pub type DESCDMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRLSTEN` reader - FRLSTEN"]
pub type FRLSTEN_R = crate::FieldReader;
#[doc = "Field `FRLSTEN` writer - FRLSTEN"]
pub type FRLSTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PERSSCHEDENA` reader - PERSSCHEDENA"]
pub type PERSSCHEDENA_R = crate::BitReader;
#[doc = "Field `PERSSCHEDENA` writer - PERSSCHEDENA"]
pub type PERSSCHEDENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FSLSPCS"]
    #[inline(always)]
    pub fn fslspcs(&self) -> FSLSPCS_R {
        FSLSPCS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FSLSS"]
    #[inline(always)]
    pub fn fslss(&self) -> FSLSS_R {
        FSLSS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 23 - DESCDMA"]
    #[inline(always)]
    pub fn descdma(&self) -> DESCDMA_R {
        DESCDMA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - FRLSTEN"]
    #[inline(always)]
    pub fn frlsten(&self) -> FRLSTEN_R {
        FRLSTEN_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - PERSSCHEDENA"]
    #[inline(always)]
    pub fn persschedena(&self) -> PERSSCHEDENA_R {
        PERSSCHEDENA_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FSLSPCS"]
    #[inline(always)]
    #[must_use]
    pub fn fslspcs(&mut self) -> FSLSPCS_W<OTG_HCFGrs> {
        FSLSPCS_W::new(self, 0)
    }
    #[doc = "Bit 23 - DESCDMA"]
    #[inline(always)]
    #[must_use]
    pub fn descdma(&mut self) -> DESCDMA_W<OTG_HCFGrs> {
        DESCDMA_W::new(self, 23)
    }
    #[doc = "Bits 24:25 - FRLSTEN"]
    #[inline(always)]
    #[must_use]
    pub fn frlsten(&mut self) -> FRLSTEN_W<OTG_HCFGrs> {
        FRLSTEN_W::new(self, 24)
    }
    #[doc = "Bit 26 - PERSSCHEDENA"]
    #[inline(always)]
    #[must_use]
    pub fn persschedena(&mut self) -> PERSSCHEDENA_W<OTG_HCFGrs> {
        PERSSCHEDENA_W::new(self, 26)
    }
}
#[doc = "This register configures the core after power-on. Do not make changes to this register after initializing the host.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HCFGrs;
impl crate::RegisterSpec for OTG_HCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hcfg::R`](R) reader structure"]
impl crate::Readable for OTG_HCFGrs {}
#[doc = "`write(|w| ..)` method takes [`otg_hcfg::W`](W) writer structure"]
impl crate::Writable for OTG_HCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_HCFG to value 0"]
impl crate::Resettable for OTG_HCFGrs {
    const RESET_VALUE: u32 = 0;
}
