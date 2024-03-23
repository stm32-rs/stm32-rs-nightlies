#[doc = "Register `TIM8_CR1` reader"]
pub type R = crate::R<TIM8_CR1rs>;
#[doc = "Register `TIM8_CR1` writer"]
pub type W = crate::W<TIM8_CR1rs>;
#[doc = "Field `CEN` reader - CEN"]
pub type CEN_R = crate::BitReader;
#[doc = "Field `CEN` writer - CEN"]
pub type CEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDIS` reader - UDIS"]
pub type UDIS_R = crate::BitReader;
#[doc = "Field `UDIS` writer - UDIS"]
pub type UDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URS` reader - URS"]
pub type URS_R = crate::BitReader;
#[doc = "Field `URS` writer - URS"]
pub type URS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPM` reader - OPM"]
pub type OPM_R = crate::BitReader;
#[doc = "Field `OPM` writer - OPM"]
pub type OPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - DIR"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - DIR"]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMS` reader - CMS"]
pub type CMS_R = crate::FieldReader;
#[doc = "Field `CMS` writer - CMS"]
pub type CMS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ARPE` reader - ARPE"]
pub type ARPE_R = crate::BitReader;
#[doc = "Field `ARPE` writer - ARPE"]
pub type ARPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKD` reader - CKD"]
pub type CKD_R = crate::FieldReader;
#[doc = "Field `CKD` writer - CKD"]
pub type CKD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UIFREMAP` reader - UIFREMAP"]
pub type UIFREMAP_R = crate::BitReader;
#[doc = "Field `UIFREMAP` writer - UIFREMAP"]
pub type UIFREMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CEN"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UDIS"]
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - URS"]
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OPM"]
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - CMS"]
    #[inline(always)]
    pub fn cms(&self) -> CMS_R {
        CMS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - ARPE"]
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - CKD"]
    #[inline(always)]
    pub fn ckd(&self) -> CKD_R {
        CKD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - UIFREMAP"]
    #[inline(always)]
    pub fn uifremap(&self) -> UIFREMAP_R {
        UIFREMAP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CEN"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<TIM8_CR1rs> {
        CEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - UDIS"]
    #[inline(always)]
    #[must_use]
    pub fn udis(&mut self) -> UDIS_W<TIM8_CR1rs> {
        UDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - URS"]
    #[inline(always)]
    #[must_use]
    pub fn urs(&mut self) -> URS_W<TIM8_CR1rs> {
        URS_W::new(self, 2)
    }
    #[doc = "Bit 3 - OPM"]
    #[inline(always)]
    #[must_use]
    pub fn opm(&mut self) -> OPM_W<TIM8_CR1rs> {
        OPM_W::new(self, 3)
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<TIM8_CR1rs> {
        DIR_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - CMS"]
    #[inline(always)]
    #[must_use]
    pub fn cms(&mut self) -> CMS_W<TIM8_CR1rs> {
        CMS_W::new(self, 5)
    }
    #[doc = "Bit 7 - ARPE"]
    #[inline(always)]
    #[must_use]
    pub fn arpe(&mut self) -> ARPE_W<TIM8_CR1rs> {
        ARPE_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - CKD"]
    #[inline(always)]
    #[must_use]
    pub fn ckd(&mut self) -> CKD_W<TIM8_CR1rs> {
        CKD_W::new(self, 8)
    }
    #[doc = "Bit 11 - UIFREMAP"]
    #[inline(always)]
    #[must_use]
    pub fn uifremap(&mut self) -> UIFREMAP_W<TIM8_CR1rs> {
        UIFREMAP_W::new(self, 11)
    }
}
#[doc = "TIM8 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim8_cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim8_cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM8_CR1rs;
impl crate::RegisterSpec for TIM8_CR1rs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim8_cr1::R`](R) reader structure"]
impl crate::Readable for TIM8_CR1rs {}
#[doc = "`write(|w| ..)` method takes [`tim8_cr1::W`](W) writer structure"]
impl crate::Writable for TIM8_CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM8_CR1 to value 0"]
impl crate::Resettable for TIM8_CR1rs {
    const RESET_VALUE: u16 = 0;
}
