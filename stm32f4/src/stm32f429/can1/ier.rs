///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
/**TMEIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMEIE {
    ///0: No interrupt when RQCPx bit is set
    Disabled = 0,
    ///1: Interrupt generated when RQCPx bit is set
    Enabled = 1,
}
impl From<TMEIE> for bool {
    #[inline(always)]
    fn from(variant: TMEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TMEIE` reader - TMEIE
pub type TMEIE_R = crate::BitReader<TMEIE>;
impl TMEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TMEIE {
        match self.bits {
            false => TMEIE::Disabled,
            true => TMEIE::Enabled,
        }
    }
    ///No interrupt when RQCPx bit is set
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TMEIE::Disabled
    }
    ///Interrupt generated when RQCPx bit is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TMEIE::Enabled
    }
}
///Field `TMEIE` writer - TMEIE
pub type TMEIE_W<'a, REG> = crate::BitWriter<'a, REG, TMEIE>;
impl<'a, REG> TMEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt when RQCPx bit is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TMEIE::Disabled)
    }
    ///Interrupt generated when RQCPx bit is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TMEIE::Enabled)
    }
}
/**FMPIE0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMPIE0 {
    ///0: No interrupt generated when state of FMP\[1:0\] bits are not 00
    Disabled = 0,
    ///1: Interrupt generated when state of FMP\[1:0\] bits are not 00b
    Enabled = 1,
}
impl From<FMPIE0> for bool {
    #[inline(always)]
    fn from(variant: FMPIE0) -> Self {
        variant as u8 != 0
    }
}
///Field `FMPIE0` reader - FMPIE0
pub type FMPIE0_R = crate::BitReader<FMPIE0>;
impl FMPIE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMPIE0 {
        match self.bits {
            false => FMPIE0::Disabled,
            true => FMPIE0::Enabled,
        }
    }
    ///No interrupt generated when state of FMP\[1:0\] bits are not 00
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FMPIE0::Disabled
    }
    ///Interrupt generated when state of FMP\[1:0\] bits are not 00b
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FMPIE0::Enabled
    }
}
///Field `FMPIE0` writer - FMPIE0
pub type FMPIE0_W<'a, REG> = crate::BitWriter<'a, REG, FMPIE0>;
impl<'a, REG> FMPIE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt generated when state of FMP\[1:0\] bits are not 00
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMPIE0::Disabled)
    }
    ///Interrupt generated when state of FMP\[1:0\] bits are not 00b
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMPIE0::Enabled)
    }
}
/**FFIE0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFIE0 {
    ///0: No interrupt when FULL bit is set
    Disabled = 0,
    ///1: Interrupt generated when FULL bit is set
    Enabled = 1,
}
impl From<FFIE0> for bool {
    #[inline(always)]
    fn from(variant: FFIE0) -> Self {
        variant as u8 != 0
    }
}
///Field `FFIE0` reader - FFIE0
pub type FFIE0_R = crate::BitReader<FFIE0>;
impl FFIE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FFIE0 {
        match self.bits {
            false => FFIE0::Disabled,
            true => FFIE0::Enabled,
        }
    }
    ///No interrupt when FULL bit is set
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FFIE0::Disabled
    }
    ///Interrupt generated when FULL bit is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FFIE0::Enabled
    }
}
///Field `FFIE0` writer - FFIE0
pub type FFIE0_W<'a, REG> = crate::BitWriter<'a, REG, FFIE0>;
impl<'a, REG> FFIE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt when FULL bit is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FFIE0::Disabled)
    }
    ///Interrupt generated when FULL bit is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FFIE0::Enabled)
    }
}
/**FOVIE0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOVIE0 {
    ///0: No interrupt when FOVR bit is set
    Disabled = 0,
    ///1: Interrupt generated when FOVR bit is set
    Enabled = 1,
}
impl From<FOVIE0> for bool {
    #[inline(always)]
    fn from(variant: FOVIE0) -> Self {
        variant as u8 != 0
    }
}
///Field `FOVIE0` reader - FOVIE0
pub type FOVIE0_R = crate::BitReader<FOVIE0>;
impl FOVIE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FOVIE0 {
        match self.bits {
            false => FOVIE0::Disabled,
            true => FOVIE0::Enabled,
        }
    }
    ///No interrupt when FOVR bit is set
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FOVIE0::Disabled
    }
    ///Interrupt generated when FOVR bit is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FOVIE0::Enabled
    }
}
///Field `FOVIE0` writer - FOVIE0
pub type FOVIE0_W<'a, REG> = crate::BitWriter<'a, REG, FOVIE0>;
impl<'a, REG> FOVIE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt when FOVR bit is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FOVIE0::Disabled)
    }
    ///Interrupt generated when FOVR bit is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FOVIE0::Enabled)
    }
}
/**FMPIE1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMPIE1 {
    ///0: No interrupt generated when state of FMP\[1:0\] bits are not 00b
    Disabled = 0,
    ///1: Interrupt generated when state of FMP\[1:0\] bits are not 00b
    Enabled = 1,
}
impl From<FMPIE1> for bool {
    #[inline(always)]
    fn from(variant: FMPIE1) -> Self {
        variant as u8 != 0
    }
}
///Field `FMPIE1` reader - FMPIE1
pub type FMPIE1_R = crate::BitReader<FMPIE1>;
impl FMPIE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMPIE1 {
        match self.bits {
            false => FMPIE1::Disabled,
            true => FMPIE1::Enabled,
        }
    }
    ///No interrupt generated when state of FMP\[1:0\] bits are not 00b
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FMPIE1::Disabled
    }
    ///Interrupt generated when state of FMP\[1:0\] bits are not 00b
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FMPIE1::Enabled
    }
}
///Field `FMPIE1` writer - FMPIE1
pub type FMPIE1_W<'a, REG> = crate::BitWriter<'a, REG, FMPIE1>;
impl<'a, REG> FMPIE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt generated when state of FMP\[1:0\] bits are not 00b
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMPIE1::Disabled)
    }
    ///Interrupt generated when state of FMP\[1:0\] bits are not 00b
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMPIE1::Enabled)
    }
}
/**FFIE1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFIE1 {
    ///0: No interrupt when FULL bit is set
    Disabled = 0,
    ///1: Interrupt generated when FULL bit is set
    Enabled = 1,
}
impl From<FFIE1> for bool {
    #[inline(always)]
    fn from(variant: FFIE1) -> Self {
        variant as u8 != 0
    }
}
///Field `FFIE1` reader - FFIE1
pub type FFIE1_R = crate::BitReader<FFIE1>;
impl FFIE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FFIE1 {
        match self.bits {
            false => FFIE1::Disabled,
            true => FFIE1::Enabled,
        }
    }
    ///No interrupt when FULL bit is set
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FFIE1::Disabled
    }
    ///Interrupt generated when FULL bit is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FFIE1::Enabled
    }
}
///Field `FFIE1` writer - FFIE1
pub type FFIE1_W<'a, REG> = crate::BitWriter<'a, REG, FFIE1>;
impl<'a, REG> FFIE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt when FULL bit is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FFIE1::Disabled)
    }
    ///Interrupt generated when FULL bit is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FFIE1::Enabled)
    }
}
/**FOVIE1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOVIE1 {
    ///0: No interrupt when FOVR is set
    Disabled = 0,
    ///1: Interrupt generation when FOVR is set
    Enabled = 1,
}
impl From<FOVIE1> for bool {
    #[inline(always)]
    fn from(variant: FOVIE1) -> Self {
        variant as u8 != 0
    }
}
///Field `FOVIE1` reader - FOVIE1
pub type FOVIE1_R = crate::BitReader<FOVIE1>;
impl FOVIE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FOVIE1 {
        match self.bits {
            false => FOVIE1::Disabled,
            true => FOVIE1::Enabled,
        }
    }
    ///No interrupt when FOVR is set
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FOVIE1::Disabled
    }
    ///Interrupt generation when FOVR is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FOVIE1::Enabled
    }
}
///Field `FOVIE1` writer - FOVIE1
pub type FOVIE1_W<'a, REG> = crate::BitWriter<'a, REG, FOVIE1>;
impl<'a, REG> FOVIE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt when FOVR is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FOVIE1::Disabled)
    }
    ///Interrupt generation when FOVR is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FOVIE1::Enabled)
    }
}
/**EWGIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWGIE {
    ///0: ERRI bit will not be set when EWGF is set
    Disabled = 0,
    ///1: ERRI bit will be set when EWGF is set
    Enabled = 1,
}
impl From<EWGIE> for bool {
    #[inline(always)]
    fn from(variant: EWGIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EWGIE` reader - EWGIE
pub type EWGIE_R = crate::BitReader<EWGIE>;
impl EWGIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWGIE {
        match self.bits {
            false => EWGIE::Disabled,
            true => EWGIE::Enabled,
        }
    }
    ///ERRI bit will not be set when EWGF is set
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWGIE::Disabled
    }
    ///ERRI bit will be set when EWGF is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWGIE::Enabled
    }
}
///Field `EWGIE` writer - EWGIE
pub type EWGIE_W<'a, REG> = crate::BitWriter<'a, REG, EWGIE>;
impl<'a, REG> EWGIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ERRI bit will not be set when EWGF is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWGIE::Disabled)
    }
    ///ERRI bit will be set when EWGF is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWGIE::Enabled)
    }
}
/**EPVIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPVIE {
    ///0: ERRI bit will not be set when EPVF is set
    Disabled = 0,
    ///1: ERRI bit will be set when EPVF is set
    Enabled = 1,
}
impl From<EPVIE> for bool {
    #[inline(always)]
    fn from(variant: EPVIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EPVIE` reader - EPVIE
pub type EPVIE_R = crate::BitReader<EPVIE>;
impl EPVIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EPVIE {
        match self.bits {
            false => EPVIE::Disabled,
            true => EPVIE::Enabled,
        }
    }
    ///ERRI bit will not be set when EPVF is set
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EPVIE::Disabled
    }
    ///ERRI bit will be set when EPVF is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EPVIE::Enabled
    }
}
///Field `EPVIE` writer - EPVIE
pub type EPVIE_W<'a, REG> = crate::BitWriter<'a, REG, EPVIE>;
impl<'a, REG> EPVIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ERRI bit will not be set when EPVF is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EPVIE::Disabled)
    }
    ///ERRI bit will be set when EPVF is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EPVIE::Enabled)
    }
}
/**BOFIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOFIE {
    ///0: ERRI bit will not be set when BOFF is set
    Disabled = 0,
    ///1: ERRI bit will be set when BOFF is set
    Enabled = 1,
}
impl From<BOFIE> for bool {
    #[inline(always)]
    fn from(variant: BOFIE) -> Self {
        variant as u8 != 0
    }
}
///Field `BOFIE` reader - BOFIE
pub type BOFIE_R = crate::BitReader<BOFIE>;
impl BOFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOFIE {
        match self.bits {
            false => BOFIE::Disabled,
            true => BOFIE::Enabled,
        }
    }
    ///ERRI bit will not be set when BOFF is set
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOFIE::Disabled
    }
    ///ERRI bit will be set when BOFF is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOFIE::Enabled
    }
}
///Field `BOFIE` writer - BOFIE
pub type BOFIE_W<'a, REG> = crate::BitWriter<'a, REG, BOFIE>;
impl<'a, REG> BOFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ERRI bit will not be set when BOFF is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOFIE::Disabled)
    }
    ///ERRI bit will be set when BOFF is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOFIE::Enabled)
    }
}
/**LECIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LECIE {
    ///0: ERRI bit will not be set when the error code in LEC\[2:0\] is set by hardware on error detection
    Disabled = 0,
    ///1: ERRI bit will be set when the error code in LEC\[2:0\] is set by hardware on error detection
    Enabled = 1,
}
impl From<LECIE> for bool {
    #[inline(always)]
    fn from(variant: LECIE) -> Self {
        variant as u8 != 0
    }
}
///Field `LECIE` reader - LECIE
pub type LECIE_R = crate::BitReader<LECIE>;
impl LECIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LECIE {
        match self.bits {
            false => LECIE::Disabled,
            true => LECIE::Enabled,
        }
    }
    ///ERRI bit will not be set when the error code in LEC\[2:0\] is set by hardware on error detection
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LECIE::Disabled
    }
    ///ERRI bit will be set when the error code in LEC\[2:0\] is set by hardware on error detection
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LECIE::Enabled
    }
}
///Field `LECIE` writer - LECIE
pub type LECIE_W<'a, REG> = crate::BitWriter<'a, REG, LECIE>;
impl<'a, REG> LECIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ERRI bit will not be set when the error code in LEC\[2:0\] is set by hardware on error detection
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LECIE::Disabled)
    }
    ///ERRI bit will be set when the error code in LEC\[2:0\] is set by hardware on error detection
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LECIE::Enabled)
    }
}
/**ERRIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE {
    ///0: No interrupt will be generated when an error condition is pending in the CAN_ESR
    Disabled = 0,
    ///1: An interrupt will be generation when an error condition is pending in the CAN_ESR
    Enabled = 1,
}
impl From<ERRIE> for bool {
    #[inline(always)]
    fn from(variant: ERRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ERRIE` reader - ERRIE
pub type ERRIE_R = crate::BitReader<ERRIE>;
impl ERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE {
        match self.bits {
            false => ERRIE::Disabled,
            true => ERRIE::Enabled,
        }
    }
    ///No interrupt will be generated when an error condition is pending in the CAN_ESR
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE::Disabled
    }
    ///An interrupt will be generation when an error condition is pending in the CAN_ESR
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE::Enabled
    }
}
///Field `ERRIE` writer - ERRIE
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt will be generated when an error condition is pending in the CAN_ESR
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Disabled)
    }
    ///An interrupt will be generation when an error condition is pending in the CAN_ESR
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Enabled)
    }
}
/**WKUIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUIE {
    ///0: No interrupt when WKUI is set
    Disabled = 0,
    ///1: Interrupt generated when WKUI bit is set
    Enabled = 1,
}
impl From<WKUIE> for bool {
    #[inline(always)]
    fn from(variant: WKUIE) -> Self {
        variant as u8 != 0
    }
}
///Field `WKUIE` reader - WKUIE
pub type WKUIE_R = crate::BitReader<WKUIE>;
impl WKUIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WKUIE {
        match self.bits {
            false => WKUIE::Disabled,
            true => WKUIE::Enabled,
        }
    }
    ///No interrupt when WKUI is set
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WKUIE::Disabled
    }
    ///Interrupt generated when WKUI bit is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WKUIE::Enabled
    }
}
///Field `WKUIE` writer - WKUIE
pub type WKUIE_W<'a, REG> = crate::BitWriter<'a, REG, WKUIE>;
impl<'a, REG> WKUIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt when WKUI is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WKUIE::Disabled)
    }
    ///Interrupt generated when WKUI bit is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WKUIE::Enabled)
    }
}
/**SLKIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLKIE {
    ///0: No interrupt when SLAKI bit is set
    Disabled = 0,
    ///1: Interrupt generated when SLAKI bit is set
    Enabled = 1,
}
impl From<SLKIE> for bool {
    #[inline(always)]
    fn from(variant: SLKIE) -> Self {
        variant as u8 != 0
    }
}
///Field `SLKIE` reader - SLKIE
pub type SLKIE_R = crate::BitReader<SLKIE>;
impl SLKIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SLKIE {
        match self.bits {
            false => SLKIE::Disabled,
            true => SLKIE::Enabled,
        }
    }
    ///No interrupt when SLAKI bit is set
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLKIE::Disabled
    }
    ///Interrupt generated when SLAKI bit is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLKIE::Enabled
    }
}
///Field `SLKIE` writer - SLKIE
pub type SLKIE_W<'a, REG> = crate::BitWriter<'a, REG, SLKIE>;
impl<'a, REG> SLKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt when SLAKI bit is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SLKIE::Disabled)
    }
    ///Interrupt generated when SLAKI bit is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SLKIE::Enabled)
    }
}
impl R {
    ///Bit 0 - TMEIE
    #[inline(always)]
    pub fn tmeie(&self) -> TMEIE_R {
        TMEIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FMPIE0
    #[inline(always)]
    pub fn fmpie0(&self) -> FMPIE0_R {
        FMPIE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FFIE0
    #[inline(always)]
    pub fn ffie0(&self) -> FFIE0_R {
        FFIE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FOVIE0
    #[inline(always)]
    pub fn fovie0(&self) -> FOVIE0_R {
        FOVIE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FMPIE1
    #[inline(always)]
    pub fn fmpie1(&self) -> FMPIE1_R {
        FMPIE1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - FFIE1
    #[inline(always)]
    pub fn ffie1(&self) -> FFIE1_R {
        FFIE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - FOVIE1
    #[inline(always)]
    pub fn fovie1(&self) -> FOVIE1_R {
        FOVIE1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - EWGIE
    #[inline(always)]
    pub fn ewgie(&self) -> EWGIE_R {
        EWGIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - EPVIE
    #[inline(always)]
    pub fn epvie(&self) -> EPVIE_R {
        EPVIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - BOFIE
    #[inline(always)]
    pub fn bofie(&self) -> BOFIE_R {
        BOFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LECIE
    #[inline(always)]
    pub fn lecie(&self) -> LECIE_R {
        LECIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - ERRIE
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - WKUIE
    #[inline(always)]
    pub fn wkuie(&self) -> WKUIE_R {
        WKUIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SLKIE
    #[inline(always)]
    pub fn slkie(&self) -> SLKIE_R {
        SLKIE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("slkie", &self.slkie())
            .field("wkuie", &self.wkuie())
            .field("errie", &self.errie())
            .field("lecie", &self.lecie())
            .field("bofie", &self.bofie())
            .field("epvie", &self.epvie())
            .field("ewgie", &self.ewgie())
            .field("fovie1", &self.fovie1())
            .field("ffie1", &self.ffie1())
            .field("fmpie1", &self.fmpie1())
            .field("fovie0", &self.fovie0())
            .field("ffie0", &self.ffie0())
            .field("fmpie0", &self.fmpie0())
            .field("tmeie", &self.tmeie())
            .finish()
    }
}
impl W {
    ///Bit 0 - TMEIE
    #[inline(always)]
    pub fn tmeie(&mut self) -> TMEIE_W<'_, IERrs> {
        TMEIE_W::new(self, 0)
    }
    ///Bit 1 - FMPIE0
    #[inline(always)]
    pub fn fmpie0(&mut self) -> FMPIE0_W<'_, IERrs> {
        FMPIE0_W::new(self, 1)
    }
    ///Bit 2 - FFIE0
    #[inline(always)]
    pub fn ffie0(&mut self) -> FFIE0_W<'_, IERrs> {
        FFIE0_W::new(self, 2)
    }
    ///Bit 3 - FOVIE0
    #[inline(always)]
    pub fn fovie0(&mut self) -> FOVIE0_W<'_, IERrs> {
        FOVIE0_W::new(self, 3)
    }
    ///Bit 4 - FMPIE1
    #[inline(always)]
    pub fn fmpie1(&mut self) -> FMPIE1_W<'_, IERrs> {
        FMPIE1_W::new(self, 4)
    }
    ///Bit 5 - FFIE1
    #[inline(always)]
    pub fn ffie1(&mut self) -> FFIE1_W<'_, IERrs> {
        FFIE1_W::new(self, 5)
    }
    ///Bit 6 - FOVIE1
    #[inline(always)]
    pub fn fovie1(&mut self) -> FOVIE1_W<'_, IERrs> {
        FOVIE1_W::new(self, 6)
    }
    ///Bit 8 - EWGIE
    #[inline(always)]
    pub fn ewgie(&mut self) -> EWGIE_W<'_, IERrs> {
        EWGIE_W::new(self, 8)
    }
    ///Bit 9 - EPVIE
    #[inline(always)]
    pub fn epvie(&mut self) -> EPVIE_W<'_, IERrs> {
        EPVIE_W::new(self, 9)
    }
    ///Bit 10 - BOFIE
    #[inline(always)]
    pub fn bofie(&mut self) -> BOFIE_W<'_, IERrs> {
        BOFIE_W::new(self, 10)
    }
    ///Bit 11 - LECIE
    #[inline(always)]
    pub fn lecie(&mut self) -> LECIE_W<'_, IERrs> {
        LECIE_W::new(self, 11)
    }
    ///Bit 15 - ERRIE
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, IERrs> {
        ERRIE_W::new(self, 15)
    }
    ///Bit 16 - WKUIE
    #[inline(always)]
    pub fn wkuie(&mut self) -> WKUIE_W<'_, IERrs> {
        WKUIE_W::new(self, 16)
    }
    ///Bit 17 - SLKIE
    #[inline(always)]
    pub fn slkie(&mut self) -> SLKIE_W<'_, IERrs> {
        SLKIE_W::new(self, 17)
    }
}
/**interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F429.html#CAN1:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
