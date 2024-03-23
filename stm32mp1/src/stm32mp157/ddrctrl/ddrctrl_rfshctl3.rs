#[doc = "Register `DDRCTRL_RFSHCTL3` reader"]
pub type R = crate::R<DDRCTRL_RFSHCTL3rs>;
#[doc = "Register `DDRCTRL_RFSHCTL3` writer"]
pub type W = crate::W<DDRCTRL_RFSHCTL3rs>;
#[doc = "Field `DIS_AUTO_REFRESH` reader - DIS_AUTO_REFRESH"]
pub type DIS_AUTO_REFRESH_R = crate::BitReader;
#[doc = "Field `DIS_AUTO_REFRESH` writer - DIS_AUTO_REFRESH"]
pub type DIS_AUTO_REFRESH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFRESH_UPDATE_LEVEL` reader - REFRESH_UPDATE_LEVEL"]
pub type REFRESH_UPDATE_LEVEL_R = crate::BitReader;
#[doc = "Field `REFRESH_UPDATE_LEVEL` writer - REFRESH_UPDATE_LEVEL"]
pub type REFRESH_UPDATE_LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DIS_AUTO_REFRESH"]
    #[inline(always)]
    pub fn dis_auto_refresh(&self) -> DIS_AUTO_REFRESH_R {
        DIS_AUTO_REFRESH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REFRESH_UPDATE_LEVEL"]
    #[inline(always)]
    pub fn refresh_update_level(&self) -> REFRESH_UPDATE_LEVEL_R {
        REFRESH_UPDATE_LEVEL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIS_AUTO_REFRESH"]
    #[inline(always)]
    #[must_use]
    pub fn dis_auto_refresh(&mut self) -> DIS_AUTO_REFRESH_W<DDRCTRL_RFSHCTL3rs> {
        DIS_AUTO_REFRESH_W::new(self, 0)
    }
    #[doc = "Bit 1 - REFRESH_UPDATE_LEVEL"]
    #[inline(always)]
    #[must_use]
    pub fn refresh_update_level(&mut self) -> REFRESH_UPDATE_LEVEL_W<DDRCTRL_RFSHCTL3rs> {
        REFRESH_UPDATE_LEVEL_W::new(self, 1)
    }
}
#[doc = "DDRCTRL refresh control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_rfshctl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_rfshctl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_RFSHCTL3rs;
impl crate::RegisterSpec for DDRCTRL_RFSHCTL3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_rfshctl3::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_RFSHCTL3rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_rfshctl3::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_RFSHCTL3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_RFSHCTL3 to value 0"]
impl crate::Resettable for DDRCTRL_RFSHCTL3rs {
    const RESET_VALUE: u32 = 0;
}
