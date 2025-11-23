///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
/**Voltage reference buffer mode enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENVR {
    ///0: Internal voltage reference mode disable (external voltage reference mode)
    Disabled = 0,
    ///1: Internal voltage reference mode (reference buffer enable or hold mode) enable
    Enabled = 1,
}
impl From<ENVR> for bool {
    #[inline(always)]
    fn from(variant: ENVR) -> Self {
        variant as u8 != 0
    }
}
///Field `ENVR` reader - Voltage reference buffer mode enable
pub type ENVR_R = crate::BitReader<ENVR>;
impl ENVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENVR {
        match self.bits {
            false => ENVR::Disabled,
            true => ENVR::Enabled,
        }
    }
    ///Internal voltage reference mode disable (external voltage reference mode)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENVR::Disabled
    }
    ///Internal voltage reference mode (reference buffer enable or hold mode) enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENVR::Enabled
    }
}
///Field `ENVR` writer - Voltage reference buffer mode enable
pub type ENVR_W<'a, REG> = crate::BitWriter<'a, REG, ENVR>;
impl<'a, REG> ENVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Internal voltage reference mode disable (external voltage reference mode)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENVR::Disabled)
    }
    ///Internal voltage reference mode (reference buffer enable or hold mode) enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENVR::Enabled)
    }
}
/**High impedance mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIZ {
    ///0: VREF+ pin is internally connected to the voltage reference buffer output
    Connected = 0,
    ///1: VREF+ pin is high impedance
    HighZ = 1,
}
impl From<HIZ> for bool {
    #[inline(always)]
    fn from(variant: HIZ) -> Self {
        variant as u8 != 0
    }
}
///Field `HIZ` reader - High impedance mode
pub type HIZ_R = crate::BitReader<HIZ>;
impl HIZ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HIZ {
        match self.bits {
            false => HIZ::Connected,
            true => HIZ::HighZ,
        }
    }
    ///VREF+ pin is internally connected to the voltage reference buffer output
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == HIZ::Connected
    }
    ///VREF+ pin is high impedance
    #[inline(always)]
    pub fn is_high_z(&self) -> bool {
        *self == HIZ::HighZ
    }
}
///Field `HIZ` writer - High impedance mode
pub type HIZ_W<'a, REG> = crate::BitWriter<'a, REG, HIZ>;
impl<'a, REG> HIZ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VREF+ pin is internally connected to the voltage reference buffer output
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(HIZ::Connected)
    }
    ///VREF+ pin is high impedance
    #[inline(always)]
    pub fn high_z(self) -> &'a mut crate::W<REG> {
        self.variant(HIZ::HighZ)
    }
}
/**Voltage reference scale

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VRS {
    ///0: Voltage reference set to VREF_OUT1 (around 2.048 V)
    V2_048 = 0,
    ///1: Voltage reference set to VREF_OUT2 (around 2.5 V)
    V2_5 = 1,
}
impl From<VRS> for bool {
    #[inline(always)]
    fn from(variant: VRS) -> Self {
        variant as u8 != 0
    }
}
///Field `VRS` reader - Voltage reference scale
pub type VRS_R = crate::BitReader<VRS>;
impl VRS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VRS {
        match self.bits {
            false => VRS::V2_048,
            true => VRS::V2_5,
        }
    }
    ///Voltage reference set to VREF_OUT1 (around 2.048 V)
    #[inline(always)]
    pub fn is_v2_048(&self) -> bool {
        *self == VRS::V2_048
    }
    ///Voltage reference set to VREF_OUT2 (around 2.5 V)
    #[inline(always)]
    pub fn is_v2_5(&self) -> bool {
        *self == VRS::V2_5
    }
}
///Field `VRS` writer - Voltage reference scale
pub type VRS_W<'a, REG> = crate::BitWriter<'a, REG, VRS>;
impl<'a, REG> VRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage reference set to VREF_OUT1 (around 2.048 V)
    #[inline(always)]
    pub fn v2_048(self) -> &'a mut crate::W<REG> {
        self.variant(VRS::V2_048)
    }
    ///Voltage reference set to VREF_OUT2 (around 2.5 V)
    #[inline(always)]
    pub fn v2_5(self) -> &'a mut crate::W<REG> {
        self.variant(VRS::V2_5)
    }
}
/**Voltage reference buffer ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VRR {
    ///0: The voltage reference buffer output is not ready
    NotReady = 0,
    ///1: The voltage reference buffer output reached the requested level
    Ready = 1,
}
impl From<VRR> for bool {
    #[inline(always)]
    fn from(variant: VRR) -> Self {
        variant as u8 != 0
    }
}
///Field `VRR` reader - Voltage reference buffer ready
pub type VRR_R = crate::BitReader<VRR>;
impl VRR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VRR {
        match self.bits {
            false => VRR::NotReady,
            true => VRR::Ready,
        }
    }
    ///The voltage reference buffer output is not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == VRR::NotReady
    }
    ///The voltage reference buffer output reached the requested level
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VRR::Ready
    }
}
impl R {
    ///Bit 0 - Voltage reference buffer mode enable
    #[inline(always)]
    pub fn envr(&self) -> ENVR_R {
        ENVR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - High impedance mode
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Voltage reference scale
    #[inline(always)]
    pub fn vrs(&self) -> VRS_R {
        VRS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Voltage reference buffer ready
    #[inline(always)]
    pub fn vrr(&self) -> VRR_R {
        VRR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("vrr", &self.vrr())
            .field("vrs", &self.vrs())
            .field("hiz", &self.hiz())
            .field("envr", &self.envr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Voltage reference buffer mode enable
    #[inline(always)]
    pub fn envr(&mut self) -> ENVR_W<'_, CSRrs> {
        ENVR_W::new(self, 0)
    }
    ///Bit 1 - High impedance mode
    #[inline(always)]
    pub fn hiz(&mut self) -> HIZ_W<'_, CSRrs> {
        HIZ_W::new(self, 1)
    }
    ///Bit 2 - Voltage reference scale
    #[inline(always)]
    pub fn vrs(&mut self) -> VRS_W<'_, CSRrs> {
        VRS_W::new(self, 2)
    }
}
/**control and status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#VREFBUF:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0x02
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x02;
}
