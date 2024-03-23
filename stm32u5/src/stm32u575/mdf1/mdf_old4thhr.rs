#[doc = "Register `MDF_OLD4THHR` reader"]
pub type R = crate::R<MDF_OLD4THHRrs>;
#[doc = "Register `MDF_OLD4THHR` writer"]
pub type W = crate::W<MDF_OLD4THHRrs>;
#[doc = "Field `OLDTHH` reader - OLD High Threshold Value Set and cleared by software. OLDTHH represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHH. This field can be write-protected, please refer to Section 1.4.15: Register protection for details"]
pub type OLDTHH_R = crate::FieldReader<u32>;
#[doc = "Field `OLDTHH` writer - OLD High Threshold Value Set and cleared by software. OLDTHH represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHH. This field can be write-protected, please refer to Section 1.4.15: Register protection for details"]
pub type OLDTHH_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - OLD High Threshold Value Set and cleared by software. OLDTHH represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHH. This field can be write-protected, please refer to Section 1.4.15: Register protection for details"]
    #[inline(always)]
    pub fn oldthh(&self) -> OLDTHH_R {
        OLDTHH_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - OLD High Threshold Value Set and cleared by software. OLDTHH represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHH. This field can be write-protected, please refer to Section 1.4.15: Register protection for details"]
    #[inline(always)]
    #[must_use]
    pub fn oldthh(&mut self) -> OLDTHH_W<MDF_OLD4THHRrs> {
        OLDTHH_W::new(self, 0)
    }
}
#[doc = "This register is used for the adjustment of the Out-off Limit high threshold.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdf_old4thhr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdf_old4thhr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDF_OLD4THHRrs;
impl crate::RegisterSpec for MDF_OLD4THHRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdf_old4thhr::R`](R) reader structure"]
impl crate::Readable for MDF_OLD4THHRrs {}
#[doc = "`write(|w| ..)` method takes [`mdf_old4thhr::W`](W) writer structure"]
impl crate::Writable for MDF_OLD4THHRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDF_OLD4THHR to value 0"]
impl crate::Resettable for MDF_OLD4THHRrs {
    const RESET_VALUE: u32 = 0;
}
