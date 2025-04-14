///Register `PUCR` reader
pub type R = crate::R<PUCRrs>;
///Register `PUCR` writer
pub type W = crate::W<PUCRrs>;
/**ULPS request on clock lane ULPS mode request on clock lane.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum URCL {
    ///0: No ULPS request
    B0x0 = 0,
    ///1: Request ULPS mode on clock lane
    B0x1 = 1,
}
impl From<URCL> for bool {
    #[inline(always)]
    fn from(variant: URCL) -> Self {
        variant as u8 != 0
    }
}
///Field `URCL` reader - ULPS request on clock lane ULPS mode request on clock lane.
pub type URCL_R = crate::BitReader<URCL>;
impl URCL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> URCL {
        match self.bits {
            false => URCL::B0x0,
            true => URCL::B0x1,
        }
    }
    ///No ULPS request
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == URCL::B0x0
    }
    ///Request ULPS mode on clock lane
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == URCL::B0x1
    }
}
///Field `URCL` writer - ULPS request on clock lane ULPS mode request on clock lane.
pub type URCL_W<'a, REG> = crate::BitWriter<'a, REG, URCL>;
impl<'a, REG> URCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No ULPS request
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(URCL::B0x0)
    }
    ///Request ULPS mode on clock lane
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(URCL::B0x1)
    }
}
/**ULPS exit on clock lane ULPS mode exit on clock lane.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UECL {
    ///0: No exit request
    B0x0 = 0,
    ///1: Exit ULPS mode on clock lane
    B0x1 = 1,
}
impl From<UECL> for bool {
    #[inline(always)]
    fn from(variant: UECL) -> Self {
        variant as u8 != 0
    }
}
///Field `UECL` reader - ULPS exit on clock lane ULPS mode exit on clock lane.
pub type UECL_R = crate::BitReader<UECL>;
impl UECL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UECL {
        match self.bits {
            false => UECL::B0x0,
            true => UECL::B0x1,
        }
    }
    ///No exit request
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UECL::B0x0
    }
    ///Exit ULPS mode on clock lane
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UECL::B0x1
    }
}
///Field `UECL` writer - ULPS exit on clock lane ULPS mode exit on clock lane.
pub type UECL_W<'a, REG> = crate::BitWriter<'a, REG, UECL>;
impl<'a, REG> UECL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No exit request
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UECL::B0x0)
    }
    ///Exit ULPS mode on clock lane
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UECL::B0x1)
    }
}
/**ULPS request on data lane ULPS mode request on all active data lanes.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum URDL {
    ///0: No ULPS request
    B0x0 = 0,
    ///1: Request ULPS mode on all active data lane UECL
    B0x1 = 1,
}
impl From<URDL> for bool {
    #[inline(always)]
    fn from(variant: URDL) -> Self {
        variant as u8 != 0
    }
}
///Field `URDL` reader - ULPS request on data lane ULPS mode request on all active data lanes.
pub type URDL_R = crate::BitReader<URDL>;
impl URDL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> URDL {
        match self.bits {
            false => URDL::B0x0,
            true => URDL::B0x1,
        }
    }
    ///No ULPS request
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == URDL::B0x0
    }
    ///Request ULPS mode on all active data lane UECL
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == URDL::B0x1
    }
}
///Field `URDL` writer - ULPS request on data lane ULPS mode request on all active data lanes.
pub type URDL_W<'a, REG> = crate::BitWriter<'a, REG, URDL>;
impl<'a, REG> URDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No ULPS request
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(URDL::B0x0)
    }
    ///Request ULPS mode on all active data lane UECL
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(URDL::B0x1)
    }
}
/**ULPS exit on data lane ULPS mode exit on all active data lanes.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UEDL {
    ///0: No exit request
    B0x0 = 0,
    ///1: Exit ULPS mode on all active data lane URDL
    B0x1 = 1,
}
impl From<UEDL> for bool {
    #[inline(always)]
    fn from(variant: UEDL) -> Self {
        variant as u8 != 0
    }
}
///Field `UEDL` reader - ULPS exit on data lane ULPS mode exit on all active data lanes.
pub type UEDL_R = crate::BitReader<UEDL>;
impl UEDL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UEDL {
        match self.bits {
            false => UEDL::B0x0,
            true => UEDL::B0x1,
        }
    }
    ///No exit request
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UEDL::B0x0
    }
    ///Exit ULPS mode on all active data lane URDL
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UEDL::B0x1
    }
}
///Field `UEDL` writer - ULPS exit on data lane ULPS mode exit on all active data lanes.
pub type UEDL_W<'a, REG> = crate::BitWriter<'a, REG, UEDL>;
impl<'a, REG> UEDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No exit request
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UEDL::B0x0)
    }
    ///Exit ULPS mode on all active data lane URDL
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UEDL::B0x1)
    }
}
impl R {
    ///Bit 0 - ULPS request on clock lane ULPS mode request on clock lane.
    #[inline(always)]
    pub fn urcl(&self) -> URCL_R {
        URCL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ULPS exit on clock lane ULPS mode exit on clock lane.
    #[inline(always)]
    pub fn uecl(&self) -> UECL_R {
        UECL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ULPS request on data lane ULPS mode request on all active data lanes.
    #[inline(always)]
    pub fn urdl(&self) -> URDL_R {
        URDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ULPS exit on data lane ULPS mode exit on all active data lanes.
    #[inline(always)]
    pub fn uedl(&self) -> UEDL_R {
        UEDL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCR")
            .field("urcl", &self.urcl())
            .field("uecl", &self.uecl())
            .field("urdl", &self.urdl())
            .field("uedl", &self.uedl())
            .finish()
    }
}
impl W {
    ///Bit 0 - ULPS request on clock lane ULPS mode request on clock lane.
    #[inline(always)]
    pub fn urcl(&mut self) -> URCL_W<PUCRrs> {
        URCL_W::new(self, 0)
    }
    ///Bit 1 - ULPS exit on clock lane ULPS mode exit on clock lane.
    #[inline(always)]
    pub fn uecl(&mut self) -> UECL_W<PUCRrs> {
        UECL_W::new(self, 1)
    }
    ///Bit 2 - ULPS request on data lane ULPS mode request on all active data lanes.
    #[inline(always)]
    pub fn urdl(&mut self) -> URDL_W<PUCRrs> {
        URDL_W::new(self, 2)
    }
    ///Bit 3 - ULPS exit on data lane ULPS mode exit on all active data lanes.
    #[inline(always)]
    pub fn uedl(&mut self) -> UEDL_W<PUCRrs> {
        UEDL_W::new(self, 3)
    }
}
/**DSI Host PHY ULPS control register

You can [`read`](crate::Reg::read) this register and get [`pucr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:PUCR)*/
pub struct PUCRrs;
impl crate::RegisterSpec for PUCRrs {
    type Ux = u32;
}
///`read()` method returns [`pucr::R`](R) reader structure
impl crate::Readable for PUCRrs {}
///`write(|w| ..)` method takes [`pucr::W`](W) writer structure
impl crate::Writable for PUCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUCR to value 0
impl crate::Resettable for PUCRrs {}
