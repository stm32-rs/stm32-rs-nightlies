///Register `RCC_IOPRSTR` reader
pub type R = crate::R<RCC_IOPRSTRrs>;
///Register `RCC_IOPRSTR` writer
pub type W = crate::W<RCC_IOPRSTRrs>;
/**I/O port A reset This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOARST {
    ///0: no effect
    B0x0 = 0,
    ///1: Reset I/O port A
    B0x1 = 1,
}
impl From<GPIOARST> for bool {
    #[inline(always)]
    fn from(variant: GPIOARST) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOARST` reader - I/O port A reset This bit is set and cleared by software.
pub type GPIOARST_R = crate::BitReader<GPIOARST>;
impl GPIOARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOARST {
        match self.bits {
            false => GPIOARST::B0x0,
            true => GPIOARST::B0x1,
        }
    }
    ///no effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOARST::B0x0
    }
    ///Reset I/O port A
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOARST::B0x1
    }
}
///Field `GPIOARST` writer - I/O port A reset This bit is set and cleared by software.
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG, GPIOARST>;
impl<'a, REG> GPIOARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOARST::B0x0)
    }
    ///Reset I/O port A
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOARST::B0x1)
    }
}
/**I/O port B reset This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOBRST {
    ///0: no effect
    B0x0 = 0,
    ///1: Reset I/O port B
    B0x1 = 1,
}
impl From<GPIOBRST> for bool {
    #[inline(always)]
    fn from(variant: GPIOBRST) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOBRST` reader - I/O port B reset This bit is set and cleared by software.
pub type GPIOBRST_R = crate::BitReader<GPIOBRST>;
impl GPIOBRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOBRST {
        match self.bits {
            false => GPIOBRST::B0x0,
            true => GPIOBRST::B0x1,
        }
    }
    ///no effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOBRST::B0x0
    }
    ///Reset I/O port B
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOBRST::B0x1
    }
}
///Field `GPIOBRST` writer - I/O port B reset This bit is set and cleared by software.
pub type GPIOBRST_W<'a, REG> = crate::BitWriter<'a, REG, GPIOBRST>;
impl<'a, REG> GPIOBRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOBRST::B0x0)
    }
    ///Reset I/O port B
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOBRST::B0x1)
    }
}
/**I/O port C reset This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOCRST {
    ///0: no effect
    B0x0 = 0,
    ///1: Reset I/O port C
    B0x1 = 1,
}
impl From<GPIOCRST> for bool {
    #[inline(always)]
    fn from(variant: GPIOCRST) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOCRST` reader - I/O port C reset This bit is set and cleared by software.
pub type GPIOCRST_R = crate::BitReader<GPIOCRST>;
impl GPIOCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOCRST {
        match self.bits {
            false => GPIOCRST::B0x0,
            true => GPIOCRST::B0x1,
        }
    }
    ///no effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOCRST::B0x0
    }
    ///Reset I/O port C
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOCRST::B0x1
    }
}
///Field `GPIOCRST` writer - I/O port C reset This bit is set and cleared by software.
pub type GPIOCRST_W<'a, REG> = crate::BitWriter<'a, REG, GPIOCRST>;
impl<'a, REG> GPIOCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOCRST::B0x0)
    }
    ///Reset I/O port C
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOCRST::B0x1)
    }
}
/**I/O port D reset This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIODRST {
    ///0: no effect
    B0x0 = 0,
    ///1: Reset I/O port D
    B0x1 = 1,
}
impl From<GPIODRST> for bool {
    #[inline(always)]
    fn from(variant: GPIODRST) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIODRST` reader - I/O port D reset This bit is set and cleared by software.
pub type GPIODRST_R = crate::BitReader<GPIODRST>;
impl GPIODRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIODRST {
        match self.bits {
            false => GPIODRST::B0x0,
            true => GPIODRST::B0x1,
        }
    }
    ///no effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIODRST::B0x0
    }
    ///Reset I/O port D
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIODRST::B0x1
    }
}
///Field `GPIODRST` writer - I/O port D reset This bit is set and cleared by software.
pub type GPIODRST_W<'a, REG> = crate::BitWriter<'a, REG, GPIODRST>;
impl<'a, REG> GPIODRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIODRST::B0x0)
    }
    ///Reset I/O port D
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIODRST::B0x1)
    }
}
/**I/O port F reset This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOFRST {
    ///0: no effect
    B0x0 = 0,
    ///1: Reset I/O port F
    B0x1 = 1,
}
impl From<GPIOFRST> for bool {
    #[inline(always)]
    fn from(variant: GPIOFRST) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOFRST` reader - I/O port F reset This bit is set and cleared by software.
pub type GPIOFRST_R = crate::BitReader<GPIOFRST>;
impl GPIOFRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOFRST {
        match self.bits {
            false => GPIOFRST::B0x0,
            true => GPIOFRST::B0x1,
        }
    }
    ///no effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOFRST::B0x0
    }
    ///Reset I/O port F
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOFRST::B0x1
    }
}
///Field `GPIOFRST` writer - I/O port F reset This bit is set and cleared by software.
pub type GPIOFRST_W<'a, REG> = crate::BitWriter<'a, REG, GPIOFRST>;
impl<'a, REG> GPIOFRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOFRST::B0x0)
    }
    ///Reset I/O port F
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOFRST::B0x1)
    }
}
impl R {
    ///Bit 0 - I/O port A reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O port B reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O port C reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - I/O port F reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IOPRSTR")
            .field("gpioarst", &self.gpioarst())
            .field("gpiobrst", &self.gpiobrst())
            .field("gpiocrst", &self.gpiocrst())
            .field("gpiodrst", &self.gpiodrst())
            .field("gpiofrst", &self.gpiofrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - I/O port A reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W<'_, RCC_IOPRSTRrs> {
        GPIOARST_W::new(self, 0)
    }
    ///Bit 1 - I/O port B reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<'_, RCC_IOPRSTRrs> {
        GPIOBRST_W::new(self, 1)
    }
    ///Bit 2 - I/O port C reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<'_, RCC_IOPRSTRrs> {
        GPIOCRST_W::new(self, 2)
    }
    ///Bit 3 - I/O port D reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<'_, RCC_IOPRSTRrs> {
        GPIODRST_W::new(self, 3)
    }
    ///Bit 5 - I/O port F reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<'_, RCC_IOPRSTRrs> {
        GPIOFRST_W::new(self, 5)
    }
}
/**RCC I/O port reset register

You can [`read`](crate::Reg::read) this register and get [`rcc_ioprstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ioprstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#RCC:RCC_IOPRSTR)*/
pub struct RCC_IOPRSTRrs;
impl crate::RegisterSpec for RCC_IOPRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_ioprstr::R`](R) reader structure
impl crate::Readable for RCC_IOPRSTRrs {}
///`write(|w| ..)` method takes [`rcc_ioprstr::W`](W) writer structure
impl crate::Writable for RCC_IOPRSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_IOPRSTR to value 0
impl crate::Resettable for RCC_IOPRSTRrs {}
