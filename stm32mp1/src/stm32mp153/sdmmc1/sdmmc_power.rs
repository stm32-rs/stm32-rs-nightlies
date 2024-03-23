#[doc = "Register `SDMMC_POWER` reader"]
pub type R = crate::R<SDMMC_POWERrs>;
#[doc = "Register `SDMMC_POWER` writer"]
pub type W = crate::W<SDMMC_POWERrs>;
#[doc = "Field `PWRCTRL` reader - PWRCTRL"]
pub type PWRCTRL_R = crate::FieldReader;
#[doc = "Field `PWRCTRL` writer - PWRCTRL"]
pub type PWRCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VSWITCH` reader - VSWITCH"]
pub type VSWITCH_R = crate::BitReader;
#[doc = "Field `VSWITCH` writer - VSWITCH"]
pub type VSWITCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSWITCHEN` reader - VSWITCHEN"]
pub type VSWITCHEN_R = crate::BitReader;
#[doc = "Field `VSWITCHEN` writer - VSWITCHEN"]
pub type VSWITCHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRPOL` reader - DIRPOL"]
pub type DIRPOL_R = crate::BitReader;
#[doc = "Field `DIRPOL` writer - DIRPOL"]
pub type DIRPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - PWRCTRL"]
    #[inline(always)]
    pub fn pwrctrl(&self) -> PWRCTRL_R {
        PWRCTRL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - VSWITCH"]
    #[inline(always)]
    pub fn vswitch(&self) -> VSWITCH_R {
        VSWITCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSWITCHEN"]
    #[inline(always)]
    pub fn vswitchen(&self) -> VSWITCHEN_R {
        VSWITCHEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIRPOL"]
    #[inline(always)]
    pub fn dirpol(&self) -> DIRPOL_R {
        DIRPOL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWRCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn pwrctrl(&mut self) -> PWRCTRL_W<SDMMC_POWERrs> {
        PWRCTRL_W::new(self, 0)
    }
    #[doc = "Bit 2 - VSWITCH"]
    #[inline(always)]
    #[must_use]
    pub fn vswitch(&mut self) -> VSWITCH_W<SDMMC_POWERrs> {
        VSWITCH_W::new(self, 2)
    }
    #[doc = "Bit 3 - VSWITCHEN"]
    #[inline(always)]
    #[must_use]
    pub fn vswitchen(&mut self) -> VSWITCHEN_W<SDMMC_POWERrs> {
        VSWITCHEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - DIRPOL"]
    #[inline(always)]
    #[must_use]
    pub fn dirpol(&mut self) -> DIRPOL_W<SDMMC_POWERrs> {
        DIRPOL_W::new(self, 4)
    }
}
#[doc = "SDMMC power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_power::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_power::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_POWERrs;
impl crate::RegisterSpec for SDMMC_POWERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_power::R`](R) reader structure"]
impl crate::Readable for SDMMC_POWERrs {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_power::W`](W) writer structure"]
impl crate::Writable for SDMMC_POWERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_POWER to value 0"]
impl crate::Resettable for SDMMC_POWERrs {
    const RESET_VALUE: u32 = 0;
}
