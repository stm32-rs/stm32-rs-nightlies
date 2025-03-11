///Register `WCFGR` reader
pub type R = crate::R<WCFGRrs>;
///Register `WCFGR` writer
pub type W = crate::W<WCFGRrs>;
/**DSI mode This bit selects the mode for the video transmission. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSIM {
    ///0: Video mode
    B0x0 = 0,
    ///1: Adapted command mode
    B0x1 = 1,
}
impl From<DSIM> for bool {
    #[inline(always)]
    fn from(variant: DSIM) -> Self {
        variant as u8 != 0
    }
}
///Field `DSIM` reader - DSI mode This bit selects the mode for the video transmission. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
pub type DSIM_R = crate::BitReader<DSIM>;
impl DSIM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSIM {
        match self.bits {
            false => DSIM::B0x0,
            true => DSIM::B0x1,
        }
    }
    ///Video mode
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DSIM::B0x0
    }
    ///Adapted command mode
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DSIM::B0x1
    }
}
///Field `DSIM` writer - DSI mode This bit selects the mode for the video transmission. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
pub type DSIM_W<'a, REG> = crate::BitWriter<'a, REG, DSIM>;
impl<'a, REG> DSIM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Video mode
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DSIM::B0x0)
    }
    ///Adapted command mode
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DSIM::B0x1)
    }
}
/**Color multiplexing This bit selects the color multiplexing used by DSI Host. This field must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.EN=0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COLMUX {
    ///0: 16-bit configuration 1
    B0x0 = 0,
    ///1: 16-bit configuration 2
    B0x1 = 1,
    ///2: 16-bit configuration 3
    B0x2 = 2,
    ///3: 18-bit configuration 1
    B0x3 = 3,
    ///4: 18-bit configuration 2
    B0x4 = 4,
    ///5: 24-bit
    B0x5 = 5,
}
impl From<COLMUX> for u8 {
    #[inline(always)]
    fn from(variant: COLMUX) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COLMUX {
    type Ux = u8;
}
impl crate::IsEnum for COLMUX {}
///Field `COLMUX` reader - Color multiplexing This bit selects the color multiplexing used by DSI Host. This field must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.EN=0).
pub type COLMUX_R = crate::FieldReader<COLMUX>;
impl COLMUX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<COLMUX> {
        match self.bits {
            0 => Some(COLMUX::B0x0),
            1 => Some(COLMUX::B0x1),
            2 => Some(COLMUX::B0x2),
            3 => Some(COLMUX::B0x3),
            4 => Some(COLMUX::B0x4),
            5 => Some(COLMUX::B0x5),
            _ => None,
        }
    }
    ///16-bit configuration 1
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == COLMUX::B0x0
    }
    ///16-bit configuration 2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == COLMUX::B0x1
    }
    ///16-bit configuration 3
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == COLMUX::B0x2
    }
    ///18-bit configuration 1
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == COLMUX::B0x3
    }
    ///18-bit configuration 2
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == COLMUX::B0x4
    }
    ///24-bit
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == COLMUX::B0x5
    }
}
///Field `COLMUX` writer - Color multiplexing This bit selects the color multiplexing used by DSI Host. This field must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.EN=0).
pub type COLMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 3, COLMUX>;
impl<'a, REG> COLMUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///16-bit configuration 1
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COLMUX::B0x0)
    }
    ///16-bit configuration 2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COLMUX::B0x1)
    }
    ///16-bit configuration 3
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(COLMUX::B0x2)
    }
    ///18-bit configuration 1
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(COLMUX::B0x3)
    }
    ///18-bit configuration 2
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(COLMUX::B0x4)
    }
    ///24-bit
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(COLMUX::B0x5)
    }
}
/**TE source This bit selects the tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TESRC {
    ///0: DSI Link
    B0x0 = 0,
    ///1: External pin
    B0x1 = 1,
}
impl From<TESRC> for bool {
    #[inline(always)]
    fn from(variant: TESRC) -> Self {
        variant as u8 != 0
    }
}
///Field `TESRC` reader - TE source This bit selects the tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
pub type TESRC_R = crate::BitReader<TESRC>;
impl TESRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TESRC {
        match self.bits {
            false => TESRC::B0x0,
            true => TESRC::B0x1,
        }
    }
    ///DSI Link
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TESRC::B0x0
    }
    ///External pin
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TESRC::B0x1
    }
}
///Field `TESRC` writer - TE source This bit selects the tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
pub type TESRC_W<'a, REG> = crate::BitWriter<'a, REG, TESRC>;
impl<'a, REG> TESRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DSI Link
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TESRC::B0x0)
    }
    ///External pin
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TESRC::B0x1)
    }
}
/**TE polarity This bit selects the polarity of the external pin tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEPOL {
    ///0: rising edge.
    B0x0 = 0,
    ///1: falling edge.
    B0x1 = 1,
}
impl From<TEPOL> for bool {
    #[inline(always)]
    fn from(variant: TEPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `TEPOL` reader - TE polarity This bit selects the polarity of the external pin tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
pub type TEPOL_R = crate::BitReader<TEPOL>;
impl TEPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEPOL {
        match self.bits {
            false => TEPOL::B0x0,
            true => TEPOL::B0x1,
        }
    }
    ///rising edge.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEPOL::B0x0
    }
    ///falling edge.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEPOL::B0x1
    }
}
///Field `TEPOL` writer - TE polarity This bit selects the polarity of the external pin tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
pub type TEPOL_W<'a, REG> = crate::BitWriter<'a, REG, TEPOL>;
impl<'a, REG> TEPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///rising edge.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEPOL::B0x0)
    }
    ///falling edge.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEPOL::B0x1)
    }
}
/**Automatic refresh This bit selects the refresh mode in DBI mode. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AR {
    ///0: automatic refresh mode disabled
    B0x0 = 0,
    ///1: automatic refresh mode enabled
    B0x1 = 1,
}
impl From<AR> for bool {
    #[inline(always)]
    fn from(variant: AR) -> Self {
        variant as u8 != 0
    }
}
///Field `AR` reader - Automatic refresh This bit selects the refresh mode in DBI mode. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
pub type AR_R = crate::BitReader<AR>;
impl AR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AR {
        match self.bits {
            false => AR::B0x0,
            true => AR::B0x1,
        }
    }
    ///automatic refresh mode disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AR::B0x0
    }
    ///automatic refresh mode enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AR::B0x1
    }
}
///Field `AR` writer - Automatic refresh This bit selects the refresh mode in DBI mode. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
pub type AR_W<'a, REG> = crate::BitWriter<'a, REG, AR>;
impl<'a, REG> AR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///automatic refresh mode disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AR::B0x0)
    }
    ///automatic refresh mode enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AR::B0x1)
    }
}
/**VSync polarity This bit selects the VSync edge on which the LTDC is halted. This bit must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.EN=0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSPOL {
    ///0: LTDC halted on a falling edge
    B0x0 = 0,
    ///1: LTDC halted on a rising edge
    B0x1 = 1,
}
impl From<VSPOL> for bool {
    #[inline(always)]
    fn from(variant: VSPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `VSPOL` reader - VSync polarity This bit selects the VSync edge on which the LTDC is halted. This bit must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.EN=0).
pub type VSPOL_R = crate::BitReader<VSPOL>;
impl VSPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VSPOL {
        match self.bits {
            false => VSPOL::B0x0,
            true => VSPOL::B0x1,
        }
    }
    ///LTDC halted on a falling edge
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VSPOL::B0x0
    }
    ///LTDC halted on a rising edge
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VSPOL::B0x1
    }
}
///Field `VSPOL` writer - VSync polarity This bit selects the VSync edge on which the LTDC is halted. This bit must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.EN=0).
pub type VSPOL_W<'a, REG> = crate::BitWriter<'a, REG, VSPOL>;
impl<'a, REG> VSPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LTDC halted on a falling edge
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VSPOL::B0x0)
    }
    ///LTDC halted on a rising edge
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(VSPOL::B0x1)
    }
}
impl R {
    ///Bit 0 - DSI mode This bit selects the mode for the video transmission. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
    #[inline(always)]
    pub fn dsim(&self) -> DSIM_R {
        DSIM_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - Color multiplexing This bit selects the color multiplexing used by DSI Host. This field must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.EN=0).
    #[inline(always)]
    pub fn colmux(&self) -> COLMUX_R {
        COLMUX_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 4 - TE source This bit selects the tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
    #[inline(always)]
    pub fn tesrc(&self) -> TESRC_R {
        TESRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TE polarity This bit selects the polarity of the external pin tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
    #[inline(always)]
    pub fn tepol(&self) -> TEPOL_R {
        TEPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Automatic refresh This bit selects the refresh mode in DBI mode. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - VSync polarity This bit selects the VSync edge on which the LTDC is halted. This bit must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.EN=0).
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WCFGR")
            .field("dsim", &self.dsim())
            .field("colmux", &self.colmux())
            .field("tesrc", &self.tesrc())
            .field("tepol", &self.tepol())
            .field("ar", &self.ar())
            .field("vspol", &self.vspol())
            .finish()
    }
}
impl W {
    ///Bit 0 - DSI mode This bit selects the mode for the video transmission. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
    #[inline(always)]
    pub fn dsim(&mut self) -> DSIM_W<WCFGRrs> {
        DSIM_W::new(self, 0)
    }
    ///Bits 1:3 - Color multiplexing This bit selects the color multiplexing used by DSI Host. This field must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.EN=0).
    #[inline(always)]
    pub fn colmux(&mut self) -> COLMUX_W<WCFGRrs> {
        COLMUX_W::new(self, 1)
    }
    ///Bit 4 - TE source This bit selects the tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
    #[inline(always)]
    pub fn tesrc(&mut self) -> TESRC_W<WCFGRrs> {
        TESRC_W::new(self, 4)
    }
    ///Bit 5 - TE polarity This bit selects the polarity of the external pin tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
    #[inline(always)]
    pub fn tepol(&mut self) -> TEPOL_W<WCFGRrs> {
        TEPOL_W::new(self, 5)
    }
    ///Bit 6 - Automatic refresh This bit selects the refresh mode in DBI mode. This bit must only be changed when DSI Host is stopped (DSI_CR.EN = 0).
    #[inline(always)]
    pub fn ar(&mut self) -> AR_W<WCFGRrs> {
        AR_W::new(self, 6)
    }
    ///Bit 7 - VSync polarity This bit selects the VSync edge on which the LTDC is halted. This bit must only be changed when DSI is stopped (DSI_WCR.DSIEN = 0 and DSI_CR.EN=0).
    #[inline(always)]
    pub fn vspol(&mut self) -> VSPOL_W<WCFGRrs> {
        VSPOL_W::new(self, 7)
    }
}
/**DSI Wrapper configuration register

You can [`read`](crate::Reg::read) this register and get [`wcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:WCFGR)*/
pub struct WCFGRrs;
impl crate::RegisterSpec for WCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`wcfgr::R`](R) reader structure
impl crate::Readable for WCFGRrs {}
///`write(|w| ..)` method takes [`wcfgr::W`](W) writer structure
impl crate::Writable for WCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WCFGR to value 0
impl crate::Resettable for WCFGRrs {}
