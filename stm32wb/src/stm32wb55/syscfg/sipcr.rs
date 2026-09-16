///Register `SIPCR` reader
pub type R = crate::R<SIPCRrs>;
///Register `SIPCR` writer
pub type W = crate::W<SIPCRrs>;
/**Enable AES1 KEY\[7:0\] security.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAES1 {
    ///0: AES1 KEY\[7:0\] security disabled
    Disabled = 0,
    ///1: AES1 KEY\[7:0\] security enabled
    Enabled = 1,
}
impl From<SAES1> for bool {
    #[inline(always)]
    fn from(variant: SAES1) -> Self {
        variant as u8 != 0
    }
}
///Field `SAES1` reader - Enable AES1 KEY\[7:0\] security.
pub type SAES1_R = crate::BitReader<SAES1>;
impl SAES1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SAES1 {
        match self.bits {
            false => SAES1::Disabled,
            true => SAES1::Enabled,
        }
    }
    ///AES1 KEY\[7:0\] security disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAES1::Disabled
    }
    ///AES1 KEY\[7:0\] security enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAES1::Enabled
    }
}
///Field `SAES1` writer - Enable AES1 KEY\[7:0\] security.
pub type SAES1_W<'a, REG> = crate::BitWriter<'a, REG, SAES1>;
impl<'a, REG> SAES1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AES1 KEY\[7:0\] security disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SAES1::Disabled)
    }
    ///AES1 KEY\[7:0\] security enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SAES1::Enabled)
    }
}
/**Enable AES2 security.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAES2 {
    ///0: AES2 security disabled
    Disabled = 0,
    ///1: AES2 security enabled
    Enabled = 1,
}
impl From<SAES2> for bool {
    #[inline(always)]
    fn from(variant: SAES2) -> Self {
        variant as u8 != 0
    }
}
///Field `SAES2` reader - Enable AES2 security.
pub type SAES2_R = crate::BitReader<SAES2>;
impl SAES2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SAES2 {
        match self.bits {
            false => SAES2::Disabled,
            true => SAES2::Enabled,
        }
    }
    ///AES2 security disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAES2::Disabled
    }
    ///AES2 security enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAES2::Enabled
    }
}
///Field `SAES2` writer - Enable AES2 security.
pub type SAES2_W<'a, REG> = crate::BitWriter<'a, REG, SAES2>;
impl<'a, REG> SAES2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AES2 security disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SAES2::Disabled)
    }
    ///AES2 security enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SAES2::Enabled)
    }
}
/**Enable PKA security

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPKA {
    ///0: PKA security disabled
    Disabled = 0,
    ///1: PKA security enabled
    Enabled = 1,
}
impl From<SPKA> for bool {
    #[inline(always)]
    fn from(variant: SPKA) -> Self {
        variant as u8 != 0
    }
}
///Field `SPKA` reader - Enable PKA security
pub type SPKA_R = crate::BitReader<SPKA>;
impl SPKA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPKA {
        match self.bits {
            false => SPKA::Disabled,
            true => SPKA::Enabled,
        }
    }
    ///PKA security disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPKA::Disabled
    }
    ///PKA security enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPKA::Enabled
    }
}
///Field `SPKA` writer - Enable PKA security
pub type SPKA_W<'a, REG> = crate::BitWriter<'a, REG, SPKA>;
impl<'a, REG> SPKA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PKA security disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPKA::Disabled)
    }
    ///PKA security enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPKA::Enabled)
    }
}
/**Enable True RNG security

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRNG {
    ///0: True RNG security disabled
    Disabled = 0,
    ///1: True RNG security enabled
    Enabled = 1,
}
impl From<SRNG> for bool {
    #[inline(always)]
    fn from(variant: SRNG) -> Self {
        variant as u8 != 0
    }
}
///Field `SRNG` reader - Enable True RNG security
pub type SRNG_R = crate::BitReader<SRNG>;
impl SRNG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRNG {
        match self.bits {
            false => SRNG::Disabled,
            true => SRNG::Enabled,
        }
    }
    ///True RNG security disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRNG::Disabled
    }
    ///True RNG security enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRNG::Enabled
    }
}
///Field `SRNG` writer - Enable True RNG security
pub type SRNG_W<'a, REG> = crate::BitWriter<'a, REG, SRNG>;
impl<'a, REG> SRNG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///True RNG security disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRNG::Disabled)
    }
    ///True RNG security enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRNG::Enabled)
    }
}
impl R {
    ///Bit 0 - Enable AES1 KEY\[7:0\] security.
    #[inline(always)]
    pub fn saes1(&self) -> SAES1_R {
        SAES1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable AES2 security.
    #[inline(always)]
    pub fn saes2(&self) -> SAES2_R {
        SAES2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable PKA security
    #[inline(always)]
    pub fn spka(&self) -> SPKA_R {
        SPKA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Enable True RNG security
    #[inline(always)]
    pub fn srng(&self) -> SRNG_R {
        SRNG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIPCR")
            .field("saes1", &self.saes1())
            .field("saes2", &self.saes2())
            .field("spka", &self.spka())
            .field("srng", &self.srng())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable AES1 KEY\[7:0\] security.
    #[inline(always)]
    pub fn saes1(&mut self) -> SAES1_W<'_, SIPCRrs> {
        SAES1_W::new(self, 0)
    }
    ///Bit 1 - Enable AES2 security.
    #[inline(always)]
    pub fn saes2(&mut self) -> SAES2_W<'_, SIPCRrs> {
        SAES2_W::new(self, 1)
    }
    ///Bit 2 - Enable PKA security
    #[inline(always)]
    pub fn spka(&mut self) -> SPKA_W<'_, SIPCRrs> {
        SPKA_W::new(self, 2)
    }
    ///Bit 3 - Enable True RNG security
    #[inline(always)]
    pub fn srng(&mut self) -> SRNG_W<'_, SIPCRrs> {
        SRNG_W::new(self, 3)
    }
}
/**secure IP control register

You can [`read`](crate::Reg::read) this register and get [`sipcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sipcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:SIPCR)*/
pub struct SIPCRrs;
impl crate::RegisterSpec for SIPCRrs {
    type Ux = u32;
}
///`read()` method returns [`sipcr::R`](R) reader structure
impl crate::Readable for SIPCRrs {}
///`write(|w| ..)` method takes [`sipcr::W`](W) writer structure
impl crate::Writable for SIPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SIPCR to value 0
impl crate::Resettable for SIPCRrs {}
