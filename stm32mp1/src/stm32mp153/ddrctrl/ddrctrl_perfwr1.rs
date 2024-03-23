#[doc = "Register `DDRCTRL_PERFWR1` reader"]
pub type R = crate::R<DDRCTRL_PERFWR1rs>;
#[doc = "Register `DDRCTRL_PERFWR1` writer"]
pub type W = crate::W<DDRCTRL_PERFWR1rs>;
#[doc = "Field `W_MAX_STARVE` reader - W_MAX_STARVE"]
pub type W_MAX_STARVE_R = crate::FieldReader<u16>;
#[doc = "Field `W_MAX_STARVE` writer - W_MAX_STARVE"]
pub type W_MAX_STARVE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `W_XACT_RUN_LENGTH` reader - W_XACT_RUN_LENGTH"]
pub type W_XACT_RUN_LENGTH_R = crate::FieldReader;
#[doc = "Field `W_XACT_RUN_LENGTH` writer - W_XACT_RUN_LENGTH"]
pub type W_XACT_RUN_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - W_MAX_STARVE"]
    #[inline(always)]
    pub fn w_max_starve(&self) -> W_MAX_STARVE_R {
        W_MAX_STARVE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - W_XACT_RUN_LENGTH"]
    #[inline(always)]
    pub fn w_xact_run_length(&self) -> W_XACT_RUN_LENGTH_R {
        W_XACT_RUN_LENGTH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - W_MAX_STARVE"]
    #[inline(always)]
    #[must_use]
    pub fn w_max_starve(&mut self) -> W_MAX_STARVE_W<DDRCTRL_PERFWR1rs> {
        W_MAX_STARVE_W::new(self, 0)
    }
    #[doc = "Bits 24:31 - W_XACT_RUN_LENGTH"]
    #[inline(always)]
    #[must_use]
    pub fn w_xact_run_length(&mut self) -> W_XACT_RUN_LENGTH_W<DDRCTRL_PERFWR1rs> {
        W_XACT_RUN_LENGTH_W::new(self, 24)
    }
}
#[doc = "DDRCTRL write CAM register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_perfwr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_perfwr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_PERFWR1rs;
impl crate::RegisterSpec for DDRCTRL_PERFWR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_perfwr1::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_PERFWR1rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_perfwr1::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_PERFWR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_PERFWR1 to value 0x0f00_007f"]
impl crate::Resettable for DDRCTRL_PERFWR1rs {
    const RESET_VALUE: u32 = 0x0f00_007f;
}
