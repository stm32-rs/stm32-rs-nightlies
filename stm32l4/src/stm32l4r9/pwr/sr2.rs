///Register `SR2` reader
pub type R = crate::R<SR2rs>;
/**Low-power regulator started This bit provides the information whether the low-power regulator is ready after a power-on reset or a Standby/Shutdown. If the Standby mode is entered while REGLPS bit is still cleared, the wakeup from Standby mode time may be increased.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGLPS {
    ///0: The low-power regulator is not ready
    NotReady = 0,
    ///1: The low-power regulator is ready
    Ready = 1,
}
impl From<REGLPS> for bool {
    #[inline(always)]
    fn from(variant: REGLPS) -> Self {
        variant as u8 != 0
    }
}
///Field `REGLPS` reader - Low-power regulator started This bit provides the information whether the low-power regulator is ready after a power-on reset or a Standby/Shutdown. If the Standby mode is entered while REGLPS bit is still cleared, the wakeup from Standby mode time may be increased.
pub type REGLPS_R = crate::BitReader<REGLPS>;
impl REGLPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REGLPS {
        match self.bits {
            false => REGLPS::NotReady,
            true => REGLPS::Ready,
        }
    }
    ///The low-power regulator is not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == REGLPS::NotReady
    }
    ///The low-power regulator is ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == REGLPS::Ready
    }
}
/**Low-power regulator flag This bit is set by hardware when the MCU is in Low-power run mode. When the MCU exits from the Low-power run mode, this bit remains at 1 until the regulator is ready in main mode. A polling on this bit must be done before increasing the product frequency. This bit is cleared by hardware when the regulator is ready.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGLPF {
    ///0: The regulator is ready in main mode (MR)
    Mr = 0,
    ///1: The regulator is in low-power mode (LPR)
    Lpr = 1,
}
impl From<REGLPF> for bool {
    #[inline(always)]
    fn from(variant: REGLPF) -> Self {
        variant as u8 != 0
    }
}
///Field `REGLPF` reader - Low-power regulator flag This bit is set by hardware when the MCU is in Low-power run mode. When the MCU exits from the Low-power run mode, this bit remains at 1 until the regulator is ready in main mode. A polling on this bit must be done before increasing the product frequency. This bit is cleared by hardware when the regulator is ready.
pub type REGLPF_R = crate::BitReader<REGLPF>;
impl REGLPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REGLPF {
        match self.bits {
            false => REGLPF::Mr,
            true => REGLPF::Lpr,
        }
    }
    ///The regulator is ready in main mode (MR)
    #[inline(always)]
    pub fn is_mr(&self) -> bool {
        *self == REGLPF::Mr
    }
    ///The regulator is in low-power mode (LPR)
    #[inline(always)]
    pub fn is_lpr(&self) -> bool {
        *self == REGLPF::Lpr
    }
}
/**Voltage scaling flag A delay is required for the internal regulator to be ready after the voltage scaling has been changed. VOSF indicates that the regulator reached the voltage level defined with VOS bits of the PWR_CR1 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOSF {
    ///0: The regulator is ready in the selected voltage range
    Ready = 0,
    ///1: The regulator output voltage is changing to the required voltage level
    NotReady = 1,
}
impl From<VOSF> for bool {
    #[inline(always)]
    fn from(variant: VOSF) -> Self {
        variant as u8 != 0
    }
}
///Field `VOSF` reader - Voltage scaling flag A delay is required for the internal regulator to be ready after the voltage scaling has been changed. VOSF indicates that the regulator reached the voltage level defined with VOS bits of the PWR_CR1 register.
pub type VOSF_R = crate::BitReader<VOSF>;
impl VOSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VOSF {
        match self.bits {
            false => VOSF::Ready,
            true => VOSF::NotReady,
        }
    }
    ///The regulator is ready in the selected voltage range
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VOSF::Ready
    }
    ///The regulator output voltage is changing to the required voltage level
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == VOSF::NotReady
    }
}
/**Power voltage detector output

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDO {
    ///0: VDD is above the selected PVD threshold
    Above = 0,
    ///1: VDD is below the selected PVD threshold
    Below = 1,
}
impl From<PVDO> for bool {
    #[inline(always)]
    fn from(variant: PVDO) -> Self {
        variant as u8 != 0
    }
}
///Field `PVDO` reader - Power voltage detector output
pub type PVDO_R = crate::BitReader<PVDO>;
impl PVDO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVDO {
        match self.bits {
            false => PVDO::Above,
            true => PVDO::Below,
        }
    }
    ///VDD is above the selected PVD threshold
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVDO::Above
    }
    ///VDD is below the selected PVD threshold
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVDO::Below
    }
}
/**Peripheral voltage monitoring output: V<sub>DDUSB</sub> vs. 1.2 V Note: PVMO1 is cleared when PVM1 is disabled (PVME1 = 0). After enabling PVM1, the PVM1 output is valid after the PVM1 wakeup time.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMO1 {
    ///0: VDDUSB voltage is above PVM1 threshold (around 1.2 V)
    Above = 0,
    ///1: VDDUSB voltage is below PVM1 threshold (around 1.2 V)
    Below = 1,
}
impl From<PVMO1> for bool {
    #[inline(always)]
    fn from(variant: PVMO1) -> Self {
        variant as u8 != 0
    }
}
///Field `PVMO1` reader - Peripheral voltage monitoring output: V<sub>DDUSB</sub> vs. 1.2 V Note: PVMO1 is cleared when PVM1 is disabled (PVME1 = 0). After enabling PVM1, the PVM1 output is valid after the PVM1 wakeup time.
pub type PVMO1_R = crate::BitReader<PVMO1>;
impl PVMO1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVMO1 {
        match self.bits {
            false => PVMO1::Above,
            true => PVMO1::Below,
        }
    }
    ///VDDUSB voltage is above PVM1 threshold (around 1.2 V)
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVMO1::Above
    }
    ///VDDUSB voltage is below PVM1 threshold (around 1.2 V)
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVMO1::Below
    }
}
/**Peripheral voltage monitoring output: V<sub>DDIO2</sub> vs. 0.9 V Note: PVMO2 is cleared when PVM2 is disabled (PVME2 = 0). After enabling PVM2, the PVM2 output is valid after the PVM2 wakeup time.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMO2 {
    ///0: VDDIO2 voltage is above PVM2 threshold (around 0.9 V)
    Above = 0,
    ///1: VDDIO2 voltage is below PVM2 threshold (around 0.9 V)
    Below = 1,
}
impl From<PVMO2> for bool {
    #[inline(always)]
    fn from(variant: PVMO2) -> Self {
        variant as u8 != 0
    }
}
///Field `PVMO2` reader - Peripheral voltage monitoring output: V<sub>DDIO2</sub> vs. 0.9 V Note: PVMO2 is cleared when PVM2 is disabled (PVME2 = 0). After enabling PVM2, the PVM2 output is valid after the PVM2 wakeup time.
pub type PVMO2_R = crate::BitReader<PVMO2>;
impl PVMO2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVMO2 {
        match self.bits {
            false => PVMO2::Above,
            true => PVMO2::Below,
        }
    }
    ///VDDIO2 voltage is above PVM2 threshold (around 0.9 V)
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVMO2::Above
    }
    ///VDDIO2 voltage is below PVM2 threshold (around 0.9 V)
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVMO2::Below
    }
}
/**Peripheral voltage monitoring output: V<sub>DDA</sub> vs. 1.62 V Note: PVMO3 is cleared when PVM3 is disabled (PVME3 = 0). After enabling PVM3, the PVM3 output is valid after the PVM3 wakeup time.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMO3 {
    ///0: VDDA voltage is above PVM3 threshold (around 1.62 V)
    Above = 0,
    ///1: VDDA voltage is below PVM3 threshold (around 1.62 V)
    Below = 1,
}
impl From<PVMO3> for bool {
    #[inline(always)]
    fn from(variant: PVMO3) -> Self {
        variant as u8 != 0
    }
}
///Field `PVMO3` reader - Peripheral voltage monitoring output: V<sub>DDA</sub> vs. 1.62 V Note: PVMO3 is cleared when PVM3 is disabled (PVME3 = 0). After enabling PVM3, the PVM3 output is valid after the PVM3 wakeup time.
pub type PVMO3_R = crate::BitReader<PVMO3>;
impl PVMO3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVMO3 {
        match self.bits {
            false => PVMO3::Above,
            true => PVMO3::Below,
        }
    }
    ///VDDA voltage is above PVM3 threshold (around 1.62 V)
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVMO3::Above
    }
    ///VDDA voltage is below PVM3 threshold (around 1.62 V)
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVMO3::Below
    }
}
/**Peripheral voltage monitoring output: V<sub>DDA</sub> vs. 2.2 V Note: PVMO4 is cleared when PVM4 is disabled (PVME4 = 0). After enabling PVM4, the PVM4 output is valid after the PVM4 wakeup time.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMO4 {
    ///0: VDDA voltage is above PVM4 threshold (around 2.2 V)
    Above = 0,
    ///1: VDDA voltage is below PVM4 threshold (around 2.2 V)
    Below = 1,
}
impl From<PVMO4> for bool {
    #[inline(always)]
    fn from(variant: PVMO4) -> Self {
        variant as u8 != 0
    }
}
///Field `PVMO4` reader - Peripheral voltage monitoring output: V<sub>DDA</sub> vs. 2.2 V Note: PVMO4 is cleared when PVM4 is disabled (PVME4 = 0). After enabling PVM4, the PVM4 output is valid after the PVM4 wakeup time.
pub type PVMO4_R = crate::BitReader<PVMO4>;
impl PVMO4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVMO4 {
        match self.bits {
            false => PVMO4::Above,
            true => PVMO4::Below,
        }
    }
    ///VDDA voltage is above PVM4 threshold (around 2.2 V)
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVMO4::Above
    }
    ///VDDA voltage is below PVM4 threshold (around 2.2 V)
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVMO4::Below
    }
}
impl R {
    ///Bit 8 - Low-power regulator started This bit provides the information whether the low-power regulator is ready after a power-on reset or a Standby/Shutdown. If the Standby mode is entered while REGLPS bit is still cleared, the wakeup from Standby mode time may be increased.
    #[inline(always)]
    pub fn reglps(&self) -> REGLPS_R {
        REGLPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Low-power regulator flag This bit is set by hardware when the MCU is in Low-power run mode. When the MCU exits from the Low-power run mode, this bit remains at 1 until the regulator is ready in main mode. A polling on this bit must be done before increasing the product frequency. This bit is cleared by hardware when the regulator is ready.
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Voltage scaling flag A delay is required for the internal regulator to be ready after the voltage scaling has been changed. VOSF indicates that the regulator reached the voltage level defined with VOS bits of the PWR_CR1 register.
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Power voltage detector output
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Peripheral voltage monitoring output: V<sub>DDUSB</sub> vs. 1.2 V Note: PVMO1 is cleared when PVM1 is disabled (PVME1 = 0). After enabling PVM1, the PVM1 output is valid after the PVM1 wakeup time.
    #[inline(always)]
    pub fn pvmo1(&self) -> PVMO1_R {
        PVMO1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Peripheral voltage monitoring output: V<sub>DDIO2</sub> vs. 0.9 V Note: PVMO2 is cleared when PVM2 is disabled (PVME2 = 0). After enabling PVM2, the PVM2 output is valid after the PVM2 wakeup time.
    #[inline(always)]
    pub fn pvmo2(&self) -> PVMO2_R {
        PVMO2_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Peripheral voltage monitoring output: V<sub>DDA</sub> vs. 1.62 V Note: PVMO3 is cleared when PVM3 is disabled (PVME3 = 0). After enabling PVM3, the PVM3 output is valid after the PVM3 wakeup time.
    #[inline(always)]
    pub fn pvmo3(&self) -> PVMO3_R {
        PVMO3_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Peripheral voltage monitoring output: V<sub>DDA</sub> vs. 2.2 V Note: PVMO4 is cleared when PVM4 is disabled (PVME4 = 0). After enabling PVM4, the PVM4 output is valid after the PVM4 wakeup time.
    #[inline(always)]
    pub fn pvmo4(&self) -> PVMO4_R {
        PVMO4_R::new(((self.bits >> 15) & 1) != 0)
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
            .field("pvmo3", &self.pvmo3())
            .field("pvmo4", &self.pvmo4())
            .finish()
    }
}
/**Power status register 2

You can [`read`](crate::Reg::read) this register and get [`sr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#PWR:SR2)*/
pub struct SR2rs;
impl crate::RegisterSpec for SR2rs {
    type Ux = u32;
}
///`read()` method returns [`sr2::R`](R) reader structure
impl crate::Readable for SR2rs {}
///`reset()` method sets SR2 to value 0
impl crate::Resettable for SR2rs {}
