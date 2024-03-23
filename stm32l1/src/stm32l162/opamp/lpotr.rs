#[doc = "Register `LPOTR` reader"]
pub type R = crate::R<LPOTRrs>;
#[doc = "Register `LPOTR` writer"]
pub type W = crate::W<LPOTRrs>;
#[doc = "Field `AO1_OPT_OFFSET_TRIM_LP` reader - OPAMP1, 10-bit offset trim value for low power mode"]
pub type AO1_OPT_OFFSET_TRIM_LP_R = crate::FieldReader<u16>;
#[doc = "Field `AO1_OPT_OFFSET_TRIM_LP` writer - OPAMP1, 10-bit offset trim value for low power mode"]
pub type AO1_OPT_OFFSET_TRIM_LP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `AO2_OPT_OFFSET_TRIM_LP` reader - OPAMP2, 10-bit offset trim value for low power mode"]
pub type AO2_OPT_OFFSET_TRIM_LP_R = crate::FieldReader<u16>;
#[doc = "Field `AO2_OPT_OFFSET_TRIM_LP` writer - OPAMP2, 10-bit offset trim value for low power mode"]
pub type AO2_OPT_OFFSET_TRIM_LP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `AO3_OPT_OFFSET_TRIM_LP` reader - OPAMP3, 10-bit offset trim value for low power mode"]
pub type AO3_OPT_OFFSET_TRIM_LP_R = crate::FieldReader<u16>;
#[doc = "Field `AO3_OPT_OFFSET_TRIM_LP` writer - OPAMP3, 10-bit offset trim value for low power mode"]
pub type AO3_OPT_OFFSET_TRIM_LP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - OPAMP1, 10-bit offset trim value for low power mode"]
    #[inline(always)]
    pub fn ao1_opt_offset_trim_lp(&self) -> AO1_OPT_OFFSET_TRIM_LP_R {
        AO1_OPT_OFFSET_TRIM_LP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - OPAMP2, 10-bit offset trim value for low power mode"]
    #[inline(always)]
    pub fn ao2_opt_offset_trim_lp(&self) -> AO2_OPT_OFFSET_TRIM_LP_R {
        AO2_OPT_OFFSET_TRIM_LP_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - OPAMP3, 10-bit offset trim value for low power mode"]
    #[inline(always)]
    pub fn ao3_opt_offset_trim_lp(&self) -> AO3_OPT_OFFSET_TRIM_LP_R {
        AO3_OPT_OFFSET_TRIM_LP_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - OPAMP1, 10-bit offset trim value for low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn ao1_opt_offset_trim_lp(&mut self) -> AO1_OPT_OFFSET_TRIM_LP_W<LPOTRrs> {
        AO1_OPT_OFFSET_TRIM_LP_W::new(self, 0)
    }
    #[doc = "Bits 10:19 - OPAMP2, 10-bit offset trim value for low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn ao2_opt_offset_trim_lp(&mut self) -> AO2_OPT_OFFSET_TRIM_LP_W<LPOTRrs> {
        AO2_OPT_OFFSET_TRIM_LP_W::new(self, 10)
    }
    #[doc = "Bits 20:29 - OPAMP3, 10-bit offset trim value for low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn ao3_opt_offset_trim_lp(&mut self) -> AO3_OPT_OFFSET_TRIM_LP_W<LPOTRrs> {
        AO3_OPT_OFFSET_TRIM_LP_W::new(self, 20)
    }
}
#[doc = "OPAMP offset trimming register for low power mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpotr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpotr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPOTRrs;
impl crate::RegisterSpec for LPOTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpotr::R`](R) reader structure"]
impl crate::Readable for LPOTRrs {}
#[doc = "`write(|w| ..)` method takes [`lpotr::W`](W) writer structure"]
impl crate::Writable for LPOTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPOTR to value 0"]
impl crate::Resettable for LPOTRrs {
    const RESET_VALUE: u32 = 0;
}
