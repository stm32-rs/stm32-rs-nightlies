#[doc = "Register `ECCR` reader"]
pub type R = crate::R<ECCRrs>;
#[doc = "Register `ECCR` writer"]
pub type W = crate::W<ECCRrs>;
#[doc = "Field `ADDR_ECC` reader - ECC fail address"]
pub type ADDR_ECC_R = crate::FieldReader<u32>;
#[doc = "ECC fail bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK_ECC {
    #[doc = "0: Bank 1"]
    Bank1 = 0,
    #[doc = "1: Bank 2"]
    Bank2 = 1,
}
impl From<BK_ECC> for bool {
    #[inline(always)]
    fn from(variant: BK_ECC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK_ECC` reader - ECC fail bank"]
pub type BK_ECC_R = crate::BitReader<BK_ECC>;
impl BK_ECC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK_ECC {
        match self.bits {
            false => BK_ECC::Bank1,
            true => BK_ECC::Bank2,
        }
    }
    #[doc = "Bank 1"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == BK_ECC::Bank1
    }
    #[doc = "Bank 2"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == BK_ECC::Bank2
    }
}
#[doc = "System Flash ECC fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSF_ECC {
    #[doc = "1: This bit indicates that the ECC error correction or double ECC error detection is located in the System Flash"]
    InSystemFlash = 1,
}
impl From<SYSF_ECC> for bool {
    #[inline(always)]
    fn from(variant: SYSF_ECC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSF_ECC` reader - System Flash ECC fail"]
pub type SYSF_ECC_R = crate::BitReader<SYSF_ECC>;
impl SYSF_ECC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSF_ECC> {
        match self.bits {
            true => Some(SYSF_ECC::InSystemFlash),
            _ => None,
        }
    }
    #[doc = "This bit indicates that the ECC error correction or double ECC error detection is located in the System Flash"]
    #[inline(always)]
    pub fn is_in_system_flash(&self) -> bool {
        *self == SYSF_ECC::InSystemFlash
    }
}
#[doc = "ECC correction interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCIE {
    #[doc = "0: ECCC interrupt disabled"]
    Disabled = 0,
    #[doc = "1: ECCC interrupt enabled"]
    Enabled = 1,
}
impl From<ECCIE> for bool {
    #[inline(always)]
    fn from(variant: ECCIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCIE` reader - ECC correction interrupt enable"]
pub type ECCIE_R = crate::BitReader<ECCIE>;
impl ECCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECCIE {
        match self.bits {
            false => ECCIE::Disabled,
            true => ECCIE::Enabled,
        }
    }
    #[doc = "ECCC interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ECCIE::Disabled
    }
    #[doc = "ECCC interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ECCIE::Enabled
    }
}
#[doc = "Field `ECCIE` writer - ECC correction interrupt enable"]
pub type ECCIE_W<'a, REG> = crate::BitWriter<'a, REG, ECCIE>;
impl<'a, REG> ECCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECCC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ECCIE::Disabled)
    }
    #[doc = "ECCC interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ECCIE::Enabled)
    }
}
#[doc = "ECC2 correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCC2R {
    #[doc = "0: No ECC error detected on MSB"]
    NoError = 0,
    #[doc = "1: Set by hardware when one ECC errors have been detected and corrected on MSB"]
    Error = 1,
}
impl From<ECCC2R> for bool {
    #[inline(always)]
    fn from(variant: ECCC2R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCC2` reader - ECC2 correction"]
pub type ECCC2_R = crate::BitReader<ECCC2R>;
impl ECCC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECCC2R {
        match self.bits {
            false => ECCC2R::NoError,
            true => ECCC2R::Error,
        }
    }
    #[doc = "No ECC error detected on MSB"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ECCC2R::NoError
    }
    #[doc = "Set by hardware when one ECC errors have been detected and corrected on MSB"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ECCC2R::Error
    }
}
#[doc = "ECC2 correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCC2W {
    #[doc = "1: Cleared by writing 1"]
    Clear = 1,
}
impl From<ECCC2W> for bool {
    #[inline(always)]
    fn from(variant: ECCC2W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCC2` writer - ECC2 correction"]
pub type ECCC2_W<'a, REG> = crate::BitWriter<'a, REG, ECCC2W>;
impl<'a, REG> ECCC2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ECCC2W::Clear)
    }
}
#[doc = "ECC2 detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCD2R {
    #[doc = "0: No double ECC errors detected on MSB"]
    NoError = 0,
    #[doc = "1: Set by hardware when two ECC errors have been detected on MSB"]
    Error = 1,
}
impl From<ECCD2R> for bool {
    #[inline(always)]
    fn from(variant: ECCD2R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCD2` reader - ECC2 detection"]
pub type ECCD2_R = crate::BitReader<ECCD2R>;
impl ECCD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECCD2R {
        match self.bits {
            false => ECCD2R::NoError,
            true => ECCD2R::Error,
        }
    }
    #[doc = "No double ECC errors detected on MSB"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ECCD2R::NoError
    }
    #[doc = "Set by hardware when two ECC errors have been detected on MSB"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ECCD2R::Error
    }
}
#[doc = "ECC2 detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCD2W {
    #[doc = "1: Cleared by writing 1"]
    Clear = 1,
}
impl From<ECCD2W> for bool {
    #[inline(always)]
    fn from(variant: ECCD2W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCD2` writer - ECC2 detection"]
pub type ECCD2_W<'a, REG> = crate::BitWriter<'a, REG, ECCD2W>;
impl<'a, REG> ECCD2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ECCD2W::Clear)
    }
}
#[doc = "ECC correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCCR {
    #[doc = "0: No ECC error detected on LSB"]
    NoError = 0,
    #[doc = "1: Set by hardware when one ECC errors have been detected and corrected on LSB"]
    Error = 1,
}
impl From<ECCCR> for bool {
    #[inline(always)]
    fn from(variant: ECCCR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCC` reader - ECC correction"]
pub type ECCC_R = crate::BitReader<ECCCR>;
impl ECCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECCCR {
        match self.bits {
            false => ECCCR::NoError,
            true => ECCCR::Error,
        }
    }
    #[doc = "No ECC error detected on LSB"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ECCCR::NoError
    }
    #[doc = "Set by hardware when one ECC errors have been detected and corrected on LSB"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ECCCR::Error
    }
}
#[doc = "ECC correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCCW {
    #[doc = "1: Cleared by writing 1"]
    Clear = 1,
}
impl From<ECCCW> for bool {
    #[inline(always)]
    fn from(variant: ECCCW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCC` writer - ECC correction"]
pub type ECCC_W<'a, REG> = crate::BitWriter<'a, REG, ECCCW>;
impl<'a, REG> ECCC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ECCCW::Clear)
    }
}
#[doc = "ECC detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCDR {
    #[doc = "0: No double ECC errors detected on LSB"]
    NoError = 0,
    #[doc = "1: Set by hardware when two ECC errors have been detected on LSB"]
    Error = 1,
}
impl From<ECCDR> for bool {
    #[inline(always)]
    fn from(variant: ECCDR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCD` reader - ECC detection"]
pub type ECCD_R = crate::BitReader<ECCDR>;
impl ECCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECCDR {
        match self.bits {
            false => ECCDR::NoError,
            true => ECCDR::Error,
        }
    }
    #[doc = "No double ECC errors detected on LSB"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ECCDR::NoError
    }
    #[doc = "Set by hardware when two ECC errors have been detected on LSB"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ECCDR::Error
    }
}
#[doc = "ECC detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCDW {
    #[doc = "1: Cleared by writing 1"]
    Clear = 1,
}
impl From<ECCDW> for bool {
    #[inline(always)]
    fn from(variant: ECCDW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCD` writer - ECC detection"]
pub type ECCD_W<'a, REG> = crate::BitWriter<'a, REG, ECCDW>;
impl<'a, REG> ECCD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cleared by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ECCDW::Clear)
    }
}
impl R {
    #[doc = "Bits 0:20 - ECC fail address"]
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new(self.bits & 0x001f_ffff)
    }
    #[doc = "Bit 19 - ECC fail bank"]
    #[inline(always)]
    pub fn bk_ecc(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - System Flash ECC fail"]
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - ECC correction interrupt enable"]
    #[inline(always)]
    pub fn eccie(&self) -> ECCIE_R {
        ECCIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - ECC2 correction"]
    #[inline(always)]
    pub fn eccc2(&self) -> ECCC2_R {
        ECCC2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ECC2 detection"]
    #[inline(always)]
    pub fn eccd2(&self) -> ECCD2_R {
        ECCD2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - ECC correction interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eccie(&mut self) -> ECCIE_W<ECCRrs> {
        ECCIE_W::new(self, 24)
    }
    #[doc = "Bit 28 - ECC2 correction"]
    #[inline(always)]
    #[must_use]
    pub fn eccc2(&mut self) -> ECCC2_W<ECCRrs> {
        ECCC2_W::new(self, 28)
    }
    #[doc = "Bit 29 - ECC2 detection"]
    #[inline(always)]
    #[must_use]
    pub fn eccd2(&mut self) -> ECCD2_W<ECCRrs> {
        ECCD2_W::new(self, 29)
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    #[must_use]
    pub fn eccc(&mut self) -> ECCC_W<ECCRrs> {
        ECCC_W::new(self, 30)
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    #[must_use]
    pub fn eccd(&mut self) -> ECCD_W<ECCRrs> {
        ECCD_W::new(self, 31)
    }
}
#[doc = "Flash ECC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCRrs;
impl crate::RegisterSpec for ECCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccr::R`](R) reader structure"]
impl crate::Readable for ECCRrs {}
#[doc = "`write(|w| ..)` method takes [`eccr::W`](W) writer structure"]
impl crate::Writable for ECCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCR to value 0"]
impl crate::Resettable for ECCRrs {
    const RESET_VALUE: u32 = 0;
}
