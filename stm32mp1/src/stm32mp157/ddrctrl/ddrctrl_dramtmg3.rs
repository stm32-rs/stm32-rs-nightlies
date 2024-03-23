#[doc = "Register `DDRCTRL_DRAMTMG3` reader"]
pub type R = crate::R<DDRCTRL_DRAMTMG3rs>;
#[doc = "Register `DDRCTRL_DRAMTMG3` writer"]
pub type W = crate::W<DDRCTRL_DRAMTMG3rs>;
#[doc = "Field `T_MOD` reader - T_MOD"]
pub type T_MOD_R = crate::FieldReader<u16>;
#[doc = "Field `T_MOD` writer - T_MOD"]
pub type T_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `T_MRD` reader - T_MRD"]
pub type T_MRD_R = crate::FieldReader;
#[doc = "Field `T_MRD` writer - T_MRD"]
pub type T_MRD_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `T_MRW` reader - T_MRW"]
pub type T_MRW_R = crate::FieldReader<u16>;
#[doc = "Field `T_MRW` writer - T_MRW"]
pub type T_MRW_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - T_MOD"]
    #[inline(always)]
    pub fn t_mod(&self) -> T_MOD_R {
        T_MOD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:17 - T_MRD"]
    #[inline(always)]
    pub fn t_mrd(&self) -> T_MRD_R {
        T_MRD_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 20:29 - T_MRW"]
    #[inline(always)]
    pub fn t_mrw(&self) -> T_MRW_R {
        T_MRW_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - T_MOD"]
    #[inline(always)]
    #[must_use]
    pub fn t_mod(&mut self) -> T_MOD_W<DDRCTRL_DRAMTMG3rs> {
        T_MOD_W::new(self, 0)
    }
    #[doc = "Bits 12:17 - T_MRD"]
    #[inline(always)]
    #[must_use]
    pub fn t_mrd(&mut self) -> T_MRD_W<DDRCTRL_DRAMTMG3rs> {
        T_MRD_W::new(self, 12)
    }
    #[doc = "Bits 20:29 - T_MRW"]
    #[inline(always)]
    #[must_use]
    pub fn t_mrw(&mut self) -> T_MRW_W<DDRCTRL_DRAMTMG3rs> {
        T_MRW_W::new(self, 20)
    }
}
#[doc = "DDRCTRL SDRAM timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DRAMTMG3rs;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_dramtmg3::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG3rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_dramtmg3::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG3 to value 0x0050_400c"]
impl crate::Resettable for DDRCTRL_DRAMTMG3rs {
    const RESET_VALUE: u32 = 0x0050_400c;
}
