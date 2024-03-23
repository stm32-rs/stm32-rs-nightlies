#[doc = "Register `DDRCTRL_PERFLPR1` reader"]
pub type R = crate::R<DDRCTRL_PERFLPR1rs>;
#[doc = "Register `DDRCTRL_PERFLPR1` writer"]
pub type W = crate::W<DDRCTRL_PERFLPR1rs>;
#[doc = "Field `LPR_MAX_STARVE` reader - LPR_MAX_STARVE"]
pub type LPR_MAX_STARVE_R = crate::FieldReader<u16>;
#[doc = "Field `LPR_MAX_STARVE` writer - LPR_MAX_STARVE"]
pub type LPR_MAX_STARVE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LPR_XACT_RUN_LENGTH` reader - LPR_XACT_RUN_LENGTH"]
pub type LPR_XACT_RUN_LENGTH_R = crate::FieldReader;
#[doc = "Field `LPR_XACT_RUN_LENGTH` writer - LPR_XACT_RUN_LENGTH"]
pub type LPR_XACT_RUN_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - LPR_MAX_STARVE"]
    #[inline(always)]
    pub fn lpr_max_starve(&self) -> LPR_MAX_STARVE_R {
        LPR_MAX_STARVE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - LPR_XACT_RUN_LENGTH"]
    #[inline(always)]
    pub fn lpr_xact_run_length(&self) -> LPR_XACT_RUN_LENGTH_R {
        LPR_XACT_RUN_LENGTH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - LPR_MAX_STARVE"]
    #[inline(always)]
    #[must_use]
    pub fn lpr_max_starve(&mut self) -> LPR_MAX_STARVE_W<DDRCTRL_PERFLPR1rs> {
        LPR_MAX_STARVE_W::new(self, 0)
    }
    #[doc = "Bits 24:31 - LPR_XACT_RUN_LENGTH"]
    #[inline(always)]
    #[must_use]
    pub fn lpr_xact_run_length(&mut self) -> LPR_XACT_RUN_LENGTH_W<DDRCTRL_PERFLPR1rs> {
        LPR_XACT_RUN_LENGTH_W::new(self, 24)
    }
}
#[doc = "DDRCTRL low priority read CAM register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_perflpr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_perflpr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_PERFLPR1rs;
impl crate::RegisterSpec for DDRCTRL_PERFLPR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_perflpr1::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_PERFLPR1rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_perflpr1::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_PERFLPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_PERFLPR1 to value 0x0f00_007f"]
impl crate::Resettable for DDRCTRL_PERFLPR1rs {
    const RESET_VALUE: u32 = 0x0f00_007f;
}
