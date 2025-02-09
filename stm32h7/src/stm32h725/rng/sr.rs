///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
/**Data ready Note: If IE=1 in RNG_CR, an interrupt is generated when DRDY=1. It can rise when the peripheral is disabled. When the output buffer becomes empty (after reading RNG_DR), this bit returns to 0 until a new random value is generated.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRDY {
    ///0: The RNG_DR register is not yet valid, no random data is available
    Invalid = 0,
    ///1: The RNG_DR register contains valid random data
    Valid = 1,
}
impl From<DRDY> for bool {
    #[inline(always)]
    fn from(variant: DRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `DRDY` reader - Data ready Note: If IE=1 in RNG_CR, an interrupt is generated when DRDY=1. It can rise when the peripheral is disabled. When the output buffer becomes empty (after reading RNG_DR), this bit returns to 0 until a new random value is generated.
pub type DRDY_R = crate::BitReader<DRDY>;
impl DRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DRDY {
        match self.bits {
            false => DRDY::Invalid,
            true => DRDY::Valid,
        }
    }
    ///The RNG_DR register is not yet valid, no random data is available
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == DRDY::Invalid
    }
    ///The RNG_DR register contains valid random data
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == DRDY::Valid
    }
}
/**Clock error current status Note: This bit is meaningless if CED (Clock error detection) bit in RNG_CR is equal to 1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CECSR {
    ///0: The RNG clock is correct (fRNGCLK> fHCLK/32)
    Correct = 0,
    ///1: The RNG clock before internal divider has been detected too slow (fRNGCLK< fHCLK/32)
    Slow = 1,
}
impl From<CECSR> for bool {
    #[inline(always)]
    fn from(variant: CECSR) -> Self {
        variant as u8 != 0
    }
}
///Field `CECS` reader - Clock error current status Note: This bit is meaningless if CED (Clock error detection) bit in RNG_CR is equal to 1.
pub type CECS_R = crate::BitReader<CECSR>;
impl CECS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CECSR {
        match self.bits {
            false => CECSR::Correct,
            true => CECSR::Slow,
        }
    }
    ///The RNG clock is correct (fRNGCLK> fHCLK/32)
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        *self == CECSR::Correct
    }
    ///The RNG clock before internal divider has been detected too slow (fRNGCLK< fHCLK/32)
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == CECSR::Slow
    }
}
/**Seed error current status ** More than 64 consecutive bits at the same value (0 or 1) ** More than 32 consecutive alternances of 0 and 1 (0101010101...01)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECSR {
    ///0: No faulty sequence has currently been detected. If the SEIS bit is set, this means that a faulty sequence was detected and the situation has been recovered
    NoFault = 0,
    ///1: At least one faulty sequence has been detected - see ref manual for details
    Fault = 1,
}
impl From<SECSR> for bool {
    #[inline(always)]
    fn from(variant: SECSR) -> Self {
        variant as u8 != 0
    }
}
///Field `SECS` reader - Seed error current status ** More than 64 consecutive bits at the same value (0 or 1) ** More than 32 consecutive alternances of 0 and 1 (0101010101...01)
pub type SECS_R = crate::BitReader<SECSR>;
impl SECS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SECSR {
        match self.bits {
            false => SECSR::NoFault,
            true => SECSR::Fault,
        }
    }
    ///No faulty sequence has currently been detected. If the SEIS bit is set, this means that a faulty sequence was detected and the situation has been recovered
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == SECSR::NoFault
    }
    ///At least one faulty sequence has been detected - see ref manual for details
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == SECSR::Fault
    }
}
/**Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing it to 0. An interrupt is pending if IE = 1 in the RNG_CR register. Note: This bit is meaningless if CED (Clock error detection) bit in RNG_CR is equal to 1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEIS {
    ///0: The RNG clock is correct (fRNGCLK> fHCLK/32)
    Correct = 0,
    ///1: The RNG clock before internal divider has been detected too slow (fRNGCLK< fHCLK/32)
    Slow = 1,
}
impl From<CEIS> for bool {
    #[inline(always)]
    fn from(variant: CEIS) -> Self {
        variant as u8 != 0
    }
}
///Field `CEIS` reader - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing it to 0. An interrupt is pending if IE = 1 in the RNG_CR register. Note: This bit is meaningless if CED (Clock error detection) bit in RNG_CR is equal to 1.
pub type CEIS_R = crate::BitReader<CEIS>;
impl CEIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CEIS {
        match self.bits {
            false => CEIS::Correct,
            true => CEIS::Slow,
        }
    }
    ///The RNG clock is correct (fRNGCLK> fHCLK/32)
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        *self == CEIS::Correct
    }
    ///The RNG clock before internal divider has been detected too slow (fRNGCLK< fHCLK/32)
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == CEIS::Slow
    }
}
///Field `CEIS` writer - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing it to 0. An interrupt is pending if IE = 1 in the RNG_CR register. Note: This bit is meaningless if CED (Clock error detection) bit in RNG_CR is equal to 1.
pub type CEIS_W<'a, REG> = crate::BitWriter<'a, REG, CEIS>;
impl<'a, REG> CEIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The RNG clock is correct (fRNGCLK> fHCLK/32)
    #[inline(always)]
    pub fn correct(self) -> &'a mut crate::W<REG> {
        self.variant(CEIS::Correct)
    }
    ///The RNG clock before internal divider has been detected too slow (fRNGCLK< fHCLK/32)
    #[inline(always)]
    pub fn slow(self) -> &'a mut crate::W<REG> {
        self.variant(CEIS::Slow)
    }
}
/**Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing it to 0. ** More than 64 consecutive bits at the same value (0 or 1) ** More than 32 consecutive alternances of 0 and 1 (0101010101...01) An interrupt is pending if IE = 1 in the RNG_CR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEIS {
    ///0: No faulty sequence detected
    NoFault = 0,
    ///1: At least one faulty sequence has been detected
    Fault = 1,
}
impl From<SEIS> for bool {
    #[inline(always)]
    fn from(variant: SEIS) -> Self {
        variant as u8 != 0
    }
}
///Field `SEIS` reader - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing it to 0. ** More than 64 consecutive bits at the same value (0 or 1) ** More than 32 consecutive alternances of 0 and 1 (0101010101...01) An interrupt is pending if IE = 1 in the RNG_CR register.
pub type SEIS_R = crate::BitReader<SEIS>;
impl SEIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SEIS {
        match self.bits {
            false => SEIS::NoFault,
            true => SEIS::Fault,
        }
    }
    ///No faulty sequence detected
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == SEIS::NoFault
    }
    ///At least one faulty sequence has been detected
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == SEIS::Fault
    }
}
///Field `SEIS` writer - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing it to 0. ** More than 64 consecutive bits at the same value (0 or 1) ** More than 32 consecutive alternances of 0 and 1 (0101010101...01) An interrupt is pending if IE = 1 in the RNG_CR register.
pub type SEIS_W<'a, REG> = crate::BitWriter<'a, REG, SEIS>;
impl<'a, REG> SEIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No faulty sequence detected
    #[inline(always)]
    pub fn no_fault(self) -> &'a mut crate::W<REG> {
        self.variant(SEIS::NoFault)
    }
    ///At least one faulty sequence has been detected
    #[inline(always)]
    pub fn fault(self) -> &'a mut crate::W<REG> {
        self.variant(SEIS::Fault)
    }
}
impl R {
    ///Bit 0 - Data ready Note: If IE=1 in RNG_CR, an interrupt is generated when DRDY=1. It can rise when the peripheral is disabled. When the output buffer becomes empty (after reading RNG_DR), this bit returns to 0 until a new random value is generated.
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clock error current status Note: This bit is meaningless if CED (Clock error detection) bit in RNG_CR is equal to 1.
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Seed error current status ** More than 64 consecutive bits at the same value (0 or 1) ** More than 32 consecutive alternances of 0 and 1 (0101010101...01)
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing it to 0. An interrupt is pending if IE = 1 in the RNG_CR register. Note: This bit is meaningless if CED (Clock error detection) bit in RNG_CR is equal to 1.
    #[inline(always)]
    pub fn ceis(&self) -> CEIS_R {
        CEIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing it to 0. ** More than 64 consecutive bits at the same value (0 or 1) ** More than 32 consecutive alternances of 0 and 1 (0101010101...01) An interrupt is pending if IE = 1 in the RNG_CR register.
    #[inline(always)]
    pub fn seis(&self) -> SEIS_R {
        SEIS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("drdy", &self.drdy())
            .field("cecs", &self.cecs())
            .field("secs", &self.secs())
            .field("ceis", &self.ceis())
            .field("seis", &self.seis())
            .finish()
    }
}
impl W {
    ///Bit 5 - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing it to 0. An interrupt is pending if IE = 1 in the RNG_CR register. Note: This bit is meaningless if CED (Clock error detection) bit in RNG_CR is equal to 1.
    #[inline(always)]
    pub fn ceis(&mut self) -> CEIS_W<SRrs> {
        CEIS_W::new(self, 5)
    }
    ///Bit 6 - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing it to 0. ** More than 64 consecutive bits at the same value (0 or 1) ** More than 32 consecutive alternances of 0 and 1 (0101010101...01) An interrupt is pending if IE = 1 in the RNG_CR register.
    #[inline(always)]
    pub fn seis(&mut self) -> SEIS_W<SRrs> {
        SEIS_W::new(self, 6)
    }
}
/**RNG status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#RNG:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
