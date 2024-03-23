#[doc = "Register `DDRCTRL_DRAMTMG0` reader"]
pub type R = crate::R<DDRCTRL_DRAMTMG0rs>;
#[doc = "Register `DDRCTRL_DRAMTMG0` writer"]
pub type W = crate::W<DDRCTRL_DRAMTMG0rs>;
#[doc = "Field `T_RAS_MIN` reader - T_RAS_MIN"]
pub type T_RAS_MIN_R = crate::FieldReader;
#[doc = "Field `T_RAS_MIN` writer - T_RAS_MIN"]
pub type T_RAS_MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `T_RAS_MAX` reader - T_RAS_MAX"]
pub type T_RAS_MAX_R = crate::FieldReader;
#[doc = "Field `T_RAS_MAX` writer - T_RAS_MAX"]
pub type T_RAS_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `T_FAW` reader - T_FAW"]
pub type T_FAW_R = crate::FieldReader;
#[doc = "Field `T_FAW` writer - T_FAW"]
pub type T_FAW_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WR2PRE` reader - WR2PRE"]
pub type WR2PRE_R = crate::FieldReader;
#[doc = "Field `WR2PRE` writer - WR2PRE"]
pub type WR2PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:5 - T_RAS_MIN"]
    #[inline(always)]
    pub fn t_ras_min(&self) -> T_RAS_MIN_R {
        T_RAS_MIN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:14 - T_RAS_MAX"]
    #[inline(always)]
    pub fn t_ras_max(&self) -> T_RAS_MAX_R {
        T_RAS_MAX_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:21 - T_FAW"]
    #[inline(always)]
    pub fn t_faw(&self) -> T_FAW_R {
        T_FAW_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:30 - WR2PRE"]
    #[inline(always)]
    pub fn wr2pre(&self) -> WR2PRE_R {
        WR2PRE_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - T_RAS_MIN"]
    #[inline(always)]
    #[must_use]
    pub fn t_ras_min(&mut self) -> T_RAS_MIN_W<DDRCTRL_DRAMTMG0rs> {
        T_RAS_MIN_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - T_RAS_MAX"]
    #[inline(always)]
    #[must_use]
    pub fn t_ras_max(&mut self) -> T_RAS_MAX_W<DDRCTRL_DRAMTMG0rs> {
        T_RAS_MAX_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - T_FAW"]
    #[inline(always)]
    #[must_use]
    pub fn t_faw(&mut self) -> T_FAW_W<DDRCTRL_DRAMTMG0rs> {
        T_FAW_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - WR2PRE"]
    #[inline(always)]
    #[must_use]
    pub fn wr2pre(&mut self) -> WR2PRE_W<DDRCTRL_DRAMTMG0rs> {
        WR2PRE_W::new(self, 24)
    }
}
#[doc = "DDRCTRL SDRAM timing register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DRAMTMG0rs;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_dramtmg0::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_dramtmg0::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG0 to value 0x0f10_1b0f"]
impl crate::Resettable for DDRCTRL_DRAMTMG0rs {
    const RESET_VALUE: u32 = 0x0f10_1b0f;
}
