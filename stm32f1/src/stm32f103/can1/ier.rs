#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "TMEIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMEIE {
    #[doc = "0: No interrupt when RQCPx bit is set"]
    Disabled = 0,
    #[doc = "1: Interrupt generated when RQCPx bit is set"]
    Enabled = 1,
}
impl From<TMEIE> for bool {
    #[inline(always)]
    fn from(variant: TMEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMEIE` reader - TMEIE"]
pub type TMEIE_R = crate::BitReader<TMEIE>;
impl TMEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMEIE {
        match self.bits {
            false => TMEIE::Disabled,
            true => TMEIE::Enabled,
        }
    }
    #[doc = "No interrupt when RQCPx bit is set"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TMEIE::Disabled
    }
    #[doc = "Interrupt generated when RQCPx bit is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TMEIE::Enabled
    }
}
#[doc = "Field `TMEIE` writer - TMEIE"]
pub type TMEIE_W<'a, REG> = crate::BitWriter<'a, REG, TMEIE>;
impl<'a, REG> TMEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt when RQCPx bit is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TMEIE::Disabled)
    }
    #[doc = "Interrupt generated when RQCPx bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TMEIE::Enabled)
    }
}
#[doc = "FMPIE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMPIE0 {
    #[doc = "0: No interrupt generated when state of FMP\\[1:0\\]
bits are not 00"]
    Disabled = 0,
    #[doc = "1: Interrupt generated when state of FMP\\[1:0\\]
bits are not 00b"]
    Enabled = 1,
}
impl From<FMPIE0> for bool {
    #[inline(always)]
    fn from(variant: FMPIE0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMPIE0` reader - FMPIE0"]
pub type FMPIE0_R = crate::BitReader<FMPIE0>;
impl FMPIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FMPIE0 {
        match self.bits {
            false => FMPIE0::Disabled,
            true => FMPIE0::Enabled,
        }
    }
    #[doc = "No interrupt generated when state of FMP\\[1:0\\]
bits are not 00"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FMPIE0::Disabled
    }
    #[doc = "Interrupt generated when state of FMP\\[1:0\\]
bits are not 00b"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FMPIE0::Enabled
    }
}
#[doc = "Field `FMPIE0` writer - FMPIE0"]
pub type FMPIE0_W<'a, REG> = crate::BitWriter<'a, REG, FMPIE0>;
impl<'a, REG> FMPIE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt generated when state of FMP\\[1:0\\]
bits are not 00"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMPIE0::Disabled)
    }
    #[doc = "Interrupt generated when state of FMP\\[1:0\\]
bits are not 00b"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMPIE0::Enabled)
    }
}
#[doc = "FFIE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFIE0 {
    #[doc = "0: No interrupt when FULL bit is set"]
    Disabled = 0,
    #[doc = "1: Interrupt generated when FULL bit is set"]
    Enabled = 1,
}
impl From<FFIE0> for bool {
    #[inline(always)]
    fn from(variant: FFIE0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFIE0` reader - FFIE0"]
pub type FFIE0_R = crate::BitReader<FFIE0>;
impl FFIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FFIE0 {
        match self.bits {
            false => FFIE0::Disabled,
            true => FFIE0::Enabled,
        }
    }
    #[doc = "No interrupt when FULL bit is set"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FFIE0::Disabled
    }
    #[doc = "Interrupt generated when FULL bit is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FFIE0::Enabled
    }
}
#[doc = "Field `FFIE0` writer - FFIE0"]
pub type FFIE0_W<'a, REG> = crate::BitWriter<'a, REG, FFIE0>;
impl<'a, REG> FFIE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt when FULL bit is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FFIE0::Disabled)
    }
    #[doc = "Interrupt generated when FULL bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FFIE0::Enabled)
    }
}
#[doc = "FOVIE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOVIE0 {
    #[doc = "0: No interrupt when FOVR bit is set"]
    Disabled = 0,
    #[doc = "1: Interrupt generated when FOVR bit is set"]
    Enabled = 1,
}
impl From<FOVIE0> for bool {
    #[inline(always)]
    fn from(variant: FOVIE0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOVIE0` reader - FOVIE0"]
pub type FOVIE0_R = crate::BitReader<FOVIE0>;
impl FOVIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FOVIE0 {
        match self.bits {
            false => FOVIE0::Disabled,
            true => FOVIE0::Enabled,
        }
    }
    #[doc = "No interrupt when FOVR bit is set"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FOVIE0::Disabled
    }
    #[doc = "Interrupt generated when FOVR bit is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FOVIE0::Enabled
    }
}
#[doc = "Field `FOVIE0` writer - FOVIE0"]
pub type FOVIE0_W<'a, REG> = crate::BitWriter<'a, REG, FOVIE0>;
impl<'a, REG> FOVIE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt when FOVR bit is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FOVIE0::Disabled)
    }
    #[doc = "Interrupt generated when FOVR bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FOVIE0::Enabled)
    }
}
#[doc = "FMPIE1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMPIE1 {
    #[doc = "0: No interrupt generated when state of FMP\\[1:0\\]
