#[doc = "Register `DDRCTRL_INIT4` reader"]
pub type R = crate::R<DDRCTRL_INIT4rs>;
#[doc = "Register `DDRCTRL_INIT4` writer"]
pub type W = crate::W<DDRCTRL_INIT4rs>;
#[doc = "Field `EMR3` reader - EMR3"]
pub type EMR3_R = crate::FieldReader<u16>;
#[doc = "Field `EMR3` writer - EMR3"]
pub type EMR3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EMR2` reader - EMR2"]
pub type EMR2_R = crate::FieldReader<u16>;
#[doc = "Field `EMR2` writer - EMR2"]
pub type EMR2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - EMR3"]
    #[inline(always)]
    pub fn emr3(&self) -> EMR3_R {
        EMR3_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - EMR2"]
    #[inline(always)]
    pub fn emr2(&self) -> EMR2_R {
        EMR2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - EMR3"]
    #[inline(always)]
    #[must_use]
    pub fn emr3(&mut self) -> EMR3_W<DDRCTRL_INIT4rs> {
        EMR3_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - EMR2"]
    #[inline(always)]
    #[must_use]
    pub fn emr2(&mut self) -> EMR2_W<DDRCTRL_INIT4rs> {
        EMR2_W::new(self, 16)
    }
}
#[doc = "DDRCTRL SDRAM initialization register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_init4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_init4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_INIT4rs;
impl crate::RegisterSpec for DDRCTRL_INIT4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_init4::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_INIT4rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_init4::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_INIT4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_INIT4 to value 0"]
impl crate::Resettable for DDRCTRL_INIT4rs {
    const RESET_VALUE: u32 = 0;
}
