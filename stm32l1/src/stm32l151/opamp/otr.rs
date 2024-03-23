#[doc = "Register `OTR` reader"]
pub type R = crate::R<OTRrs>;
#[doc = "Register `OTR` writer"]
pub type W = crate::W<OTRrs>;
#[doc = "Field `AO1_OPT_OFFSET_TRIM` reader - OPAMP1, 10-bit offset trim value for normal mode"]
pub type AO1_OPT_OFFSET_TRIM_R = crate::FieldReader<u16>;
#[doc = "Field `AO1_OPT_OFFSET_TRIM` writer - OPAMP1, 10-bit offset trim value for normal mode"]
pub type AO1_OPT_OFFSET_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `AO2_OPT_OFFSET_TRIM` reader - OPAMP2, 10-bit offset trim value for normal mode"]
pub type AO2_OPT_OFFSET_TRIM_R = crate::FieldReader<u16>;
#[doc = "Field `AO2_OPT_OFFSET_TRIM` writer - OPAMP2, 10-bit offset trim value for normal mode"]
pub type AO2_OPT_OFFSET_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `AO3_OPT_OFFSET_TRIM` reader - OPAMP3, 10-bit offset trim value for normal mode"]
pub type AO3_OPT_OFFSET_TRIM_R = crate::FieldReader<u16>;
#[doc = "Field `AO3_OPT_OFFSET_TRIM` writer - OPAMP3, 10-bit offset trim value for normal mode"]
pub type AO3_OPT_OFFSET_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `OT_USER` reader - Select user or factory trimming value"]
pub type OT_USER_R = crate::BitReader;
#[doc = "Field `OT_USER` writer - Select user or factory trimming value"]
pub type OT_USER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - OPAMP1, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    pub fn ao1_opt_offset_trim(&self) -> AO1_OPT_OFFSET_TRIM_R {
        AO1_OPT_OFFSET_TRIM_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - OPAMP2, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    pub fn ao2_opt_offset_trim(&self) -> AO2_OPT_OFFSET_TRIM_R {
        AO2_OPT_OFFSET_TRIM_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - OPAMP3, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    pub fn ao3_opt_offset_trim(&self) -> AO3_OPT_OFFSET_TRIM_R {
        AO3_OPT_OFFSET_TRIM_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Select user or factory trimming value"]
    #[inline(always)]
    pub fn ot_user(&self) -> OT_USER_R {
        OT_USER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - OPAMP1, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    #[must_use]
    pub fn ao1_opt_offset_trim(&mut self) -> AO1_OPT_OFFSET_TRIM_W<OTRrs> {
        AO1_OPT_OFFSET_TRIM_W::new(self, 0)
    }
    #[doc = "Bits 10:19 - OPAMP2, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    #[must_use]
    pub fn ao2_opt_offset_trim(&mut self) -> AO2_OPT_OFFSET_TRIM_W<OTRrs> {
        AO2_OPT_OFFSET_TRIM_W::new(self, 10)
    }
    #[doc = "Bits 20:29 - OPAMP3, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    #[must_use]
    pub fn ao3_opt_offset_trim(&mut self) -> AO3_OPT_OFFSET_TRIM_W<OTRrs> {
        AO3_OPT_OFFSET_TRIM_W::new(self, 20)
    }
    #[doc = "Bit 31 - Select user or factory trimming value"]
    #[inline(always)]
    #[must_use]
    pub fn ot_user(&mut self) -> OT_USER_W<OTRrs> {
        OT_USER_W::new(self, 31)
    }
}
#[doc = "offset trimming register for normal mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTRrs;
impl crate::RegisterSpec for OTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otr::R`](R) reader structure"]
impl crate::Readable for OTRrs {}
#[doc = "`write(|w| ..)` method takes [`otr::W`](W) writer structure"]
impl crate::Writable for OTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTR to value 0"]
impl crate::Resettable for OTRrs {
    const RESET_VALUE: u32 = 0;
}