bits are not 00b"]
    Disabled = 0,
    #[doc = "1: Interrupt generated when state of FMP\\[1:0\\]
bits are not 00b"]
    Enabled = 1,
}
impl From<FMPIE1> for bool {
    #[inline(always)]
    fn from(variant: FMPIE1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMPIE1` reader - FMPIE1"]
pub type FMPIE1_R = crate::BitReader<FMPIE1>;
impl FMPIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FMPIE1 {
        match self.bits {
            false => FMPIE1::Disabled,
            true => FMPIE1::Enabled,
        }
    }
    #[doc = "No interrupt generated when state of FMP\\[1:0\\]
bits are not 00b"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FMPIE1::Disabled
    }
    #[doc = "Interrupt generated when state of FMP\\[1:0\\]
bits are not 00b"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FMPIE1::Enabled
    }
}
#[doc = "Field `FMPIE1` writer - FMPIE1"]
pub type FMPIE1_W<'a, REG> = crate::BitWriter<'a, REG, FMPIE1>;
impl<'a, REG> FMPIE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt generated when state of FMP\\[1:0\\]
bits are not 00b"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMPIE1::Disabled)
    }
    #[doc = "Interrupt generated when state of FMP\\[1:0\\]
bits are not 00b"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMPIE1::Enabled)
    }
}
#[doc = "FFIE1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFIE1 {
    #[doc = "0: No interrupt when FULL bit is set"]
    Disabled = 0,
    #[doc = "1: Interrupt generated when FULL bit is set"]
    Enabled = 1,
}
impl From<FFIE1> for bool {
    #[inline(always)]
    fn from(variant: FFIE1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFIE1` reader - FFIE1"]
pub type FFIE1_R = crate::BitReader<FFIE1>;
impl FFIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FFIE1 {
        match self.bits {
            false => FFIE1::Disabled,
            true => FFIE1::Enabled,
        }
    }
    #[doc = "No interrupt when FULL bit is set"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FFIE1::Disabled
    }
    #[doc = "Interrupt generated when FULL bit is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FFIE1::Enabled
    }
}
#[doc = "Field `FFIE1` writer - FFIE1"]
pub type FFIE1_W<'a, REG> = crate::BitWriter<'a, REG, FFIE1>;
impl<'a, REG> FFIE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt when FULL bit is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FFIE1::Disabled)
    }
    #[doc = "Interrupt generated when FULL bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FFIE1::Enabled)
    }
}
#[doc = "FOVIE1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOVIE1 {
    #[doc = "0: No interrupt when FOVR is set"]
    Disabled = 0,
    #[doc = "1: Interrupt generation when FOVR is set"]
    Enabled = 1,
}
impl From<FOVIE1> for bool {
    #[inline(always)]
    fn from(variant: FOVIE1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOVIE1` reader - FOVIE1"]
pub type FOVIE1_R = crate::BitReader<FOVIE1>;
impl FOVIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FOVIE1 {
        match self.bits {
            false => FOVIE1::Disabled,
            true => FOVIE1::Enabled,
        }
    }
    #[doc = "No interrupt when FOVR is set"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FOVIE1::Disabled
    }
    #[doc = "Interrupt generation when FOVR is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FOVIE1::Enabled
    }
}
#[doc = "Field `FOVIE1` writer - FOVIE1"]
pub type FOVIE1_W<'a, REG> = crate::BitWriter<'a, REG, FOVIE1>;
impl<'a, REG> FOVIE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt when FOVR is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FOVIE1::Disabled)
    }
    #[doc = "Interrupt generation when FOVR is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FOVIE1::Enabled)
    }
}
#[doc = "EWGIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWGIE {
    #[doc = "0: ERRI bit will not be set when EWGF is set"]
    Disabled = 0,
    #[doc = "1: ERRI bit will be set when EWGF is set"]
    Enabled = 1,
}
impl From<EWGIE> for bool {
    #[inline(always)]
    fn from(variant: EWGIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWGIE` reader - EWGIE"]
pub type EWGIE_R = crate::BitReader<EWGIE>;
impl EWGIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EWGIE {
        match self.bits {
            false => EWGIE::Disabled,
            true => EWGIE::Enabled,
        }
    }
    #[doc = "ERRI bit will not be set when EWGF is set"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWGIE::Disabled
    }
    #[doc = "ERRI bit will be set when EWGF is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWGIE::Enabled
    }
}
#[doc = "Field `EWGIE` writer - EWGIE"]
pub type EWGIE_W<'a, REG> = crate::BitWriter<'a, REG, EWGIE>;
impl<'a, REG> EWGIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ERRI bit will not be set when EWGF is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWGIE::Disabled)
    }
    #[doc = "ERRI bit will be set when EWGF is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWGIE::Enabled)
    }
}
#[doc = "EPVIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPVIE {
    #[doc = "0: ERRI bit will not be set when EPVF is set"]
    Disabled = 0,
    #[doc = "1: ERRI bit will be set when EPVF is set"]
    Enabled = 1,
}
impl From<EPVIE> for bool {
    #[inline(always)]
    fn from(variant: EPVIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPVIE` reader - EPVIE"]
pub type EPVIE_R = crate::BitReader<EPVIE>;
impl EPVIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPVIE {
        match self.bits {
            false => EPVIE::Disabled,
            true => EPVIE::Enabled,
        }
    }
    #[doc = "ERRI bit will not be set when EPVF is set"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EPVIE::Disabled
    }
    #[doc = "ERRI bit will be set when EPVF is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EPVIE::Enabled
    }
}
#[doc = "Field `EPVIE` writer - EPVIE"]
pub type EPVIE_W<'a, REG> = crate::BitWriter<'a, REG, EPVIE>;
impl<'a, REG> EPVIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ERRI bit will not be set when EPVF is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EPVIE::Disabled)
    }
    #[doc = "ERRI bit will be set when EPVF is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EPVIE::Enabled)
    }
}
#[doc = "BOFIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOFIE {
    #[doc = "0: ERRI bit will not be set when BOFF is set"]
    Disabled = 0,
    #[doc = "1: ERRI bit will be set when BOFF is set"]
    Enabled = 1,
}
impl From<BOFIE> for bool {
    #[inline(always)]
    fn from(variant: BOFIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOFIE` reader - BOFIE"]
pub type BOFIE_R = crate::BitReader<BOFIE>;
impl BOFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BOFIE {
        match self.bits {
            false => BOFIE::Disabled,
            true => BOFIE::Enabled,
        }
    }
    #[doc = "ERRI bit will not be set when BOFF is set"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOFIE::Disabled
    }
    #[doc = "ERRI bit will be set when BOFF is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOFIE::Enabled
    }
}
#[doc = "Field `BOFIE` writer - BOFIE"]
pub type BOFIE_W<'a, REG> = crate::BitWriter<'a, REG, BOFIE>;
impl<'a, REG> BOFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ERRI bit will not be set when BOFF is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOFIE::Disabled)
    }
    #[doc = "ERRI bit will be set when BOFF is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOFIE::Enabled)
    }
}
#[doc = "LECIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LECIE {
    #[doc = "0: ERRI bit will not be set when the error code in LEC\\[2:0\\]
