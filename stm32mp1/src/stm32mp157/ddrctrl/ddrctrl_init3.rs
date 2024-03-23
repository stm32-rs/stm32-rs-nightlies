#[doc = "Register `DDRCTRL_INIT3` reader"]
pub type R = crate::R<DDRCTRL_INIT3rs>;
#[doc = "Register `DDRCTRL_INIT3` writer"]
pub type W = crate::W<DDRCTRL_INIT3rs>;
#[doc = "Field `EMR` reader - EMR"]
pub type EMR_R = crate::FieldReader<u16>;
#[doc = "Field `EMR` writer - EMR"]
pub type EMR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MR` reader - MR"]
pub type MR_R = crate::FieldReader<u16>;
#[doc = "Field `MR` writer - MR"]
pub type MR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - EMR"]
    #[inline(always)]
    pub fn emr(&self) -> EMR_R {
        EMR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MR"]
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - EMR"]
    #[inline(always)]
    #[must_use]
    pub fn emr(&mut self) -> EMR_W<DDRCTRL_INIT3rs> {
        EMR_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - MR"]
    #[inline(always)]
    #[must_use]
    pub fn mr(&mut self) -> MR_W<DDRCTRL_INIT3rs> {
        MR_W::new(self, 16)
    }
}
#[doc = "DDRCTRL SDRAM initialization register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_init3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_init3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_INIT3rs;
impl crate::RegisterSpec for DDRCTRL_INIT3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_init3::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_INIT3rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_init3::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_INIT3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_INIT3 to value 0x0510"]
impl crate::Resettable for DDRCTRL_INIT3rs {
    const RESET_VALUE: u32 = 0x0510;
}
