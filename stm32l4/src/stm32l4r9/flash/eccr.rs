///Register `ECCR` reader
pub type R = crate::R<ECCRrs>;
///Register `ECCR` writer
pub type W = crate::W<ECCRrs>;
///Field `ADDR_ECC` reader - ECC fail address
pub type ADDR_ECC_R = crate::FieldReader<u32>;
/**ECC fail bank

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK_ECC {
    ///0: Bank 1
    Bank1 = 0,
    ///1: Bank 2
    Bank2 = 1,
}
impl From<BK_ECC> for bool {
    #[inline(always)]
    fn from(variant: BK_ECC) -> Self {
        variant as u8 != 0
    }
}
///Field `BK_ECC` reader - ECC fail bank
pub type BK_ECC_R = crate::BitReader<BK_ECC>;
impl BK_ECC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK_ECC {
        match self.bits {
            false => BK_ECC::Bank1,
            true => BK_ECC::Bank2,
        }
    }
    ///Bank 1
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == BK_ECC::Bank1
    }
    ///Bank 2
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == BK_ECC::Bank2
    }
}
/**System Flash ECC fail

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSF_ECC {
    ///1: This bit indicates that the ECC error correction or double ECC error detection is located in the System Flash
    InSystemFlash = 1,
}
impl From<SYSF_ECC> for bool {
    #[inline(always)]
    fn from(variant: SYSF_ECC) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSF_ECC` reader - System Flash ECC fail
pub type SYSF_ECC_R = crate::BitReader<SYSF_ECC>;
impl SYSF_ECC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSF_ECC> {
        match self.bits {
            true => Some(SYSF_ECC::InSystemFlash),
            _ => None,
        }
    }
    ///This bit indicates that the ECC error correction or double ECC error detection is located in the System Flash
    #[inline(always)]
    pub fn is_in_system_flash(&self) -> bool {
        *self == SYSF_ECC::InSystemFlash
    }
}
/**ECC correction interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCIE {
    ///0: ECCC interrupt disabled
    Disabled = 0,
    ///1: ECCC interrupt enabled
    Enabled = 1,
}
impl From<ECCIE> for bool {
    #[inline(always)]
    fn from(variant: ECCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCIE` reader - ECC correction interrupt enable
pub type ECCIE_R = crate::BitReader<ECCIE>;
impl ECCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECCIE {
        match self.bits {
            false => ECCIE::Disabled,
            true => ECCIE::Enabled,
        }
    }
    ///ECCC interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ECCIE::Disabled
    }
    ///ECCC interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ECCIE::Enabled
    }
}
///Field `ECCIE` writer - ECC correction interrupt enable
pub type ECCIE_W<'a, REG> = crate::BitWriter<'a, REG, ECCIE>;
impl<'a, REG> ECCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ECCC interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ECCIE::Disabled)
    }
    ///ECCC interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ECCIE::Enabled)
    }
}
/**ECC2 correction

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCC2R {
    ///0: No ECC error detected on MSB
    NoError = 0,
    ///1: Set by hardware when one ECC errors have been detected and corrected on MSB
    Error = 1,
}
impl From<ECCC2R> for bool {
    #[inline(always)]
    fn from(variant: ECCC2R) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCC2` reader - ECC2 correction
pub type ECCC2_R = crate::BitReader<ECCC2R>;
impl ECCC2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECCC2R {
        match self.bits {
            false => ECCC2R::NoError,
            true => ECCC2R::Error,
        }
    }
    ///No ECC error detected on MSB
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ECCC2R::NoError
    }
    ///Set by hardware when one ECC errors have been detected and corrected on MSB
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ECCC2R::Error
    }
}
/**ECC2 correction

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCC2W {
    ///1: Cleared by writing 1
    Clear = 1,
}
impl From<ECCC2W> for bool {
    #[inline(always)]
    fn from(variant: ECCC2W) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCC2` writer - ECC2 correction
pub type ECCC2_W<'a, REG> = crate::BitWriter<'a, REG, ECCC2W>;
impl<'a, REG> ECCC2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cleared by writing 1
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ECCC2W::Clear)
    }
}
/**ECC2 detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCD2R {
    ///0: No double ECC errors detected on MSB
    NoError = 0,
    ///1: Set by hardware when two ECC errors have been detected on MSB
    Error = 1,
}
impl From<ECCD2R> for bool {
    #[inline(always)]
    fn from(variant: ECCD2R) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCD2` reader - ECC2 detection
pub type ECCD2_R = crate::BitReader<ECCD2R>;
impl ECCD2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECCD2R {
        match self.bits {
            false => ECCD2R::NoError,
            true => ECCD2R::Error,
        }
    }
    ///No double ECC errors detected on MSB
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ECCD2R::NoError
    }
    ///Set by hardware when two ECC errors have been detected on MSB
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ECCD2R::Error
    }
}
/**ECC2 detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCD2W {
    ///1: Cleared by writing 1
    Clear = 1,
}
impl From<ECCD2W> for bool {
    #[inline(always)]
    fn from(variant: ECCD2W) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCD2` writer - ECC2 detection
pub type ECCD2_W<'a, REG> = crate::BitWriter<'a, REG, ECCD2W>;
impl<'a, REG> ECCD2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cleared by writing 1
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ECCD2W::Clear)
    }
}
/**ECC correction

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCCR {
    ///0: No ECC error detected on LSB
    NoError = 0,
    ///1: Set by hardware when one ECC errors have been detected and corrected on LSB
    Error = 1,
}
impl From<ECCCR> for bool {
    #[inline(always)]
    fn from(variant: ECCCR) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCC` reader - ECC correction
pub type ECCC_R = crate::BitReader<ECCCR>;
impl ECCC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECCCR {
        match self.bits {
            false => ECCCR::NoError,
            true => ECCCR::Error,
        }
    }
    ///No ECC error detected on LSB
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ECCCR::NoError
    }
    ///Set by hardware when one ECC errors have been detected and corrected on LSB
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ECCCR::Error
    }
}
/**ECC correction

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCCW {
    ///1: Cleared by writing 1
    Clear = 1,
}
impl From<ECCCW> for bool {
    #[inline(always)]
    fn from(variant: ECCCW) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCC` writer - ECC correction
pub type ECCC_W<'a, REG> = crate::BitWriter<'a, REG, ECCCW>;
impl<'a, REG> ECCC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cleared by writing 1
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ECCCW::Clear)
    }
}
/**ECC detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCDR {
    ///0: No double ECC errors detected on LSB
    NoError = 0,
    ///1: Set by hardware when two ECC errors have been detected on LSB
    Error = 1,
}
impl From<ECCDR> for bool {
    #[inline(always)]
    fn from(variant: ECCDR) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCD` reader - ECC detection
pub type ECCD_R = crate::BitReader<ECCDR>;
impl ECCD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECCDR {
        match self.bits {
            false => ECCDR::NoError,
            true => ECCDR::Error,
        }
    }
    ///No double ECC errors detected on LSB
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ECCDR::NoError
    }
    ///Set by hardware when two ECC errors have been detected on LSB
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ECCDR::Error
    }
}
/**ECC detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCDW {
    ///1: Cleared by writing 1
    Clear = 1,
}
impl From<ECCDW> for bool {
    #[inline(always)]
    fn from(variant: ECCDW) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCD` writer - ECC detection
pub type ECCD_W<'a, REG> = crate::BitWriter<'a, REG, ECCDW>;
impl<'a, REG> ECCD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cleared by writing 1
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ECCDW::Clear)
    }
}
impl R {
    ///Bits 0:20 - ECC fail address
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new(self.bits & 0x001f_ffff)
    }
    ///Bit 19 - ECC fail bank
    #[inline(always)]
    pub fn bk_ecc(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - System Flash ECC fail
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - ECC correction interrupt enable
    #[inline(always)]
    pub fn eccie(&self) -> ECCIE_R {
        ECCIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - ECC2 correction
    #[inline(always)]
    pub fn eccc2(&self) -> ECCC2_R {
        ECCC2_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ECC2 detection
    #[inline(always)]
    pub fn eccd2(&self) -> ECCD2_R {
        ECCD2_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - ECC correction
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ECC detection
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCR")
            .field("addr_ecc", &self.addr_ecc())
            .field("bk_ecc", &self.bk_ecc())
            .field("sysf_ecc", &self.sysf_ecc())
            .field("eccie", &self.eccie())
            .field("eccc", &self.eccc())
            .field("eccd", &self.eccd())
            .field("eccd2", &self.eccd2())
            .field("eccc2", &self.eccc2())
            .finish()
    }
}
impl W {
    ///Bit 24 - ECC correction interrupt enable
    #[inline(always)]
    pub fn eccie(&mut self) -> ECCIE_W<'_, ECCRrs> {
        ECCIE_W::new(self, 24)
    }
    ///Bit 28 - ECC2 correction
    #[inline(always)]
    pub fn eccc2(&mut self) -> ECCC2_W<'_, ECCRrs> {
        ECCC2_W::new(self, 28)
    }
    ///Bit 29 - ECC2 detection
    #[inline(always)]
    pub fn eccd2(&mut self) -> ECCD2_W<'_, ECCRrs> {
        ECCD2_W::new(self, 29)
    }
    ///Bit 30 - ECC correction
    #[inline(always)]
    pub fn eccc(&mut self) -> ECCC_W<'_, ECCRrs> {
        ECCC_W::new(self, 30)
    }
    ///Bit 31 - ECC detection
    #[inline(always)]
    pub fn eccd(&mut self) -> ECCD_W<'_, ECCRrs> {
        ECCD_W::new(self, 31)
    }
}
/**Flash ECC register

You can [`read`](crate::Reg::read) this register and get [`eccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#FLASH:ECCR)*/
pub struct ECCRrs;
impl crate::RegisterSpec for ECCRrs {
    type Ux = u32;
}
///`read()` method returns [`eccr::R`](R) reader structure
impl crate::Readable for ECCRrs {}
///`write(|w| ..)` method takes [`eccr::W`](W) writer structure
impl crate::Writable for ECCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECCR to value 0
impl crate::Resettable for ECCRrs {}
