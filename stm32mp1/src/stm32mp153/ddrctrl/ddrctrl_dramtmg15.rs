#[doc = "Register `DDRCTRL_DRAMTMG15` reader"]
pub type R = crate::R<DDRCTRL_DRAMTMG15rs>;
#[doc = "Register `DDRCTRL_DRAMTMG15` writer"]
pub type W = crate::W<DDRCTRL_DRAMTMG15rs>;
#[doc = "Field `T_STAB_X32` reader - T_STAB_X32"]
pub type T_STAB_X32_R = crate::FieldReader;
#[doc = "Field `T_STAB_X32` writer - T_STAB_X32"]
pub type T_STAB_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EN_DFI_LP_T_STAB` reader - EN_DFI_LP_T_STAB"]
pub type EN_DFI_LP_T_STAB_R = crate::BitReader;
#[doc = "Field `EN_DFI_LP_T_STAB` writer - EN_DFI_LP_T_STAB"]
pub type EN_DFI_LP_T_STAB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - T_STAB_X32"]
    #[inline(always)]
    pub fn t_stab_x32(&self) -> T_STAB_X32_R {
        T_STAB_X32_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - EN_DFI_LP_T_STAB"]
    #[inline(always)]
    pub fn en_dfi_lp_t_stab(&self) -> EN_DFI_LP_T_STAB_R {
        EN_DFI_LP_T_STAB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - T_STAB_X32"]
    #[inline(always)]
    #[must_use]
    pub fn t_stab_x32(&mut self) -> T_STAB_X32_W<DDRCTRL_DRAMTMG15rs> {
        T_STAB_X32_W::new(self, 0)
    }
    #[doc = "Bit 31 - EN_DFI_LP_T_STAB"]
    #[inline(always)]
    #[must_use]
    pub fn en_dfi_lp_t_stab(&mut self) -> EN_DFI_LP_T_STAB_W<DDRCTRL_DRAMTMG15rs> {
        EN_DFI_LP_T_STAB_W::new(self, 31)
    }
}
#[doc = "DDRCTRL SDRAM timing register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DRAMTMG15rs;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG15rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_dramtmg15::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG15rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_dramtmg15::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG15rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG15 to value 0"]
impl crate::Resettable for DDRCTRL_DRAMTMG15rs {
    const RESET_VALUE: u32 = 0;
}