is set by hardware on error detection"]
    Disabled = 0,
    #[doc = "1: ERRI bit will be set when the error code in LEC\\[2:0\\]
is set by hardware on error detection"]
    Enabled = 1,
}
impl From<LECIE> for bool {
    #[inline(always)]
    fn from(variant: LECIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LECIE` reader - LECIE"]
pub type LECIE_R = crate::BitReader<LECIE>;
impl LECIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LECIE {
        match self.bits {
            false => LECIE::Disabled,
            true => LECIE::Enabled,
        }
    }
    #[doc = "ERRI bit will not be set when the error code in LEC\\[2:0\\]
is set by hardware on error detection"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LECIE::Disabled
    }
    #[doc = "ERRI bit will be set when the error code in LEC\\[2:0\\]
is set by hardware on error detection"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LECIE::Enabled
    }
}
#[doc = "Field `LECIE` writer - LECIE"]
pub type LECIE_W<'a, REG> = crate::BitWriter<'a, REG, LECIE>;
impl<'a, REG> LECIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ERRI bit will not be set when the error code in LEC\\[2:0\\]
is set by hardware on error detection"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LECIE::Disabled)
    }
    #[doc = "ERRI bit will be set when the error code in LEC\\[2:0\\]
is set by hardware on error detection"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LECIE::Enabled)
    }
}
#[doc = "ERRIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE {
    #[doc = "0: No interrupt will be generated when an error condition is pending in the CAN_ESR"]
    Disabled = 0,
    #[doc = "1: An interrupt will be generation when an error condition is pending in the CAN_ESR"]
    Enabled = 1,
}
impl From<ERRIE> for bool {
    #[inline(always)]
    fn from(variant: ERRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - ERRIE"]
pub type ERRIE_R = crate::BitReader<ERRIE>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE {
        match self.bits {
            false => ERRIE::Disabled,
            true => ERRIE::Enabled,
        }
    }
    #[doc = "No interrupt will be generated when an error condition is pending in the CAN_ESR"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE::Disabled
    }
    #[doc = "An interrupt will be generation when an error condition is pending in the CAN_ESR"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE::Enabled
    }
}
#[doc = "Field `ERRIE` writer - ERRIE"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt will be generated when an error condition is pending in the CAN_ESR"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Disabled)
    }
    #[doc = "An interrupt will be generation when an error condition is pending in the CAN_ESR"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Enabled)
    }
}
#[doc = "WKUIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUIE {
    #[doc = "0: No interrupt when WKUI is set"]
    Disabled = 0,
    #[doc = "1: Interrupt generated when WKUI bit is set"]
    Enabled = 1,
}
impl From<WKUIE> for bool {
    #[inline(always)]
    fn from(variant: WKUIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUIE` reader - WKUIE"]
pub type WKUIE_R = crate::BitReader<WKUIE>;
impl WKUIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WKUIE {
        match self.bits {
            false => WKUIE::Disabled,
            true => WKUIE::Enabled,
        }
    }
    #[doc = "No interrupt when WKUI is set"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WKUIE::Disabled
    }
    #[doc = "Interrupt generated when WKUI bit is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WKUIE::Enabled
    }
}
#[doc = "Field `WKUIE` writer - WKUIE"]
pub type WKUIE_W<'a, REG> = crate::BitWriter<'a, REG, WKUIE>;
impl<'a, REG> WKUIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt when WKUI is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WKUIE::Disabled)
    }
    #[doc = "Interrupt generated when WKUI bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WKUIE::Enabled)
    }
}
#[doc = "SLKIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLKIE {
    #[doc = "0: No interrupt when SLAKI bit is set"]
    Disabled = 0,
    #[doc = "1: Interrupt generated when SLAKI bit is set"]
    Enabled = 1,
}
impl From<SLKIE> for bool {
    #[inline(always)]
    fn from(variant: SLKIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLKIE` reader - SLKIE"]
pub type SLKIE_R = crate::BitReader<SLKIE>;
impl SLKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLKIE {
        match self.bits {
            false => SLKIE::Disabled,
            true => SLKIE::Enabled,
        }
    }
    #[doc = "No interrupt when SLAKI bit is set"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLKIE::Disabled
    }
    #[doc = "Interrupt generated when SLAKI bit is set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLKIE::Enabled
    }
}
#[doc = "Field `SLKIE` writer - SLKIE"]
pub type SLKIE_W<'a, REG> = crate::BitWriter<'a, REG, SLKIE>;
impl<'a, REG> SLKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt when SLAKI bit is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SLKIE::Disabled)
    }
    #[doc = "Interrupt generated when SLAKI bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SLKIE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - TMEIE"]
    #[inline(always)]
    pub fn tmeie(&self) -> TMEIE_R {
        TMEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FMPIE0"]
    #[inline(always)]
    pub fn fmpie0(&self) -> FMPIE0_R {
        FMPIE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FFIE0"]
    #[inline(always)]
    pub fn ffie0(&self) -> FFIE0_R {
        FFIE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FOVIE0"]
    #[inline(always)]
    pub fn fovie0(&self) -> FOVIE0_R {
        FOVIE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FMPIE1"]
    #[inline(always)]
    pub fn fmpie1(&self) -> FMPIE1_R {
        FMPIE1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FFIE1"]
    #[inline(always)]
    pub fn ffie1(&self) -> FFIE1_R {
        FFIE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FOVIE1"]
    #[inline(always)]
    pub fn fovie1(&self) -> FOVIE1_R {
        FOVIE1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - EWGIE"]
    #[inline(always)]
    pub fn ewgie(&self) -> EWGIE_R {
        EWGIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EPVIE"]
    #[inline(always)]
    pub fn epvie(&self) -> EPVIE_R {
        EPVIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BOFIE"]
    #[inline(always)]
    pub fn bofie(&self) -> BOFIE_R {
        BOFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LECIE"]
    #[inline(always)]
    pub fn lecie(&self) -> LECIE_R {
        LECIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - ERRIE"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - WKUIE"]
    #[inline(always)]
    pub fn wkuie(&self) -> WKUIE_R {
        WKUIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SLKIE"]
    #[inline(always)]
    pub fn slkie(&self) -> SLKIE_R {
        SLKIE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TMEIE"]
    #[inline(always)]
    #[must_use]
    pub fn tmeie(&mut self) -> TMEIE_W<IERrs> {
        TMEIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - FMPIE0"]
    #[inline(always)]
    #[must_use]
    pub fn fmpie0(&mut self) -> FMPIE0_W<IERrs> {
        FMPIE0_W::new(self, 1)
    }
    #[doc = "Bit 2 - FFIE0"]
    #[inline(always)]
    #[must_use]
    pub fn ffie0(&mut self) -> FFIE0_W<IERrs> {
        FFIE0_W::new(self, 2)
    }
    #[doc = "Bit 3 - FOVIE0"]
    #[inline(always)]
    #[must_use]
    pub fn fovie0(&mut self) -> FOVIE0_W<IERrs> {
        FOVIE0_W::new(self, 3)
    }
    #[doc = "Bit 4 - FMPIE1"]
    #[inline(always)]
    #[must_use]
    pub fn fmpie1(&mut self) -> FMPIE1_W<IERrs> {
        FMPIE1_W::new(self, 4)
    }
    #[doc = "Bit 5 - FFIE1"]
    #[inline(always)]
    #[must_use]
    pub fn ffie1(&mut self) -> FFIE1_W<IERrs> {
        FFIE1_W::new(self, 5)
    }
    #[doc = "Bit 6 - FOVIE1"]
    #[inline(always)]
    #[must_use]
    pub fn fovie1(&mut self) -> FOVIE1_W<IERrs> {
        FOVIE1_W::new(self, 6)
    }
    #[doc = "Bit 8 - EWGIE"]
    #[inline(always)]
    #[must_use]
    pub fn ewgie(&mut self) -> EWGIE_W<IERrs> {
        EWGIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - EPVIE"]
    #[inline(always)]
    #[must_use]
    pub fn epvie(&mut self) -> EPVIE_W<IERrs> {
        EPVIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - BOFIE"]
    #[inline(always)]
    #[must_use]
    pub fn bofie(&mut self) -> BOFIE_W<IERrs> {
        BOFIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - LECIE"]
    #[inline(always)]
    #[must_use]
    pub fn lecie(&mut self) -> LECIE_W<IERrs> {
        LECIE_W::new(self, 11)
    }
    #[doc = "Bit 15 - ERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<IERrs> {
        ERRIE_W::new(self, 15)
    }
    #[doc = "Bit 16 - WKUIE"]
    #[inline(always)]
    #[must_use]
    pub fn wkuie(&mut self) -> WKUIE_W<IERrs> {
        WKUIE_W::new(self, 16)
    }
    #[doc = "Bit 17 - SLKIE"]
    #[inline(always)]
    #[must_use]
    pub fn slkie(&mut self) -> SLKIE_W<IERrs> {
        SLKIE_W::new(self, 17)
    }
}
#[doc = "CAN_IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IERrs {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
