#[doc = "Register `DDRCTRL_DRAMTMG4` reader"]
pub type R = crate::R<DDRCTRL_DRAMTMG4rs>;
#[doc = "Register `DDRCTRL_DRAMTMG4` writer"]
pub type W = crate::W<DDRCTRL_DRAMTMG4rs>;
#[doc = "Field `T_RP` reader - T_RP"]
pub type T_RP_R = crate::FieldReader;
#[doc = "Field `T_RP` writer - T_RP"]
pub type T_RP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `T_RRD` reader - T_RRD"]
pub type T_RRD_R = crate::FieldReader;
#[doc = "Field `T_RRD` writer - T_RRD"]
pub type T_RRD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `T_CCD` reader - T_CCD"]
pub type T_CCD_R = crate::FieldReader;
#[doc = "Field `T_CCD` writer - T_CCD"]
pub type T_CCD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `T_RCD` reader - T_RCD"]
pub type T_RCD_R = crate::FieldReader;
#[doc = "Field `T_RCD` writer - T_RCD"]
pub type T_RCD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - T_RP"]
    #[inline(always)]
    pub fn t_rp(&self) -> T_RP_R {
        T_RP_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - T_RRD"]
    #[inline(always)]
    pub fn t_rrd(&self) -> T_RRD_R {
        T_RRD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - T_CCD"]
    #[inline(always)]
    pub fn t_ccd(&self) -> T_CCD_R {
        T_CCD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - T_RCD"]
    #[inline(always)]
    pub fn t_rcd(&self) -> T_RCD_R {
        T_RCD_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - T_RP"]
    #[inline(always)]
    #[must_use]
    pub fn t_rp(&mut self) -> T_RP_W<DDRCTRL_DRAMTMG4rs> {
        T_RP_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - T_RRD"]
    #[inline(always)]
    #[must_use]
    pub fn t_rrd(&mut self) -> T_RRD_W<DDRCTRL_DRAMTMG4rs> {
        T_RRD_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - T_CCD"]
    #[inline(always)]
    #[must_use]
    pub fn t_ccd(&mut self) -> T_CCD_W<DDRCTRL_DRAMTMG4rs> {
        T_CCD_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - T_RCD"]
    #[inline(always)]
    #[must_use]
    pub fn t_rcd(&mut self) -> T_RCD_W<DDRCTRL_DRAMTMG4rs> {
        T_RCD_W::new(self, 24)
    }
}
#[doc = "DDRCTRL SDRAM timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DRAMTMG4rs;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_dramtmg4::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG4rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_dramtmg4::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG4 to value 0x0504_0405"]
impl crate::Resettable for DDRCTRL_DRAMTMG4rs {
    const RESET_VALUE: u32 = 0x0504_0405;
}
