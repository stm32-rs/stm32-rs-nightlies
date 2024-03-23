#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Data Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRDY {
    #[doc = "0: The RNG_DR register is not yet valid, no random data is available"]
    Invalid = 0,
    #[doc = "1: The RNG_DR register contains valid random data"]
    Valid = 1,
}
impl From<DRDY> for bool {
    #[inline(always)]
    fn from(variant: DRDY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRDY` reader - Data Ready"]
pub type DRDY_R = crate::BitReader<DRDY>;
impl DRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRDY {
        match self.bits {
            false => DRDY::Invalid,
            true => DRDY::Valid,
        }
    }
    #[doc = "The RNG_DR register is not yet valid, no random data is available"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == DRDY::Invalid
    }
    #[doc = "The RNG_DR register contains valid random data"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == DRDY::Valid
    }
}
#[doc = "Clock error current status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CECSR {
    #[doc = "0: The RNG clock is correct (fRNGCLK> fHCLK/32)"]
    Correct = 0,
    #[doc = "1: The RNG clock before internal divider has been detected too slow (fRNGCLK&lt; fHCLK/32)"]
    Slow = 1,
}
impl From<CECSR> for bool {
    #[inline(always)]
    fn from(variant: CECSR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CECS` reader - Clock error current status"]
pub type CECS_R = crate::BitReader<CECSR>;
impl CECS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CECSR {
        match self.bits {
            false => CECSR::Correct,
            true => CECSR::Slow,
        }
    }
    #[doc = "The RNG clock is correct (fRNGCLK> fHCLK/32)"]
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        *self == CECSR::Correct
    }
    #[doc = "The RNG clock before internal divider has been detected too slow (fRNGCLK&lt; fHCLK/32)"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == CECSR::Slow
    }
}
#[doc = "Seed error current status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECSR {
    #[doc = "0: No faulty sequence has currently been detected. If the SEIS bit is set, this means that a faulty sequence was detected and the situation has been recovered"]
    NoFault = 0,
    #[doc = "1: At least one faulty sequence has been detected - see ref manual for details"]
    Fault = 1,
}
impl From<SECSR> for bool {
    #[inline(always)]
    fn from(variant: SECSR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECS` reader - Seed error current status"]
pub type SECS_R = crate::BitReader<SECSR>;
impl SECS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SECSR {
        match self.bits {
            false => SECSR::NoFault,
            true => SECSR::Fault,
        }
    }
    #[doc = "No faulty sequence has currently been detected. If the SEIS bit is set, this means that a faulty sequence was detected and the situation has been recovered"]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == SECSR::NoFault
    }
    #[doc = "At least one faulty sequence has been detected - see ref manual for details"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == SECSR::Fault
    }
}
#[doc = "Clock error interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEIS {
    #[doc = "0: The RNG clock is correct (fRNGCLK> fHCLK/32)"]
    Correct = 0,
    #[doc = "1: The RNG clock before internal divider has been detected too slow (fRNGCLK&lt; fHCLK/32)"]
    Slow = 1,
}
impl From<CEIS> for bool {
    #[inline(always)]
    fn from(variant: CEIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIS` reader - Clock error interrupt status"]
pub type CEIS_R = crate::BitReader<CEIS>;
impl CEIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEIS {
        match self.bits {
            false => CEIS::Correct,
            true => CEIS::Slow,
        }
    }
    #[doc = "The RNG clock is correct (fRNGCLK> fHCLK/32)"]
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        *self == CEIS::Correct
    }
    #[doc = "The RNG clock before internal divider has been detected too slow (fRNGCLK&lt; fHCLK/32)"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == CEIS::Slow
    }
}
#[doc = "Field `CEIS` writer - Clock error interrupt status"]
pub type CEIS_W<'a, REG> = crate::BitWriter<'a, REG, CEIS>;
impl<'a, REG> CEIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RNG clock is correct (fRNGCLK> fHCLK/32)"]
    #[inline(always)]
    pub fn correct(self) -> &'a mut crate::W<REG> {
        self.variant(CEIS::Correct)
    }
    #[doc = "The RNG clock before internal divider has been detected too slow (fRNGCLK&lt; fHCLK/32)"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut crate::W<REG> {
        self.variant(CEIS::Slow)
    }
}
#[doc = "Seed error interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEIS {
    #[doc = "0: No faulty sequence detected"]
    NoFault = 0,
    #[doc = "1: At least one faulty sequence has been detected"]
    Fault = 1,
}
impl From<SEIS> for bool {
    #[inline(always)]
    fn from(variant: SEIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEIS` reader - Seed error interrupt status"]
pub type SEIS_R = crate::BitReader<SEIS>;
impl SEIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SEIS {
        match self.bits {
            false => SEIS::NoFault,
            true => SEIS::Fault,
        }
    }
    #[doc = "No faulty sequence detected"]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == SEIS::NoFault
    }
    #[doc = "At least one faulty sequence has been detected"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == SEIS::Fault
    }
}
#[doc = "Field `SEIS` writer - Seed error interrupt status"]
pub type SEIS_W<'a, REG> = crate::BitWriter<'a, REG, SEIS>;
impl<'a, REG> SEIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No faulty sequence detected"]
    #[inline(always)]
    pub fn no_fault(self) -> &'a mut crate::W<REG> {
        self.variant(SEIS::NoFault)
    }
    #[doc = "At least one faulty sequence has been detected"]
    #[inline(always)]
    pub fn fault(self) -> &'a mut crate::W<REG> {
        self.variant(SEIS::Fault)
    }
}
impl R {
    #[doc = "Bit 0 - Data Ready"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock error current status"]
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Seed error current status"]
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock error interrupt status"]
    #[inline(always)]
    pub fn ceis(&self) -> CEIS_R {
        CEIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Seed error interrupt status"]
    #[inline(always)]
    pub fn seis(&self) -> SEIS_R {
        SEIS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Clock error interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn ceis(&mut self) -> CEIS_W<SRrs> {
        CEIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Seed error interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn seis(&mut self) -> SEIS_W<SRrs> {
        SEIS_W::new(self, 6)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
