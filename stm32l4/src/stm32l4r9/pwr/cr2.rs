///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
/**Power voltage detector enable Note: This bit is write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: This bit is reset only by a system reset.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDE {
    ///0: Power voltage detector disabled
    Disabled = 0,
    ///1: Power voltage detector enabled
    Enabled = 1,
}
impl From<PVDE> for bool {
    #[inline(always)]
    fn from(variant: PVDE) -> Self {
        variant as u8 != 0
    }
}
///Field `PVDE` reader - Power voltage detector enable Note: This bit is write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: This bit is reset only by a system reset.
pub type PVDE_R = crate::BitReader<PVDE>;
impl PVDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVDE {
        match self.bits {
            false => PVDE::Disabled,
            true => PVDE::Enabled,
        }
    }
    ///Power voltage detector disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVDE::Disabled
    }
    ///Power voltage detector enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVDE::Enabled
    }
}
///Field `PVDE` writer - Power voltage detector enable Note: This bit is write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: This bit is reset only by a system reset.
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG, PVDE>;
impl<'a, REG> PVDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Power voltage detector disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE::Disabled)
    }
    ///Power voltage detector enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE::Enabled)
    }
}
/**Power voltage detector level selection. These bits select the voltage threshold detected by the power voltage detector: Note: These bits are write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: These bits are reset only by a system reset.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLS {
    ///0: VPVD0 around 2.0 V
    Vpvd0 = 0,
    ///1: VPVD1 around 2.2 V
    Vpvd1 = 1,
    ///2: VPVD2 around 2.4 V
    Vpvd2 = 2,
    ///3: VPVD3 around 2.5 V
    Vpvd3 = 3,
    ///4: VPVD4 around 2.6 V
    Vpvd4 = 4,
    ///5: VPVD5 around 2.8 V
    Vpvd5 = 5,
    ///6: VPVD6 around 2.9 V
    Vpvd6 = 6,
    ///7: External input analog voltage PVD_IN (compared internally to VREFINT)
    Pvdin = 7,
}
impl From<PLS> for u8 {
    #[inline(always)]
    fn from(variant: PLS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLS {
    type Ux = u8;
}
impl crate::IsEnum for PLS {}
///Field `PLS` reader - Power voltage detector level selection. These bits select the voltage threshold detected by the power voltage detector: Note: These bits are write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: These bits are reset only by a system reset.
pub type PLS_R = crate::FieldReader<PLS>;
impl PLS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLS {
        match self.bits {
            0 => PLS::Vpvd0,
            1 => PLS::Vpvd1,
            2 => PLS::Vpvd2,
            3 => PLS::Vpvd3,
            4 => PLS::Vpvd4,
            5 => PLS::Vpvd5,
            6 => PLS::Vpvd6,
            7 => PLS::Pvdin,
            _ => unreachable!(),
        }
    }
    ///VPVD0 around 2.0 V
    #[inline(always)]
    pub fn is_vpvd0(&self) -> bool {
        *self == PLS::Vpvd0
    }
    ///VPVD1 around 2.2 V
    #[inline(always)]
    pub fn is_vpvd1(&self) -> bool {
        *self == PLS::Vpvd1
    }
    ///VPVD2 around 2.4 V
    #[inline(always)]
    pub fn is_vpvd2(&self) -> bool {
        *self == PLS::Vpvd2
    }
    ///VPVD3 around 2.5 V
    #[inline(always)]
    pub fn is_vpvd3(&self) -> bool {
        *self == PLS::Vpvd3
    }
    ///VPVD4 around 2.6 V
    #[inline(always)]
    pub fn is_vpvd4(&self) -> bool {
        *self == PLS::Vpvd4
    }
    ///VPVD5 around 2.8 V
    #[inline(always)]
    pub fn is_vpvd5(&self) -> bool {
        *self == PLS::Vpvd5
    }
    ///VPVD6 around 2.9 V
    #[inline(always)]
    pub fn is_vpvd6(&self) -> bool {
        *self == PLS::Vpvd6
    }
    ///External input analog voltage PVD_IN (compared internally to VREFINT)
    #[inline(always)]
    pub fn is_pvdin(&self) -> bool {
        *self == PLS::Pvdin
    }
}
///Field `PLS` writer - Power voltage detector level selection. These bits select the voltage threshold detected by the power voltage detector: Note: These bits are write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: These bits are reset only by a system reset.
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PLS, crate::Safe>;
impl<'a, REG> PLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///VPVD0 around 2.0 V
    #[inline(always)]
    pub fn vpvd0(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::Vpvd0)
    }
    ///VPVD1 around 2.2 V
    #[inline(always)]
    pub fn vpvd1(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::Vpvd1)
    }
    ///VPVD2 around 2.4 V
    #[inline(always)]
    pub fn vpvd2(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::Vpvd2)
    }
    ///VPVD3 around 2.5 V
    #[inline(always)]
    pub fn vpvd3(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::Vpvd3)
    }
    ///VPVD4 around 2.6 V
    #[inline(always)]
    pub fn vpvd4(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::Vpvd4)
    }
    ///VPVD5 around 2.8 V
    #[inline(always)]
    pub fn vpvd5(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::Vpvd5)
    }
    ///VPVD6 around 2.9 V
    #[inline(always)]
    pub fn vpvd6(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::Vpvd6)
    }
    ///External input analog voltage PVD_IN (compared internally to VREFINT)
    #[inline(always)]
    pub fn pvdin(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::Pvdin)
    }
}
/**Peripheral voltage monitoring 1 enable: V<sub>DDUSB</sub> vs. 1.2V

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVME1 {
    ///0: PVM2 (VDDUSB monitoring vs. 1.2V threshold) disable
    Disabled = 0,
    ///1: PVM2 (VDDUSB monitoring vs. 1.2V threshold) enable
    Enabled = 1,
}
impl From<PVME1> for bool {
    #[inline(always)]
    fn from(variant: PVME1) -> Self {
        variant as u8 != 0
    }
}
///Field `PVME1` reader - Peripheral voltage monitoring 1 enable: V<sub>DDUSB</sub> vs. 1.2V
pub type PVME1_R = crate::BitReader<PVME1>;
impl PVME1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVME1 {
        match self.bits {
            false => PVME1::Disabled,
            true => PVME1::Enabled,
        }
    }
    ///PVM2 (VDDUSB monitoring vs. 1.2V threshold) disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVME1::Disabled
    }
    ///PVM2 (VDDUSB monitoring vs. 1.2V threshold) enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVME1::Enabled
    }
}
///Field `PVME1` writer - Peripheral voltage monitoring 1 enable: V<sub>DDUSB</sub> vs. 1.2V
pub type PVME1_W<'a, REG> = crate::BitWriter<'a, REG, PVME1>;
impl<'a, REG> PVME1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PVM2 (VDDUSB monitoring vs. 1.2V threshold) disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVME1::Disabled)
    }
    ///PVM2 (VDDUSB monitoring vs. 1.2V threshold) enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVME1::Enabled)
    }
}
/**Peripheral voltage monitoring 2 enable: V<sub>DDIO2</sub> vs. 0.9V

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVME2 {
    ///0: PVM2 (VDDIO2 monitoring vs. 0.9V threshold) disable
    Disabled = 0,
    ///1: PVM2 (VDDIO2 monitoring vs. 0.9V threshold) enable
    Enabled = 1,
}
impl From<PVME2> for bool {
    #[inline(always)]
    fn from(variant: PVME2) -> Self {
        variant as u8 != 0
    }
}
///Field `PVME2` reader - Peripheral voltage monitoring 2 enable: V<sub>DDIO2</sub> vs. 0.9V
pub type PVME2_R = crate::BitReader<PVME2>;
impl PVME2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVME2 {
        match self.bits {
            false => PVME2::Disabled,
            true => PVME2::Enabled,
        }
    }
    ///PVM2 (VDDIO2 monitoring vs. 0.9V threshold) disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVME2::Disabled
    }
    ///PVM2 (VDDIO2 monitoring vs. 0.9V threshold) enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVME2::Enabled
    }
}
///Field `PVME2` writer - Peripheral voltage monitoring 2 enable: V<sub>DDIO2</sub> vs. 0.9V
pub type PVME2_W<'a, REG> = crate::BitWriter<'a, REG, PVME2>;
impl<'a, REG> PVME2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PVM2 (VDDIO2 monitoring vs. 0.9V threshold) disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVME2::Disabled)
    }
    ///PVM2 (VDDIO2 monitoring vs. 0.9V threshold) enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVME2::Enabled)
    }
}
/**Peripheral voltage monitoring 3 enable: V<sub>DDA</sub> vs. 1.62V

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVME3 {
    ///0: PVM3 (VDDA monitoring vs. 1.62V threshold) disable
    Disabled = 0,
    ///1: PVM3 (VDDA monitoring vs. 1.62V threshold) enable
    Enabled = 1,
}
impl From<PVME3> for bool {
    #[inline(always)]
    fn from(variant: PVME3) -> Self {
        variant as u8 != 0
    }
}
///Field `PVME3` reader - Peripheral voltage monitoring 3 enable: V<sub>DDA</sub> vs. 1.62V
pub type PVME3_R = crate::BitReader<PVME3>;
impl PVME3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVME3 {
        match self.bits {
            false => PVME3::Disabled,
            true => PVME3::Enabled,
        }
    }
    ///PVM3 (VDDA monitoring vs. 1.62V threshold) disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVME3::Disabled
    }
    ///PVM3 (VDDA monitoring vs. 1.62V threshold) enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVME3::Enabled
    }
}
///Field `PVME3` writer - Peripheral voltage monitoring 3 enable: V<sub>DDA</sub> vs. 1.62V
pub type PVME3_W<'a, REG> = crate::BitWriter<'a, REG, PVME3>;
impl<'a, REG> PVME3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PVM3 (VDDA monitoring vs. 1.62V threshold) disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVME3::Disabled)
    }
    ///PVM3 (VDDA monitoring vs. 1.62V threshold) enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVME3::Enabled)
    }
}
/**Peripheral voltage monitoring 4 enable: V<sub>DDA</sub> vs. 2.2V

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVME4 {
    ///0: PVM4 (VDDA monitoring vs. 2.2V threshold) disable
    Disabled = 0,
    ///1: PVM4 (VDDA monitoring vs. 2.2V threshold) enable
    Enabled = 1,
}
impl From<PVME4> for bool {
    #[inline(always)]
    fn from(variant: PVME4) -> Self {
        variant as u8 != 0
    }
}
///Field `PVME4` reader - Peripheral voltage monitoring 4 enable: V<sub>DDA</sub> vs. 2.2V
pub type PVME4_R = crate::BitReader<PVME4>;
impl PVME4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVME4 {
        match self.bits {
            false => PVME4::Disabled,
            true => PVME4::Enabled,
        }
    }
    ///PVM4 (VDDA monitoring vs. 2.2V threshold) disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVME4::Disabled
    }
    ///PVM4 (VDDA monitoring vs. 2.2V threshold) enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVME4::Enabled
    }
}
///Field `PVME4` writer - Peripheral voltage monitoring 4 enable: V<sub>DDA</sub> vs. 2.2V
pub type PVME4_W<'a, REG> = crate::BitWriter<'a, REG, PVME4>;
impl<'a, REG> PVME4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PVM4 (VDDA monitoring vs. 2.2V threshold) disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVME4::Disabled)
    }
    ///PVM4 (VDDA monitoring vs. 2.2V threshold) enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVME4::Enabled)
    }
}
/**V<sub>DDIO2</sub> Independent I/Os supply valid This bit is used to validate the V<sub>DDIO2</sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\[15:2\]. If V<sub>DDIO2</sub> is not always present in the application, the PVM can be used to determine whether this supply is ready or not.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOSV {
    ///0: VDDIO2 is not present. Logical and electrical isolation is applied to ignore this supply
    NotPresent = 0,
    ///1: VDDIO2 is valid
    Valid = 1,
}
impl From<IOSV> for bool {
    #[inline(always)]
    fn from(variant: IOSV) -> Self {
        variant as u8 != 0
    }
}
///Field `IOSV` reader - V<sub>DDIO2</sub> Independent I/Os supply valid This bit is used to validate the V<sub>DDIO2</sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\[15:2\]. If V<sub>DDIO2</sub> is not always present in the application, the PVM can be used to determine whether this supply is ready or not.
pub type IOSV_R = crate::BitReader<IOSV>;
impl IOSV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IOSV {
        match self.bits {
            false => IOSV::NotPresent,
            true => IOSV::Valid,
        }
    }
    ///VDDIO2 is not present. Logical and electrical isolation is applied to ignore this supply
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == IOSV::NotPresent
    }
    ///VDDIO2 is valid
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == IOSV::Valid
    }
}
///Field `IOSV` writer - V<sub>DDIO2</sub> Independent I/Os supply valid This bit is used to validate the V<sub>DDIO2</sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\[15:2\]. If V<sub>DDIO2</sub> is not always present in the application, the PVM can be used to determine whether this supply is ready or not.
pub type IOSV_W<'a, REG> = crate::BitWriter<'a, REG, IOSV>;
impl<'a, REG> IOSV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VDDIO2 is not present. Logical and electrical isolation is applied to ignore this supply
    #[inline(always)]
    pub fn not_present(self) -> &'a mut crate::W<REG> {
        self.variant(IOSV::NotPresent)
    }
    ///VDDIO2 is valid
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(IOSV::Valid)
    }
}
/**V<sub>DDUSB</sub> USB supply valid This bit is used to validate the V<sub>DDUSB</sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB OTG_FS peripheral. If V<sub>DDUSB</sub> is not always present in the application, the PVM can be used to determine whether this supply is ready or not.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USV {
    ///0: VDDUSB is not present. Logical and electrical isolation is applied to ignore this supply
    NotPresent = 0,
    ///1: VDDUSB is valid
    Valid = 1,
}
impl From<USV> for bool {
    #[inline(always)]
    fn from(variant: USV) -> Self {
        variant as u8 != 0
    }
}
///Field `USV` reader - V<sub>DDUSB</sub> USB supply valid This bit is used to validate the V<sub>DDUSB</sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB OTG_FS peripheral. If V<sub>DDUSB</sub> is not always present in the application, the PVM can be used to determine whether this supply is ready or not.
pub type USV_R = crate::BitReader<USV>;
impl USV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USV {
        match self.bits {
            false => USV::NotPresent,
            true => USV::Valid,
        }
    }
    ///VDDUSB is not present. Logical and electrical isolation is applied to ignore this supply
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == USV::NotPresent
    }
    ///VDDUSB is valid
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == USV::Valid
    }
}
///Field `USV` writer - V<sub>DDUSB</sub> USB supply valid This bit is used to validate the V<sub>DDUSB</sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB OTG_FS peripheral. If V<sub>DDUSB</sub> is not always present in the application, the PVM can be used to determine whether this supply is ready or not.
pub type USV_W<'a, REG> = crate::BitWriter<'a, REG, USV>;
impl<'a, REG> USV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VDDUSB is not present. Logical and electrical isolation is applied to ignore this supply
    #[inline(always)]
    pub fn not_present(self) -> &'a mut crate::W<REG> {
        self.variant(USV::NotPresent)
    }
    ///VDDUSB is valid
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(USV::Valid)
    }
}
impl R {
    ///Bit 0 - Power voltage detector enable Note: This bit is write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: This bit is reset only by a system reset.
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - Power voltage detector level selection. These bits select the voltage threshold detected by the power voltage detector: Note: These bits are write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: These bits are reset only by a system reset.
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 4 - Peripheral voltage monitoring 1 enable: V<sub>DDUSB</sub> vs. 1.2V
    #[inline(always)]
    pub fn pvme1(&self) -> PVME1_R {
        PVME1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Peripheral voltage monitoring 2 enable: V<sub>DDIO2</sub> vs. 0.9V
    #[inline(always)]
    pub fn pvme2(&self) -> PVME2_R {
        PVME2_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Peripheral voltage monitoring 3 enable: V<sub>DDA</sub> vs. 1.62V
    #[inline(always)]
    pub fn pvme3(&self) -> PVME3_R {
        PVME3_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Peripheral voltage monitoring 4 enable: V<sub>DDA</sub> vs. 2.2V
    #[inline(always)]
    pub fn pvme4(&self) -> PVME4_R {
        PVME4_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - V<sub>DDIO2</sub> Independent I/Os supply valid This bit is used to validate the V<sub>DDIO2</sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\[15:2\]. If V<sub>DDIO2</sub> is not always present in the application, the PVM can be used to determine whether this supply is ready or not.
    #[inline(always)]
    pub fn iosv(&self) -> IOSV_R {
        IOSV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - V<sub>DDUSB</sub> USB supply valid This bit is used to validate the V<sub>DDUSB</sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB OTG_FS peripheral. If V<sub>DDUSB</sub> is not always present in the application, the PVM can be used to determine whether this supply is ready or not.
    #[inline(always)]
    pub fn usv(&self) -> USV_R {
        USV_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("pvde", &self.pvde())
            .field("pls", &self.pls())
            .field("pvme1", &self.pvme1())
            .field("pvme2", &self.pvme2())
            .field("pvme3", &self.pvme3())
            .field("pvme4", &self.pvme4())
            .field("iosv", &self.iosv())
            .field("usv", &self.usv())
            .finish()
    }
}
impl W {
    ///Bit 0 - Power voltage detector enable Note: This bit is write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: This bit is reset only by a system reset.
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<'_, CR2rs> {
        PVDE_W::new(self, 0)
    }
    ///Bits 1:3 - Power voltage detector level selection. These bits select the voltage threshold detected by the power voltage detector: Note: These bits are write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: These bits are reset only by a system reset.
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W<'_, CR2rs> {
        PLS_W::new(self, 1)
    }
    ///Bit 4 - Peripheral voltage monitoring 1 enable: V<sub>DDUSB</sub> vs. 1.2V
    #[inline(always)]
    pub fn pvme1(&mut self) -> PVME1_W<'_, CR2rs> {
        PVME1_W::new(self, 4)
    }
    ///Bit 5 - Peripheral voltage monitoring 2 enable: V<sub>DDIO2</sub> vs. 0.9V
    #[inline(always)]
    pub fn pvme2(&mut self) -> PVME2_W<'_, CR2rs> {
        PVME2_W::new(self, 5)
    }
    ///Bit 6 - Peripheral voltage monitoring 3 enable: V<sub>DDA</sub> vs. 1.62V
    #[inline(always)]
    pub fn pvme3(&mut self) -> PVME3_W<'_, CR2rs> {
        PVME3_W::new(self, 6)
    }
    ///Bit 7 - Peripheral voltage monitoring 4 enable: V<sub>DDA</sub> vs. 2.2V
    #[inline(always)]
    pub fn pvme4(&mut self) -> PVME4_W<'_, CR2rs> {
        PVME4_W::new(self, 7)
    }
    ///Bit 9 - V<sub>DDIO2</sub> Independent I/Os supply valid This bit is used to validate the V<sub>DDIO2</sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\[15:2\]. If V<sub>DDIO2</sub> is not always present in the application, the PVM can be used to determine whether this supply is ready or not.
    #[inline(always)]
    pub fn iosv(&mut self) -> IOSV_W<'_, CR2rs> {
        IOSV_W::new(self, 9)
    }
    ///Bit 10 - V<sub>DDUSB</sub> USB supply valid This bit is used to validate the V<sub>DDUSB</sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB OTG_FS peripheral. If V<sub>DDUSB</sub> is not always present in the application, the PVM can be used to determine whether this supply is ready or not.
    #[inline(always)]
    pub fn usv(&mut self) -> USV_W<'_, CR2rs> {
        USV_W::new(self, 10)
    }
}
/**Power control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#PWR:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
