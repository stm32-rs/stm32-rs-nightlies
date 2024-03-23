#[doc = "Register `DDRCTRL_PERFHPR1` reader"]
pub type R = crate::R<DDRCTRL_PERFHPR1rs>;
#[doc = "Register `DDRCTRL_PERFHPR1` writer"]
pub type W = crate::W<DDRCTRL_PERFHPR1rs>;
#[doc = "Field `HPR_MAX_STARVE` reader - HPR_MAX_STARVE"]
pub type HPR_MAX_STARVE_R = crate::FieldReader<u16>;
#[doc = "Field `HPR_MAX_STARVE` writer - HPR_MAX_STARVE"]
pub type HPR_MAX_STARVE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HPR_XACT_RUN_LENGTH` reader - HPR_XACT_RUN_LENGTH"]
pub type HPR_XACT_RUN_LENGTH_R = crate::FieldReader;
#[doc = "Field `HPR_XACT_RUN_LENGTH` writer - HPR_XACT_RUN_LENGTH"]
pub type HPR_XACT_RUN_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - HPR_MAX_STARVE"]
    #[inline(always)]
    pub fn hpr_max_starve(&self) -> HPR_MAX_STARVE_R {
        HPR_MAX_STARVE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - HPR_XACT_RUN_LENGTH"]
    #[inline(always)]
    pub fn hpr_xact_run_length(&self) -> HPR_XACT_RUN_LENGTH_R {
        HPR_XACT_RUN_LENGTH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - HPR_MAX_STARVE"]
    #[inline(always)]
    #[must_use]
    pub fn hpr_max_starve(&mut self) -> HPR_MAX_STARVE_W<DDRCTRL_PERFHPR1rs> {
        HPR_MAX_STARVE_W::new(self, 0)
    }
    #[doc = "Bits 24:31 - HPR_XACT_RUN_LENGTH"]
    #[inline(always)]
    #[must_use]
    pub fn hpr_xact_run_length(&mut self) -> HPR_XACT_RUN_LENGTH_W<DDRCTRL_PERFHPR1rs> {
        HPR_XACT_RUN_LENGTH_W::new(self, 24)
    }
}
#[doc = "DDRCTRL high priority read CAM register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_perfhpr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_perfhpr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_PERFHPR1rs;
impl crate::RegisterSpec for DDRCTRL_PERFHPR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_perfhpr1::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_PERFHPR1rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_perfhpr1::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_PERFHPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_PERFHPR1 to value 0x0f00_0001"]
impl crate::Resettable for DDRCTRL_PERFHPR1rs {
    const RESET_VALUE: u32 = 0x0f00_0001;
}
