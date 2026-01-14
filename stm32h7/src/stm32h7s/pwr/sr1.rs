///Register `SR1` reader
pub type R = crate::R<SR1rs>;
///Field `ACTVOS` reader - VOS currently applied for V<sub>CORE</sub> voltage scaling selection. These bit reflect the last VOS value applied to the PMU.
pub type ACTVOS_R = crate::BitReader;
///Field `ACTVOSRDY` reader - Voltage levels ready bit for currently used ACTVOS and SDHILEVEL This bit is set to 1 by hardware when the voltage regulator and the SMPS step-down converter are both disabled and Bypass mode is selected in PWR control register 2 (PWR_CSR2).
pub type ACTVOSRDY_R = crate::BitReader;
///Field `PVDO` reader - Programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. PLS\[2:0\] bits. bits. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set.
pub type PVDO_R = crate::BitReader;
///Field `AVDO` reader - Analog voltage detector output on VDDA This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the AVDEN bit is set
pub type AVDO_R = crate::BitReader;
impl R {
    ///Bit 0 - VOS currently applied for V<sub>CORE</sub> voltage scaling selection. These bit reflect the last VOS value applied to the PMU.
    #[inline(always)]
    pub fn actvos(&self) -> ACTVOS_R {
        ACTVOS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Voltage levels ready bit for currently used ACTVOS and SDHILEVEL This bit is set to 1 by hardware when the voltage regulator and the SMPS step-down converter are both disabled and Bypass mode is selected in PWR control register 2 (PWR_CSR2).
    #[inline(always)]
    pub fn actvosrdy(&self) -> ACTVOSRDY_R {
        ACTVOSRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. PLS\[2:0\] bits. bits. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set.
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 13 - Analog voltage detector output on VDDA This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the AVDEN bit is set
    #[inline(always)]
    pub fn avdo(&self) -> AVDO_R {
        AVDO_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR1")
            .field("actvos", &self.actvos())
            .field("actvosrdy", &self.actvosrdy())
            .field("pvdo", &self.pvdo())
            .field("avdo", &self.avdo())
            .finish()
    }
}
/**PWR control status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#PWR:SR1)*/
pub struct SR1rs;
impl crate::RegisterSpec for SR1rs {
    type Ux = u32;
}
///`read()` method returns [`sr1::R`](R) reader structure
impl crate::Readable for SR1rs {}
///`reset()` method sets SR1 to value 0
impl crate::Resettable for SR1rs {}
