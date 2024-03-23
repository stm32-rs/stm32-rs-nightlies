#[doc = "Register `TARG5_FN_MOD_ISS_BM` reader"]
pub type R = crate::R<TARG5_FN_MOD_ISS_BMrs>;
#[doc = "Register `TARG5_FN_MOD_ISS_BM` writer"]
pub type W = crate::W<TARG5_FN_MOD_ISS_BMrs>;
#[doc = "Field `READ_ISS_OVERRIDE` reader - READ_ISS_OVERRIDE"]
pub type READ_ISS_OVERRIDE_R = crate::BitReader;
#[doc = "Field `READ_ISS_OVERRIDE` writer - READ_ISS_OVERRIDE"]
pub type READ_ISS_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ISS_OVERRIDE` reader - Switch matrix write issuing override for target"]
pub type WRITE_ISS_OVERRIDE_R = crate::BitReader;
#[doc = "Field `WRITE_ISS_OVERRIDE` writer - Switch matrix write issuing override for target"]
pub type WRITE_ISS_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - READ_ISS_OVERRIDE"]
    #[inline(always)]
    pub fn read_iss_override(&self) -> READ_ISS_OVERRIDE_R {
        READ_ISS_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Switch matrix write issuing override for target"]
    #[inline(always)]
    pub fn write_iss_override(&self) -> WRITE_ISS_OVERRIDE_R {
        WRITE_ISS_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - READ_ISS_OVERRIDE"]
    #[inline(always)]
    #[must_use]
    pub fn read_iss_override(&mut self) -> READ_ISS_OVERRIDE_W<TARG5_FN_MOD_ISS_BMrs> {
        READ_ISS_OVERRIDE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Switch matrix write issuing override for target"]
    #[inline(always)]
    #[must_use]
    pub fn write_iss_override(&mut self) -> WRITE_ISS_OVERRIDE_W<TARG5_FN_MOD_ISS_BMrs> {
        WRITE_ISS_OVERRIDE_W::new(self, 1)
    }
}
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`targ5_fn_mod_iss_bm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`targ5_fn_mod_iss_bm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARG5_FN_MOD_ISS_BMrs;
impl crate::RegisterSpec for TARG5_FN_MOD_ISS_BMrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`targ5_fn_mod_iss_bm::R`](R) reader structure"]
impl crate::Readable for TARG5_FN_MOD_ISS_BMrs {}
#[doc = "`write(|w| ..)` method takes [`targ5_fn_mod_iss_bm::W`](W) writer structure"]
impl crate::Writable for TARG5_FN_MOD_ISS_BMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TARG5_FN_MOD_ISS_BM to value 0x04"]
impl crate::Resettable for TARG5_FN_MOD_ISS_BMrs {
    const RESET_VALUE: u32 = 0x04;
}
