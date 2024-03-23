#[doc = "Register `MDF_OLD0THLR` reader"]
pub type R = crate::R<MDF_OLD0THLRrs>;
#[doc = "Register `MDF_OLD0THLR` writer"]
pub type W = crate::W<MDF_OLD0THLRrs>;
#[doc = "Field `OLDTHL` reader - OLD Low Threshold Value Set and cleared by software. OLDTHL represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHL. This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
pub type OLDTHL_R = crate::FieldReader<u32>;
#[doc = "Field `OLDTHL` writer - OLD Low Threshold Value Set and cleared by software. OLDTHL represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHL. This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
pub type OLDTHL_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - OLD Low Threshold Value Set and cleared by software. OLDTHL represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHL. This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
    #[inline(always)]
    pub fn oldthl(&self) -> OLDTHL_R {
        OLDTHL_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - OLD Low Threshold Value Set and cleared by software. OLDTHL represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHL. This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
    #[inline(always)]
    #[must_use]
    pub fn oldthl(&mut self) -> OLDTHL_W<MDF_OLD0THLRrs> {
        OLDTHL_W::new(self, 0)
    }
}
#[doc = "This register is used for the adjustment of the Out-off Limit low threshold.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdf_old0thlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdf_old0thlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDF_OLD0THLRrs;
impl crate::RegisterSpec for MDF_OLD0THLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdf_old0thlr::R`](R) reader structure"]
impl crate::Readable for MDF_OLD0THLRrs {}
#[doc = "`write(|w| ..)` method takes [`mdf_old0thlr::W`](W) writer structure"]
impl crate::Writable for MDF_OLD0THLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDF_OLD0THLR to value 0"]
impl crate::Resettable for MDF_OLD0THLRrs {
    const RESET_VALUE: u32 = 0;
}
