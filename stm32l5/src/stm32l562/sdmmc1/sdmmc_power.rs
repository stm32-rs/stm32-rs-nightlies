///Register `SDMMC_POWER` reader
pub type R = crate::R<SDMMC_POWERrs>;
///Register `SDMMC_POWER` writer
pub type W = crate::W<SDMMC_POWERrs>;
///Field `PWRCTRL` reader - SDMMC state control bits. These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL?11). These bits are used to define the functional state of the SDMMC signals: Any further write will be ignored, PWRCTRL value will keep 11.
pub type PWRCTRL_R = crate::FieldReader;
///Field `PWRCTRL` writer - SDMMC state control bits. These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL?11). These bits are used to define the functional state of the SDMMC signals: Any further write will be ignored, PWRCTRL value will keep 11.
pub type PWRCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VSWITCH` reader - Voltage switch sequence start. This bit is used to start the timing critical section of the voltage switch sequence:
pub type VSWITCH_R = crate::BitReader;
///Field `VSWITCH` writer - Voltage switch sequence start. This bit is used to start the timing critical section of the voltage switch sequence:
pub type VSWITCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSWITCHEN` reader - Voltage switch procedure enable. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:
pub type VSWITCHEN_R = crate::BitReader;
///Field `VSWITCHEN` writer - Voltage switch procedure enable. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:
pub type VSWITCHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIRPOL` reader - Data and command direction signals polarity selection. This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00).
pub type DIRPOL_R = crate::BitReader;
///Field `DIRPOL` writer - Data and command direction signals polarity selection. This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00).
pub type DIRPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - SDMMC state control bits. These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL?11). These bits are used to define the functional state of the SDMMC signals: Any further write will be ignored, PWRCTRL value will keep 11.
    #[inline(always)]
    pub fn pwrctrl(&self) -> PWRCTRL_R {
        PWRCTRL_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Voltage switch sequence start. This bit is used to start the timing critical section of the voltage switch sequence:
    #[inline(always)]
    pub fn vswitch(&self) -> VSWITCH_R {
        VSWITCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Voltage switch procedure enable. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:
    #[inline(always)]
    pub fn vswitchen(&self) -> VSWITCHEN_R {
        VSWITCHEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Data and command direction signals polarity selection. This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00).
    #[inline(always)]
    pub fn dirpol(&self) -> DIRPOL_R {
        DIRPOL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDMMC_POWER")
            .field("pwrctrl", &self.pwrctrl())
            .field("vswitch", &self.vswitch())
            .field("vswitchen", &self.vswitchen())
            .field("dirpol", &self.dirpol())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - SDMMC state control bits. These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL?11). These bits are used to define the functional state of the SDMMC signals: Any further write will be ignored, PWRCTRL value will keep 11.
    #[inline(always)]
    pub fn pwrctrl(&mut self) -> PWRCTRL_W<SDMMC_POWERrs> {
        PWRCTRL_W::new(self, 0)
    }
    ///Bit 2 - Voltage switch sequence start. This bit is used to start the timing critical section of the voltage switch sequence:
    #[inline(always)]
    pub fn vswitch(&mut self) -> VSWITCH_W<SDMMC_POWERrs> {
        VSWITCH_W::new(self, 2)
    }
    ///Bit 3 - Voltage switch procedure enable. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:
    #[inline(always)]
    pub fn vswitchen(&mut self) -> VSWITCHEN_W<SDMMC_POWERrs> {
        VSWITCHEN_W::new(self, 3)
    }
    ///Bit 4 - Data and command direction signals polarity selection. This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00).
    #[inline(always)]
    pub fn dirpol(&mut self) -> DIRPOL_W<SDMMC_POWERrs> {
        DIRPOL_W::new(self, 4)
    }
}
/**SDMMC power control register

You can [`read`](crate::Reg::read) this register and get [`sdmmc_power::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_power::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_POWER)*/
pub struct SDMMC_POWERrs;
impl crate::RegisterSpec for SDMMC_POWERrs {
    type Ux = u32;
}
///`read()` method returns [`sdmmc_power::R`](R) reader structure
impl crate::Readable for SDMMC_POWERrs {}
///`write(|w| ..)` method takes [`sdmmc_power::W`](W) writer structure
impl crate::Writable for SDMMC_POWERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SDMMC_POWER to value 0
impl crate::Resettable for SDMMC_POWERrs {
    const RESET_VALUE: u32 = 0;
}
