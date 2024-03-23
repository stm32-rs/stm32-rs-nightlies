#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "Field `CEN` reader - Counter enable"]
pub type CEN_R = crate::BitReader;
#[doc = "Field `CEN` writer - Counter enable"]
pub type CEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDIS` reader - Update disable"]
pub type UDIS_R = crate::BitReader;
#[doc = "Field `UDIS` writer - Update disable"]
pub type UDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URS` reader - Update request source"]
pub type URS_R = crate::BitReader;
#[doc = "Field `URS` writer - Update request source"]
pub type URS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPM` reader - One-pulse mode"]
pub type OPM_R = crate::BitReader;
#[doc = "Field `OPM` writer - One-pulse mode"]
pub type OPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - Direction"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - Direction"]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMS` reader - Center-aligned mode selection"]
pub type CMS_R = crate::FieldReader;
#[doc = "Field `CMS` writer - Center-aligned mode selection"]
pub type CMS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ARPE` reader - Auto-reload preload enable"]
pub type ARPE_R = crate::BitReader;
#[doc = "Field `ARPE` writer - Auto-reload preload enable"]
pub type ARPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKD` reader - Clock division"]
pub type CKD_R = crate::FieldReader;
#[doc = "Field `CKD` writer - Clock division"]
pub type CKD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UIFREMAP` reader - UIF status bit remapping"]
pub type UIFREMAP_R = crate::BitReader;
#[doc = "Field `UIFREMAP` writer - UIF status bit remapping"]
pub type UIFREMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DITHEN` reader - Dithering Enable"]
pub type DITHEN_R = crate::BitReader;
#[doc = "Field `DITHEN` writer - Dithering Enable"]
pub type DITHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Center-aligned mode selection"]
    #[inline(always)]
    pub fn cms(&self) -> CMS_R {
        CMS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn ckd(&self) -> CKD_R {
        CKD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - UIF status bit remapping"]
    #[inline(always)]
    pub fn uifremap(&self) -> UIFREMAP_R {
        UIFREMAP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Dithering Enable"]
    #[inline(always)]
    pub fn dithen(&self) -> DITHEN_R {
        DITHEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<CR1rs> {
        CEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    #[must_use]
    pub fn udis(&mut self) -> UDIS_W<CR1rs> {
        UDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    #[must_use]
    pub fn urs(&mut self) -> URS_W<CR1rs> {
        URS_W::new(self, 2)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    #[must_use]
    pub fn opm(&mut self) -> OPM_W<CR1rs> {
        OPM_W::new(self, 3)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<CR1rs> {
        DIR_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Center-aligned mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn cms(&mut self) -> CMS_W<CR1rs> {
        CMS_W::new(self, 5)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn arpe(&mut self) -> ARPE_W<CR1rs> {
        ARPE_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    #[must_use]
    pub fn ckd(&mut self) -> CKD_W<CR1rs> {
        CKD_W::new(self, 8)
    }
    #[doc = "Bit 11 - UIF status bit remapping"]
    #[inline(always)]
    #[must_use]
    pub fn uifremap(&mut self) -> UIFREMAP_W<CR1rs> {
        UIFREMAP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Dithering Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dithen(&mut self) -> DITHEN_W<CR1rs> {
        DITHEN_W::new(self, 12)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0;
}
