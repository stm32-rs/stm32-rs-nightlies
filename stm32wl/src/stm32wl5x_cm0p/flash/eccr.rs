///Register `ECCR` reader
pub type R = crate::R<ECCRrs>;
///Register `ECCR` writer
pub type W = crate::W<ECCRrs>;
///Field `ADDR_ECC` reader - ECC fail address
pub type ADDR_ECC_R = crate::FieldReader<u32>;
/**System Flash ECC fail

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSF_ECC {
    ///0: No System Flash memory ECC fail
    NotInFlash = 0,
    ///1: System Flash memory ECC fail
    InFlash = 1,
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
    pub const fn variant(&self) -> SYSF_ECC {
        match self.bits {
            false => SYSF_ECC::NotInFlash,
            true => SYSF_ECC::InFlash,
        }
    }
    ///No System Flash memory ECC fail
    #[inline(always)]
    pub fn is_not_in_flash(&self) -> bool {
        *self == SYSF_ECC::NotInFlash
    }
    ///System Flash memory ECC fail
    #[inline(always)]
    pub fn is_in_flash(&self) -> bool {
        *self == SYSF_ECC::InFlash
    }
}
/**ECC correction interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCCIE {
    ///0: ECCC interrupt disabled
    Disabled = 0,
    ///1: ECCC interrupt enabled
    Enabled = 1,
}
impl From<ECCCIE> for bool {
    #[inline(always)]
    fn from(variant: ECCCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCCIE` reader - ECC correction interrupt enable
pub type ECCCIE_R = crate::BitReader<ECCCIE>;
impl ECCCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECCCIE {
        match self.bits {
            false => ECCCIE::Disabled,
            true => ECCCIE::Enabled,
        }
    }
    ///ECCC interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ECCCIE::Disabled
    }
    ///ECCC interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ECCCIE::Enabled
    }
}
///Field `ECCCIE` writer - ECC correction interrupt enable
pub type ECCCIE_W<'a, REG> = crate::BitWriter<'a, REG, ECCCIE>;
impl<'a, REG> ECCCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ECCC interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ECCCIE::Disabled)
    }
    ///ECCC interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ECCCIE::Enabled)
    }
}
///Field `CPUID` reader - CPU identification
pub type CPUID_R = crate::FieldReader;
/**ECC correction

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCCR {
    ///0: ECC error corrected
    NoEvent = 0,
    ///1: No ECC error corrected
    Event = 1,
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
            false => ECCCR::NoEvent,
            true => ECCCR::Event,
        }
    }
    ///ECC error corrected
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == ECCCR::NoEvent
    }
    ///No ECC error corrected
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == ECCCR::Event
    }
}
/**ECC correction

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCCW {
    ///1: Clear the flag
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
    ///Clear the flag
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
    ///0: Two ECC errors detected
    NoEvent = 0,
    ///1: No two ECC errors detected
    Event = 1,
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
            false => ECCDR::NoEvent,
            true => ECCDR::Event,
        }
    }
    ///Two ECC errors detected
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == ECCDR::NoEvent
    }
    ///No two ECC errors detected
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == ECCDR::Event
    }
}
/**ECC detection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCDW {
    ///1: Clear the flag
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
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ECCDW::Clear)
    }
}
impl R {
    ///Bits 0:16 - ECC fail address
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new(self.bits & 0x0001_ffff)
    }
    ///Bit 20 - System Flash ECC fail
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - ECC correction interrupt enable
    #[inline(always)]
    pub fn ecccie(&self) -> ECCCIE_R {
        ECCCIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 26:28 - CPU identification
    #[inline(always)]
    pub fn cpuid(&self) -> CPUID_R {
        CPUID_R::new(((self.bits >> 26) & 7) as u8)
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
            .field("sysf_ecc", &self.sysf_ecc())
            .field("ecccie", &self.ecccie())
            .field("cpuid", &self.cpuid())
            .field("eccc", &self.eccc())
            .field("eccd", &self.eccd())
            .finish()
    }
}
impl W {
    ///Bit 24 - ECC correction interrupt enable
    #[inline(always)]
    pub fn ecccie(&mut self) -> ECCCIE_W<'_, ECCRrs> {
        ECCCIE_W::new(self, 24)
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#FLASH:ECCR)*/
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
