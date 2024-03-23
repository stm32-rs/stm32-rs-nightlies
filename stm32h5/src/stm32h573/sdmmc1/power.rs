#[doc = "Register `POWER` reader"]
pub type R = crate::R<POWERrs>;
#[doc = "Register `POWER` writer"]
pub type W = crate::W<POWERrs>;
#[doc = "Field `PWRCTRL` reader - SDMMC state control bits These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL ≠ 11). These bits are used to define the functional state of the SDMMC signals: stopped, SDMMC_D\\[7:0\\], SDMMC_CMD and SDMMC_CK are driven high. Any further write is ignored, PWRCTRL value keeps 11."]
pub type PWRCTRL_R = crate::FieldReader;
#[doc = "Field `PWRCTRL` writer - SDMMC state control bits These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL ≠ 11). These bits are used to define the functional state of the SDMMC signals: stopped, SDMMC_D\\[7:0\\], SDMMC_CMD and SDMMC_CK are driven high. Any further write is ignored, PWRCTRL value keeps 11."]
pub type PWRCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VSWITCH` reader - Voltage switch sequence start This bit is used to start the timing critical section of the voltage switch sequence:"]
pub type VSWITCH_R = crate::BitReader;
#[doc = "Field `VSWITCH` writer - Voltage switch sequence start This bit is used to start the timing critical section of the voltage switch sequence:"]
pub type VSWITCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSWITCHEN` reader - Voltage switch procedure enable This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:"]
pub type VSWITCHEN_R = crate::BitReader;
#[doc = "Field `VSWITCHEN` writer - Voltage switch procedure enable This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:"]
pub type VSWITCHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRPOL` reader - Data and command direction signals polarity selection This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00)."]
pub type DIRPOL_R = crate::BitReader;
#[doc = "Field `DIRPOL` writer - Data and command direction signals polarity selection This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00)."]
pub type DIRPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SDMMC state control bits These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL ≠ 11). These bits are used to define the functional state of the SDMMC signals: stopped, SDMMC_D\\[7:0\\], SDMMC_CMD and SDMMC_CK are driven high. Any further write is ignored, PWRCTRL value keeps 11."]
    #[inline(always)]
    pub fn pwrctrl(&self) -> PWRCTRL_R {
        PWRCTRL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Voltage switch sequence start This bit is used to start the timing critical section of the voltage switch sequence:"]
    #[inline(always)]
    pub fn vswitch(&self) -> VSWITCH_R {
        VSWITCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage switch procedure enable This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:"]
    #[inline(always)]
    pub fn vswitchen(&self) -> VSWITCHEN_R {
        VSWITCHEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data and command direction signals polarity selection This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00)."]
    #[inline(always)]
    pub fn dirpol(&self) -> DIRPOL_R {
        DIRPOL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SDMMC state control bits These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL ≠ 11). These bits are used to define the functional state of the SDMMC signals: stopped, SDMMC_D\\[7:0\\], SDMMC_CMD and SDMMC_CK are driven high. Any further write is ignored, PWRCTRL value keeps 11."]
    #[inline(always)]
    #[must_use]
    pub fn pwrctrl(&mut self) -> PWRCTRL_W<POWERrs> {
        PWRCTRL_W::new(self, 0)
    }
    #[doc = "Bit 2 - Voltage switch sequence start This bit is used to start the timing critical section of the voltage switch sequence:"]
    #[inline(always)]
    #[must_use]
    pub fn vswitch(&mut self) -> VSWITCH_W<POWERrs> {
        VSWITCH_W::new(self, 2)
    }
    #[doc = "Bit 3 - Voltage switch procedure enable This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:"]
    #[inline(always)]
    #[must_use]
    pub fn vswitchen(&mut self) -> VSWITCHEN_W<POWERrs> {
        VSWITCHEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Data and command direction signals polarity selection This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00)."]
    #[inline(always)]
    #[must_use]
    pub fn dirpol(&mut self) -> DIRPOL_W<POWERrs> {
        DIRPOL_W::new(self, 4)
    }
}
#[doc = "SDMMC_POWER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWERrs;
impl crate::RegisterSpec for POWERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power::R`](R) reader structure"]
impl crate::Readable for POWERrs {}
#[doc = "`write(|w| ..)` method takes [`power::W`](W) writer structure"]
impl crate::Writable for POWERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER to value 0"]
impl crate::Resettable for POWERrs {
    const RESET_VALUE: u32 = 0;
}
