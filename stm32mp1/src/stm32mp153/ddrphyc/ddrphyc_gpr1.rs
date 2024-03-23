#[doc = "Register `DDRPHYC_GPR1` reader"]
pub type R = crate::R<DDRPHYC_GPR1rs>;
#[doc = "Register `DDRPHYC_GPR1` writer"]
pub type W = crate::W<DDRPHYC_GPR1rs>;
#[doc = "Field `GPR1` reader - GPR1"]
pub type GPR1_R = crate::FieldReader<u32>;
#[doc = "Field `GPR1` writer - GPR1"]
pub type GPR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPR1"]
    #[inline(always)]
    pub fn gpr1(&self) -> GPR1_R {
        GPR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPR1"]
    #[inline(always)]
    #[must_use]
    pub fn gpr1(&mut self) -> GPR1_W<DDRPHYC_GPR1rs> {
        GPR1_W::new(self, 0)
    }
}
#[doc = "DDRPHYC general purpose register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_gpr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_gpr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_GPR1rs;
impl crate::RegisterSpec for DDRPHYC_GPR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_gpr1::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_GPR1rs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_gpr1::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_GPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_GPR1 to value 0"]
impl crate::Resettable for DDRPHYC_GPR1rs {
    const RESET_VALUE: u32 = 0;
}
