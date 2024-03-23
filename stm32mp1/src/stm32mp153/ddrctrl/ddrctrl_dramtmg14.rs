#[doc = "Register `DDRCTRL_DRAMTMG14` reader"]
pub type R = crate::R<DDRCTRL_DRAMTMG14rs>;
#[doc = "Register `DDRCTRL_DRAMTMG14` writer"]
pub type W = crate::W<DDRCTRL_DRAMTMG14rs>;
#[doc = "Field `T_XSR` reader - T_XSR"]
pub type T_XSR_R = crate::FieldReader<u16>;
#[doc = "Field `T_XSR` writer - T_XSR"]
pub type T_XSR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - T_XSR"]
    #[inline(always)]
    pub fn t_xsr(&self) -> T_XSR_R {
        T_XSR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - T_XSR"]
    #[inline(always)]
    #[must_use]
    pub fn t_xsr(&mut self) -> T_XSR_W<DDRCTRL_DRAMTMG14rs> {
        T_XSR_W::new(self, 0)
    }
}
#[doc = "DDRCTRL SDRAM timing register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DRAMTMG14rs;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG14rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_dramtmg14::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG14rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_dramtmg14::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG14rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG14 to value 0xa0"]
impl crate::Resettable for DDRCTRL_DRAMTMG14rs {
    const RESET_VALUE: u32 = 0xa0;
}
