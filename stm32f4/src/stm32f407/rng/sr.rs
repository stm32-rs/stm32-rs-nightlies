///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
/**Data ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRDY {
    ///0: The RNG_DR register is not yet valid, no random data is available
    NotValid = 0,
    ///1: The RNG_DR register contains valid random data. Once the RNG_DR register has been read, this bit returns to 0 until a new random value is generated.
    Valid = 1,
}
impl From<DRDY> for bool {
    #[inline(always)]
    fn from(variant: DRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `DRDY` reader - Data ready
pub type DRDY_R = crate::BitReader<DRDY>;
impl DRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DRDY {
        match self.bits {
            false => DRDY::NotValid,
            true => DRDY::Valid,
        }
    }
    ///The RNG_DR register is not yet valid, no random data is available
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == DRDY::NotValid
    }
    ///The RNG_DR register contains valid random data. Once the RNG_DR register has been read, this bit returns to 0 until a new random value is generated.
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == DRDY::Valid
    }
}
/**Clock error current status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CECS {
    ///0: The RNG clock is correct (fRNGCLK> fHCLK/16). If the CEIS bit is set, this means that a slow clock was detected and the situation has been recovered.
    Correct = 0,
    ///1: The RNG clock is too slow (fRNGCLK< fHCLK/16)
    TooSlow = 1,
}
impl From<CECS> for bool {
    #[inline(always)]
    fn from(variant: CECS) -> Self {
        variant as u8 != 0
    }
}
///Field `CECS` reader - Clock error current status
pub type CECS_R = crate::BitReader<CECS>;
impl CECS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CECS {
        match self.bits {
            false => CECS::Correct,
            true => CECS::TooSlow,
        }
    }
    ///The RNG clock is correct (fRNGCLK> fHCLK/16). If the CEIS bit is set, this means that a slow clock was detected and the situation has been recovered.
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        *self == CECS::Correct
    }
    ///The RNG clock is too slow (fRNGCLK< fHCLK/16)
    #[inline(always)]
    pub fn is_too_slow(&self) -> bool {
        *self == CECS::TooSlow
    }
}
/**Seed error current status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECS {
    ///0: No faulty sequence has currently been detected. If the SEIS bit is set, this means that a faulty sequence was detected and the situation has been recovered.
    NoFaulty = 0,
    ///1: One of the noise source has provided more than 64 consecutive bits at a constant value (“0” or “1”), or more than 32 consecutive occurrence of two bits patterns (“01” or “10”)
    Faulty = 1,
}
impl From<SECS> for bool {
    #[inline(always)]
    fn from(variant: SECS) -> Self {
        variant as u8 != 0
    }
}
///Field `SECS` reader - Seed error current status
pub type SECS_R = crate::BitReader<SECS>;
impl SECS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SECS {
        match self.bits {
            false => SECS::NoFaulty,
            true => SECS::Faulty,
        }
    }
    ///No faulty sequence has currently been detected. If the SEIS bit is set, this means that a faulty sequence was detected and the situation has been recovered.
    #[inline(always)]
    pub fn is_no_faulty(&self) -> bool {
        *self == SECS::NoFaulty
    }
    ///One of the noise source has provided more than 64 consecutive bits at a constant value (“0” or “1”), or more than 32 consecutive occurrence of two bits patterns (“01” or “10”)
    #[inline(always)]
    pub fn is_faulty(&self) -> bool {
        *self == SECS::Faulty
    }
}
/**Clock error interrupt status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEIS {
    ///0: The RNG clock is correct (fRNGCLK > fHCLK/16)
    Correct = 0,
    ///1: The RNG has been detected too slow (fRNGCLK < fHCLK/16) An interrupt is pending if IE = 1 in the RNG_CR register
    TooSlow = 1,
}
impl From<CEIS> for bool {
    #[inline(always)]
    fn from(variant: CEIS) -> Self {
        variant as u8 != 0
    }
}
///Field `CEIS` reader - Clock error interrupt status
pub type CEIS_R = crate::BitReader<CEIS>;
impl CEIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CEIS {
        match self.bits {
            false => CEIS::Correct,
            true => CEIS::TooSlow,
        }
    }
    ///The RNG clock is correct (fRNGCLK > fHCLK/16)
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        *self == CEIS::Correct
    }
    ///The RNG has been detected too slow (fRNGCLK < fHCLK/16) An interrupt is pending if IE = 1 in the RNG_CR register
    #[inline(always)]
    pub fn is_too_slow(&self) -> bool {
        *self == CEIS::TooSlow
    }
}
///Field `CEIS` writer - Clock error interrupt status
pub type CEIS_W<'a, REG> = crate::BitWriter<'a, REG, CEIS>;
impl<'a, REG> CEIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The RNG clock is correct (fRNGCLK > fHCLK/16)
    #[inline(always)]
    pub fn correct(self) -> &'a mut crate::W<REG> {
        self.variant(CEIS::Correct)
    }
    ///The RNG has been detected too slow (fRNGCLK < fHCLK/16) An interrupt is pending if IE = 1 in the RNG_CR register
    #[inline(always)]
    pub fn too_slow(self) -> &'a mut crate::W<REG> {
        self.variant(CEIS::TooSlow)
    }
}
/**Seed error interrupt status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEIS {
    ///0: No faulty sequence detected
    NoFaulty = 0,
    ///1: At least one faulty sequence has been detected. See **SECS** bit description for details. An interrupt is pending if IE = 1 in the RNG_CR register.
    Faulty = 1,
}
impl From<SEIS> for bool {
    #[inline(always)]
    fn from(variant: SEIS) -> Self {
        variant as u8 != 0
    }
}
///Field `SEIS` reader - Seed error interrupt status
pub type SEIS_R = crate::BitReader<SEIS>;
impl SEIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SEIS {
        match self.bits {
            false => SEIS::NoFaulty,
            true => SEIS::Faulty,
        }
    }
    ///No faulty sequence detected
    #[inline(always)]
    pub fn is_no_faulty(&self) -> bool {
        *self == SEIS::NoFaulty
    }
    ///At least one faulty sequence has been detected. See **SECS** bit description for details. An interrupt is pending if IE = 1 in the RNG_CR register.
    #[inline(always)]
    pub fn is_faulty(&self) -> bool {
        *self == SEIS::Faulty
    }
}
///Field `SEIS` writer - Seed error interrupt status
pub type SEIS_W<'a, REG> = crate::BitWriter<'a, REG, SEIS>;
impl<'a, REG> SEIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No faulty sequence detected
    #[inline(always)]
    pub fn no_faulty(self) -> &'a mut crate::W<REG> {
        self.variant(SEIS::NoFaulty)
    }
    ///At least one faulty sequence has been detected. See **SECS** bit description for details. An interrupt is pending if IE = 1 in the RNG_CR register.
    #[inline(always)]
    pub fn faulty(self) -> &'a mut crate::W<REG> {
        self.variant(SEIS::Faulty)
    }
}
impl R {
    ///Bit 0 - Data ready
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clock error current status
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Seed error current status
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - Clock error interrupt status
    #[inline(always)]
    pub fn ceis(&self) -> CEIS_R {
        CEIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Seed error interrupt status
    #[inline(always)]
    pub fn seis(&self) -> SEIS_R {
        SEIS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("seis", &self.seis())
            .field("ceis", &self.ceis())
            .field("secs", &self.secs())
            .field("cecs", &self.cecs())
            .field("drdy", &self.drdy())
            .finish()
    }
}
impl W {
    ///Bit 5 - Clock error interrupt status
    #[inline(always)]
    pub fn ceis(&mut self) -> CEIS_W<SRrs> {
        CEIS_W::new(self, 5)
    }
    ///Bit 6 - Seed error interrupt status
    #[inline(always)]
    pub fn seis(&mut self) -> SEIS_W<SRrs> {
        SEIS_W::new(self, 6)
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F407.html#RNG:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
