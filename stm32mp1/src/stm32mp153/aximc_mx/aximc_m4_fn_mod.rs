#[doc = "Register `AXIMC_M4_FN_MOD` reader"]
pub type R = crate::R<AXIMC_M4_FN_MODrs>;
#[doc = "Register `AXIMC_M4_FN_MOD` writer"]
pub type W = crate::W<AXIMC_M4_FN_MODrs>;
#[doc = "Field `READ_ISS_OVERRIDE` reader - READ_ISS_OVERRIDE"]
pub type READ_ISS_OVERRIDE_R = crate::BitReader;
#[doc = "Field `READ_ISS_OVERRIDE` writer - READ_ISS_OVERRIDE"]
pub type READ_ISS_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ISS_OVERRIDE` reader - WRITE_ISS_OVERRIDE"]
pub type WRITE_ISS_OVERRIDE_R = crate::BitReader;
#[doc = "Field `WRITE_ISS_OVERRIDE` writer - WRITE_ISS_OVERRIDE"]
pub type WRITE_ISS_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - READ_ISS_OVERRIDE"]
    #[inline(always)]
    pub fn read_iss_override(&self) -> READ_ISS_OVERRIDE_R {
        READ_ISS_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WRITE_ISS_OVERRIDE"]
    #[inline(always)]
    pub fn write_iss_override(&self) -> WRITE_ISS_OVERRIDE_R {
        WRITE_ISS_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - READ_ISS_OVERRIDE"]
    #[inline(always)]
    #[must_use]
    pub fn read_iss_override(&mut self) -> READ_ISS_OVERRIDE_W<AXIMC_M4_FN_MODrs> {
        READ_ISS_OVERRIDE_W::new(self, 0)
    }
    #[doc = "Bit 1 - WRITE_ISS_OVERRIDE"]
    #[inline(always)]
    #[must_use]
    pub fn write_iss_override(&mut self) -> WRITE_ISS_OVERRIDE_W<AXIMC_M4_FN_MODrs> {
        WRITE_ISS_OVERRIDE_W::new(self, 1)
    }
}
#[doc = "AXIMC master 4 packing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m4_fn_mod::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m4_fn_mod::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXIMC_M4_FN_MODrs;
impl crate::RegisterSpec for AXIMC_M4_FN_MODrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aximc_m4_fn_mod::R`](R) reader structure"]
impl crate::Readable for AXIMC_M4_FN_MODrs {}
#[doc = "`write(|w| ..)` method takes [`aximc_m4_fn_mod::W`](W) writer structure"]
impl crate::Writable for AXIMC_M4_FN_MODrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AXIMC_M4_FN_MOD to value 0"]
impl crate::Resettable for AXIMC_M4_FN_MODrs {
    const RESET_VALUE: u32 = 0;
}
