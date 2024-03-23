#[doc = "Register `DDRCTRL_DRAMTMG7` reader"]
pub type R = crate::R<DDRCTRL_DRAMTMG7rs>;
#[doc = "Register `DDRCTRL_DRAMTMG7` writer"]
pub type W = crate::W<DDRCTRL_DRAMTMG7rs>;
#[doc = "Field `T_CKPDX` reader - T_CKPDX"]
pub type T_CKPDX_R = crate::FieldReader;
#[doc = "Field `T_CKPDX` writer - T_CKPDX"]
pub type T_CKPDX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `T_CKPDE` reader - T_CKPDE"]
pub type T_CKPDE_R = crate::FieldReader;
#[doc = "Field `T_CKPDE` writer - T_CKPDE"]
pub type T_CKPDE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - T_CKPDX"]
    #[inline(always)]
    pub fn t_ckpdx(&self) -> T_CKPDX_R {
        T_CKPDX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - T_CKPDE"]
    #[inline(always)]
    pub fn t_ckpde(&self) -> T_CKPDE_R {
        T_CKPDE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - T_CKPDX"]
    #[inline(always)]
    #[must_use]
    pub fn t_ckpdx(&mut self) -> T_CKPDX_W<DDRCTRL_DRAMTMG7rs> {
        T_CKPDX_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - T_CKPDE"]
    #[inline(always)]
    #[must_use]
    pub fn t_ckpde(&mut self) -> T_CKPDE_W<DDRCTRL_DRAMTMG7rs> {
        T_CKPDE_W::new(self, 8)
    }
}
#[doc = "DDRCTRL SDRAM timing register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DRAMTMG7rs;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_dramtmg7::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG7rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_dramtmg7::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG7 to value 0x0202"]
impl crate::Resettable for DDRCTRL_DRAMTMG7rs {
    const RESET_VALUE: u32 = 0x0202;
}
