#[doc = "Register `DDRCTRL_DRAMTMG5` reader"]
pub type R = crate::R<DDRCTRL_DRAMTMG5rs>;
#[doc = "Register `DDRCTRL_DRAMTMG5` writer"]
pub type W = crate::W<DDRCTRL_DRAMTMG5rs>;
#[doc = "Field `T_CKE` reader - T_CKE"]
pub type T_CKE_R = crate::FieldReader;
#[doc = "Field `T_CKE` writer - T_CKE"]
pub type T_CKE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `T_CKESR` reader - T_CKESR"]
pub type T_CKESR_R = crate::FieldReader;
#[doc = "Field `T_CKESR` writer - T_CKESR"]
pub type T_CKESR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `T_CKSRE` reader - T_CKSRE"]
pub type T_CKSRE_R = crate::FieldReader;
#[doc = "Field `T_CKSRE` writer - T_CKSRE"]
pub type T_CKSRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `T_CKSRX` reader - T_CKSRX"]
pub type T_CKSRX_R = crate::FieldReader;
#[doc = "Field `T_CKSRX` writer - T_CKSRX"]
pub type T_CKSRX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - T_CKE"]
    #[inline(always)]
    pub fn t_cke(&self) -> T_CKE_R {
        T_CKE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - T_CKESR"]
    #[inline(always)]
    pub fn t_ckesr(&self) -> T_CKESR_R {
        T_CKESR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - T_CKSRE"]
    #[inline(always)]
    pub fn t_cksre(&self) -> T_CKSRE_R {
        T_CKSRE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - T_CKSRX"]
    #[inline(always)]
    pub fn t_cksrx(&self) -> T_CKSRX_R {
        T_CKSRX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - T_CKE"]
    #[inline(always)]
    #[must_use]
    pub fn t_cke(&mut self) -> T_CKE_W<DDRCTRL_DRAMTMG5rs> {
        T_CKE_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - T_CKESR"]
    #[inline(always)]
    #[must_use]
    pub fn t_ckesr(&mut self) -> T_CKESR_W<DDRCTRL_DRAMTMG5rs> {
        T_CKESR_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - T_CKSRE"]
    #[inline(always)]
    #[must_use]
    pub fn t_cksre(&mut self) -> T_CKSRE_W<DDRCTRL_DRAMTMG5rs> {
        T_CKSRE_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - T_CKSRX"]
    #[inline(always)]
    #[must_use]
    pub fn t_cksrx(&mut self) -> T_CKSRX_W<DDRCTRL_DRAMTMG5rs> {
        T_CKSRX_W::new(self, 24)
    }
}
#[doc = "DDRCTRL SDRAM timing register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DRAMTMG5rs;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_dramtmg5::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG5rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_dramtmg5::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG5 to value 0x0505_0403"]
impl crate::Resettable for DDRCTRL_DRAMTMG5rs {
    const RESET_VALUE: u32 = 0x0505_0403;
}
