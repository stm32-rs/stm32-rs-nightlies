///Register `SR2` reader
pub type R = crate::R<SR2rs>;
///Field `REGLPS` reader - Low-power regulator started This bit provides the information whether the low-power regulator is ready after a power-on reset or a Standby/Shutdown. If the Standby mode is entered while REGLPS bit is still cleared, the wakeup from Standby mode time may be increased.
pub type REGLPS_R = crate::BitReader;
///Field `REGLPF` reader - Low-power regulator flag This bit is set by hardware when the MCU is in Low-power run mode. When the MCU exits the Low-power run mode, this bit remains at 1 until the regulator is ready in main mode. A polling on this bit must be done before increasing the product frequency. This bit is cleared by hardware when the regulator is ready.
pub type REGLPF_R = crate::BitReader;
///Field `VOSF` reader - Voltage scaling flag A delay is required for the internal regulator to be ready after the voltage scaling has been changed. VOSF indicates that the regulator reached the voltage level defined with VOS bits of the PWR_CR1 register.
pub type VOSF_R = crate::BitReader;
///Field `PVDO` reader - Programmable voltage detector output
pub type PVDO_R = crate::BitReader;
///Field `PVMO1` reader - Peripheral voltage monitoring output: V<sub>DDA</sub> vs. 1.62 V Note: PVMO1 is cleared when PVM1 is disabled (PVME = 0). After enabling PVM1, the PVM1 output is valid after the PVM1 wakeup time.
pub type PVMO1_R = crate::BitReader;
///Field `PVMO2` reader - Peripheral voltage monitoring output: V<sub>DDA</sub> vs. 1.8 V Note: PVMO2 is cleared when PVM2 is disabled (PVME = 0). After enabling PVM2, the PVM2 output is valid after the PVM2 wakeup time.
pub type PVMO2_R = crate::BitReader;
impl R {
    ///Bit 8 - Low-power regulator started This bit provides the information whether the low-power regulator is ready after a power-on reset or a Standby/Shutdown. If the Standby mode is entered while REGLPS bit is still cleared, the wakeup from Standby mode time may be increased.
    #[inline(always)]
    pub fn reglps(&self) -> REGLPS_R {
        REGLPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Low-power regulator flag This bit is set by hardware when the MCU is in Low-power run mode. When the MCU exits the Low-power run mode, this bit remains at 1 until the regulator is ready in main mode. A polling on this bit must be done before increasing the product frequency. This bit is cleared by hardware when the regulator is ready.
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Voltage scaling flag A delay is required for the internal regulator to be ready after the voltage scaling has been changed. VOSF indicates that the regulator reached the voltage level defined with VOS bits of the PWR_CR1 register.
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Programmable voltage detector output
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - Peripheral voltage monitoring output: V<sub>DDA</sub> vs. 1.62 V Note: PVMO1 is cleared when PVM1 is disabled (PVME = 0). After enabling PVM1, the PVM1 output is valid after the PVM1 wakeup time.
    #[inline(always)]
    pub fn pvmo1(&self) -> PVMO1_R {
        PVMO1_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Peripheral voltage monitoring output: V<sub>DDA</sub> vs. 1.8 V Note: PVMO2 is cleared when PVM2 is disabled (PVME = 0). After enabling PVM2, the PVM2 output is valid after the PVM2 wakeup time.
    #[inline(always)]
    pub fn pvmo2(&self) -> PVMO2_R {
        PVMO2_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR2")
            .field("reglps", &self.reglps())
            .field("reglpf", &self.reglpf())
            .field("vosf", &self.vosf())
            .field("pvdo", &self.pvdo())
            .field("pvmo1", &self.pvmo1())
            .field("pvmo2", &self.pvmo2())
            .finish()
    }
}
/**Power status register 2

You can [`read`](crate::Reg::read) this register and get [`sr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#PWR:SR2)*/
pub struct SR2rs;
impl crate::RegisterSpec for SR2rs {
    type Ux = u32;
}
///`read()` method returns [`sr2::R`](R) reader structure
impl crate::Readable for SR2rs {}
///`reset()` method sets SR2 to value 0
impl crate::Resettable for SR2rs {}
