///Register `WCR` reader
pub type R = crate::R<WCRrs>;
///Register `WCR` writer
pub type W = crate::W<WCRrs>;
/**Color mode This bit controls the display color mode in video mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COLM {
    ///0: Full color mode
    B0x0 = 0,
    ///1: Eight color mode
    B0x1 = 1,
}
impl From<COLM> for bool {
    #[inline(always)]
    fn from(variant: COLM) -> Self {
        variant as u8 != 0
    }
}
///Field `COLM` reader - Color mode This bit controls the display color mode in video mode.
pub type COLM_R = crate::BitReader<COLM>;
impl COLM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COLM {
        match self.bits {
            false => COLM::B0x0,
            true => COLM::B0x1,
        }
    }
    ///Full color mode
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == COLM::B0x0
    }
    ///Eight color mode
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == COLM::B0x1
    }
}
///Field `COLM` writer - Color mode This bit controls the display color mode in video mode.
pub type COLM_W<'a, REG> = crate::BitWriter<'a, REG, COLM>;
impl<'a, REG> COLM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Full color mode
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COLM::B0x0)
    }
    ///Eight color mode
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COLM::B0x1)
    }
}
/**Shutdown This bit controls the display shutdown in video mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHTDN {
    ///0: display ON
    B0x0 = 0,
    ///1: display OFF
    B0x1 = 1,
}
impl From<SHTDN> for bool {
    #[inline(always)]
    fn from(variant: SHTDN) -> Self {
        variant as u8 != 0
    }
}
///Field `SHTDN` reader - Shutdown This bit controls the display shutdown in video mode.
pub type SHTDN_R = crate::BitReader<SHTDN>;
impl SHTDN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SHTDN {
        match self.bits {
            false => SHTDN::B0x0,
            true => SHTDN::B0x1,
        }
    }
    ///display ON
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SHTDN::B0x0
    }
    ///display OFF
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SHTDN::B0x1
    }
}
///Field `SHTDN` writer - Shutdown This bit controls the display shutdown in video mode.
pub type SHTDN_W<'a, REG> = crate::BitWriter<'a, REG, SHTDN>;
impl<'a, REG> SHTDN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///display ON
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SHTDN::B0x0)
    }
    ///display OFF
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SHTDN::B0x1)
    }
}
/**LTDC enable This bit enables the LTDC for a frame transfer in adapted command mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCEN {
    ///0: LTDC disabled
    B0x0 = 0,
    ///1: LTDC enabled
    B0x1 = 1,
}
impl From<LTDCEN> for bool {
    #[inline(always)]
    fn from(variant: LTDCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LTDCEN` reader - LTDC enable This bit enables the LTDC for a frame transfer in adapted command mode.
pub type LTDCEN_R = crate::BitReader<LTDCEN>;
impl LTDCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LTDCEN {
        match self.bits {
            false => LTDCEN::B0x0,
            true => LTDCEN::B0x1,
        }
    }
    ///LTDC disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LTDCEN::B0x0
    }
    ///LTDC enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LTDCEN::B0x1
    }
}
///Field `LTDCEN` writer - LTDC enable This bit enables the LTDC for a frame transfer in adapted command mode.
pub type LTDCEN_W<'a, REG> = crate::BitWriter<'a, REG, LTDCEN>;
impl<'a, REG> LTDCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LTDC disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCEN::B0x0)
    }
    ///LTDC enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCEN::B0x1)
    }
}
/**DSI enable This bit enables the DSI Wrapper.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSIEN {
    ///0: DSI disabled
    B0x0 = 0,
    ///1: DSI enabled
    B0x1 = 1,
}
impl From<DSIEN> for bool {
    #[inline(always)]
    fn from(variant: DSIEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DSIEN` reader - DSI enable This bit enables the DSI Wrapper.
pub type DSIEN_R = crate::BitReader<DSIEN>;
impl DSIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSIEN {
        match self.bits {
            false => DSIEN::B0x0,
            true => DSIEN::B0x1,
        }
    }
    ///DSI disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DSIEN::B0x0
    }
    ///DSI enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DSIEN::B0x1
    }
}
///Field `DSIEN` writer - DSI enable This bit enables the DSI Wrapper.
pub type DSIEN_W<'a, REG> = crate::BitWriter<'a, REG, DSIEN>;
impl<'a, REG> DSIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DSI disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DSIEN::B0x0)
    }
    ///DSI enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DSIEN::B0x1)
    }
}
impl R {
    ///Bit 0 - Color mode This bit controls the display color mode in video mode.
    #[inline(always)]
    pub fn colm(&self) -> COLM_R {
        COLM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Shutdown This bit controls the display shutdown in video mode.
    #[inline(always)]
    pub fn shtdn(&self) -> SHTDN_R {
        SHTDN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LTDC enable This bit enables the LTDC for a frame transfer in adapted command mode.
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DSI enable This bit enables the DSI Wrapper.
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WCR")
            .field("colm", &self.colm())
            .field("shtdn", &self.shtdn())
            .field("ltdcen", &self.ltdcen())
            .field("dsien", &self.dsien())
            .finish()
    }
}
impl W {
    ///Bit 0 - Color mode This bit controls the display color mode in video mode.
    #[inline(always)]
    pub fn colm(&mut self) -> COLM_W<WCRrs> {
        COLM_W::new(self, 0)
    }
    ///Bit 1 - Shutdown This bit controls the display shutdown in video mode.
    #[inline(always)]
    pub fn shtdn(&mut self) -> SHTDN_W<WCRrs> {
        SHTDN_W::new(self, 1)
    }
    ///Bit 2 - LTDC enable This bit enables the LTDC for a frame transfer in adapted command mode.
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W<WCRrs> {
        LTDCEN_W::new(self, 2)
    }
    ///Bit 3 - DSI enable This bit enables the DSI Wrapper.
    #[inline(always)]
    pub fn dsien(&mut self) -> DSIEN_W<WCRrs> {
        DSIEN_W::new(self, 3)
    }
}
/**DSI Wrapper control register

You can [`read`](crate::Reg::read) this register and get [`wcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:WCR)*/
pub struct WCRrs;
impl crate::RegisterSpec for WCRrs {
    type Ux = u32;
}
///`read()` method returns [`wcr::R`](R) reader structure
impl crate::Readable for WCRrs {}
///`write(|w| ..)` method takes [`wcr::W`](W) writer structure
impl crate::Writable for WCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WCR to value 0
impl crate::Resettable for WCRrs {
    const RESET_VALUE: u32 = 0;
}
