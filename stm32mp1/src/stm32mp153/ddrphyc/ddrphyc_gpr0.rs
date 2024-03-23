#[doc = "Register `DDRPHYC_GPR0` reader"]
pub type R = crate::R<DDRPHYC_GPR0rs>;
#[doc = "Register `DDRPHYC_GPR0` writer"]
pub type W = crate::W<DDRPHYC_GPR0rs>;
#[doc = "Field `GPR0` reader - GPR0"]
pub type GPR0_R = crate::FieldReader<u32>;
#[doc = "Field `GPR0` writer - GPR0"]
pub type GPR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPR0"]
    #[inline(always)]
    pub fn gpr0(&self) -> GPR0_R {
        GPR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPR0"]
    #[inline(always)]
    #[must_use]
    pub fn gpr0(&mut self) -> GPR0_W<DDRPHYC_GPR0rs> {
        GPR0_W::new(self, 0)
    }
}
#[doc = "DDRPHYC general purpose register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_gpr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_gpr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_GPR0rs;
impl crate::RegisterSpec for DDRPHYC_GPR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_gpr0::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_GPR0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_gpr0::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_GPR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_GPR0 to value 0"]
impl crate::Resettable for DDRPHYC_GPR0rs {
    const RESET_VALUE: u32 = 0;
}
