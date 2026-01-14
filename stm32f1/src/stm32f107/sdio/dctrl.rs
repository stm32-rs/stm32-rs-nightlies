///Register `DCTRL` reader
pub type R = crate::R<DCTRLrs>;
///Register `DCTRL` writer
pub type W = crate::W<DCTRLrs>;
/**DTEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEN {
    ///0: Disabled
    Disabled = 0,
    ///1: Start transfer
    Enabled = 1,
}
impl From<DTEN> for bool {
    #[inline(always)]
    fn from(variant: DTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DTEN` reader - DTEN
pub type DTEN_R = crate::BitReader<DTEN>;
impl DTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTEN {
        match self.bits {
            false => DTEN::Disabled,
            true => DTEN::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTEN::Disabled
    }
    ///Start transfer
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTEN::Enabled
    }
}
///Field `DTEN` writer - DTEN
pub type DTEN_W<'a, REG> = crate::BitWriter<'a, REG, DTEN>;
impl<'a, REG> DTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTEN::Disabled)
    }
    ///Start transfer
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTEN::Enabled)
    }
}
/**Data transfer direction selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTDIR {
    ///0: From controller to card
    ControllerToCard = 0,
    ///1: From card to controller
    CardToController = 1,
}
impl From<DTDIR> for bool {
    #[inline(always)]
    fn from(variant: DTDIR) -> Self {
        variant as u8 != 0
    }
}
///Field `DTDIR` reader - Data transfer direction selection
pub type DTDIR_R = crate::BitReader<DTDIR>;
impl DTDIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTDIR {
        match self.bits {
            false => DTDIR::ControllerToCard,
            true => DTDIR::CardToController,
        }
    }
    ///From controller to card
    #[inline(always)]
    pub fn is_controller_to_card(&self) -> bool {
        *self == DTDIR::ControllerToCard
    }
    ///From card to controller
    #[inline(always)]
    pub fn is_card_to_controller(&self) -> bool {
        *self == DTDIR::CardToController
    }
}
///Field `DTDIR` writer - Data transfer direction selection
pub type DTDIR_W<'a, REG> = crate::BitWriter<'a, REG, DTDIR>;
impl<'a, REG> DTDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///From controller to card
    #[inline(always)]
    pub fn controller_to_card(self) -> &'a mut crate::W<REG> {
        self.variant(DTDIR::ControllerToCard)
    }
    ///From card to controller
    #[inline(always)]
    pub fn card_to_controller(self) -> &'a mut crate::W<REG> {
        self.variant(DTDIR::CardToController)
    }
}
/**Data transfer mode selection 1: Stream or SDIO multibyte data transfer.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTMODE {
    ///0: Bloack data transfer
    BlockMode = 0,
    ///1: Stream or SDIO multibyte data transfer
    StreamMode = 1,
}
impl From<DTMODE> for bool {
    #[inline(always)]
    fn from(variant: DTMODE) -> Self {
        variant as u8 != 0
    }
}
///Field `DTMODE` reader - Data transfer mode selection 1: Stream or SDIO multibyte data transfer.
pub type DTMODE_R = crate::BitReader<DTMODE>;
impl DTMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTMODE {
        match self.bits {
            false => DTMODE::BlockMode,
            true => DTMODE::StreamMode,
        }
    }
    ///Bloack data transfer
    #[inline(always)]
    pub fn is_block_mode(&self) -> bool {
        *self == DTMODE::BlockMode
    }
    ///Stream or SDIO multibyte data transfer
    #[inline(always)]
    pub fn is_stream_mode(&self) -> bool {
        *self == DTMODE::StreamMode
    }
}
///Field `DTMODE` writer - Data transfer mode selection 1: Stream or SDIO multibyte data transfer.
pub type DTMODE_W<'a, REG> = crate::BitWriter<'a, REG, DTMODE>;
impl<'a, REG> DTMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bloack data transfer
    #[inline(always)]
    pub fn block_mode(self) -> &'a mut crate::W<REG> {
        self.variant(DTMODE::BlockMode)
    }
    ///Stream or SDIO multibyte data transfer
    #[inline(always)]
    pub fn stream_mode(self) -> &'a mut crate::W<REG> {
        self.variant(DTMODE::StreamMode)
    }
}
/**DMA enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN {
    ///0: Dma disabled
    Disabled = 0,
    ///1: Dma enabled
    Enabled = 1,
}
impl From<DMAEN> for bool {
    #[inline(always)]
    fn from(variant: DMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAEN` reader - DMA enable bit
pub type DMAEN_R = crate::BitReader<DMAEN>;
impl DMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN {
        match self.bits {
            false => DMAEN::Disabled,
            true => DMAEN::Enabled,
        }
    }
    ///Dma disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN::Disabled
    }
    ///Dma enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN::Enabled
    }
}
///Field `DMAEN` writer - DMA enable bit
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Dma disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Disabled)
    }
    ///Dma enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Enabled)
    }
}
///Field `DBLOCKSIZE` reader - Data block size
pub type DBLOCKSIZE_R = crate::FieldReader;
///Field `DBLOCKSIZE` writer - Data block size
pub type DBLOCKSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
/**Read wait start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWSTART {
    ///0: Don't start read wait operation
    Disabled = 0,
    ///1: Read wait operation starts
    Enabled = 1,
}
impl From<RWSTART> for bool {
    #[inline(always)]
    fn from(variant: RWSTART) -> Self {
        variant as u8 != 0
    }
}
///Field `RWSTART` reader - Read wait start
pub type RWSTART_R = crate::BitReader<RWSTART>;
impl RWSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RWSTART {
        match self.bits {
            false => RWSTART::Disabled,
            true => RWSTART::Enabled,
        }
    }
    ///Don't start read wait operation
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RWSTART::Disabled
    }
    ///Read wait operation starts
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RWSTART::Enabled
    }
}
///Field `RWSTART` writer - Read wait start
pub type RWSTART_W<'a, REG> = crate::BitWriter<'a, REG, RWSTART>;
impl<'a, REG> RWSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Don't start read wait operation
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RWSTART::Disabled)
    }
    ///Read wait operation starts
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RWSTART::Enabled)
    }
}
/**Read wait stop

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWSTOP {
    ///0: Read wait in progress if RWSTART is enabled
    Disabled = 0,
    ///1: Enable for read wait stop if RWSTART is enabled
    Enabled = 1,
}
impl From<RWSTOP> for bool {
    #[inline(always)]
    fn from(variant: RWSTOP) -> Self {
        variant as u8 != 0
    }
}
///Field `RWSTOP` reader - Read wait stop
pub type RWSTOP_R = crate::BitReader<RWSTOP>;
impl RWSTOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RWSTOP {
        match self.bits {
            false => RWSTOP::Disabled,
            true => RWSTOP::Enabled,
        }
    }
    ///Read wait in progress if RWSTART is enabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RWSTOP::Disabled
    }
    ///Enable for read wait stop if RWSTART is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RWSTOP::Enabled
    }
}
///Field `RWSTOP` writer - Read wait stop
pub type RWSTOP_W<'a, REG> = crate::BitWriter<'a, REG, RWSTOP>;
impl<'a, REG> RWSTOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Read wait in progress if RWSTART is enabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RWSTOP::Disabled)
    }
    ///Enable for read wait stop if RWSTART is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RWSTOP::Enabled)
    }
}
/**Read wait mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWMOD {
    ///0: Read wait control stopping using SDIO_D2
    D2 = 0,
    ///1: Read wait control using SDIO_CK
    Ck = 1,
}
impl From<RWMOD> for bool {
    #[inline(always)]
    fn from(variant: RWMOD) -> Self {
        variant as u8 != 0
    }
}
///Field `RWMOD` reader - Read wait mode
pub type RWMOD_R = crate::BitReader<RWMOD>;
impl RWMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RWMOD {
        match self.bits {
            false => RWMOD::D2,
            true => RWMOD::Ck,
        }
    }
    ///Read wait control stopping using SDIO_D2
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == RWMOD::D2
    }
    ///Read wait control using SDIO_CK
    #[inline(always)]
    pub fn is_ck(&self) -> bool {
        *self == RWMOD::Ck
    }
}
///Field `RWMOD` writer - Read wait mode
pub type RWMOD_W<'a, REG> = crate::BitWriter<'a, REG, RWMOD>;
impl<'a, REG> RWMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Read wait control stopping using SDIO_D2
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(RWMOD::D2)
    }
    ///Read wait control using SDIO_CK
    #[inline(always)]
    pub fn ck(self) -> &'a mut crate::W<REG> {
        self.variant(RWMOD::Ck)
    }
}
/**SD I/O enable functions

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIOEN {
    ///0: SDIO operations disabled
    Disabled = 0,
    ///1: SDIO operations enabled
    Enabled = 1,
}
impl From<SDIOEN> for bool {
    #[inline(always)]
    fn from(variant: SDIOEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SDIOEN` reader - SD I/O enable functions
pub type SDIOEN_R = crate::BitReader<SDIOEN>;
impl SDIOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDIOEN {
        match self.bits {
            false => SDIOEN::Disabled,
            true => SDIOEN::Enabled,
        }
    }
    ///SDIO operations disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDIOEN::Disabled
    }
    ///SDIO operations enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDIOEN::Enabled
    }
}
///Field `SDIOEN` writer - SD I/O enable functions
pub type SDIOEN_W<'a, REG> = crate::BitWriter<'a, REG, SDIOEN>;
impl<'a, REG> SDIOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SDIO operations disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDIOEN::Disabled)
    }
    ///SDIO operations enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDIOEN::Enabled)
    }
}
impl R {
    ///Bit 0 - DTEN
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data transfer direction selection
    #[inline(always)]
    pub fn dtdir(&self) -> DTDIR_R {
        DTDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Data transfer mode selection 1: Stream or SDIO multibyte data transfer.
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DMA enable bit
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - Data block size
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - Read wait start
    #[inline(always)]
    pub fn rwstart(&self) -> RWSTART_R {
        RWSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Read wait stop
    #[inline(always)]
    pub fn rwstop(&self) -> RWSTOP_R {
        RWSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Read wait mode
    #[inline(always)]
    pub fn rwmod(&self) -> RWMOD_R {
        RWMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SD I/O enable functions
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCTRL")
            .field("sdioen", &self.sdioen())
            .field("rwmod", &self.rwmod())
            .field("rwstop", &self.rwstop())
            .field("rwstart", &self.rwstart())
            .field("dblocksize", &self.dblocksize())
            .field("dmaen", &self.dmaen())
            .field("dtmode", &self.dtmode())
            .field("dtdir", &self.dtdir())
            .field("dten", &self.dten())
            .finish()
    }
}
impl W {
    ///Bit 0 - DTEN
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W<'_, DCTRLrs> {
        DTEN_W::new(self, 0)
    }
    ///Bit 1 - Data transfer direction selection
    #[inline(always)]
    pub fn dtdir(&mut self) -> DTDIR_W<'_, DCTRLrs> {
        DTDIR_W::new(self, 1)
    }
    ///Bit 2 - Data transfer mode selection 1: Stream or SDIO multibyte data transfer.
    #[inline(always)]
    pub fn dtmode(&mut self) -> DTMODE_W<'_, DCTRLrs> {
        DTMODE_W::new(self, 2)
    }
    ///Bit 3 - DMA enable bit
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, DCTRLrs> {
        DMAEN_W::new(self, 3)
    }
    ///Bits 4:7 - Data block size
    #[inline(always)]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W<'_, DCTRLrs> {
        DBLOCKSIZE_W::new(self, 4)
    }
    ///Bit 8 - Read wait start
    #[inline(always)]
    pub fn rwstart(&mut self) -> RWSTART_W<'_, DCTRLrs> {
        RWSTART_W::new(self, 8)
    }
    ///Bit 9 - Read wait stop
    #[inline(always)]
    pub fn rwstop(&mut self) -> RWSTOP_W<'_, DCTRLrs> {
        RWSTOP_W::new(self, 9)
    }
    ///Bit 10 - Read wait mode
    #[inline(always)]
    pub fn rwmod(&mut self) -> RWMOD_W<'_, DCTRLrs> {
        RWMOD_W::new(self, 10)
    }
    ///Bit 11 - SD I/O enable functions
    #[inline(always)]
    pub fn sdioen(&mut self) -> SDIOEN_W<'_, DCTRLrs> {
        SDIOEN_W::new(self, 11)
    }
}
/**data control register

You can [`read`](crate::Reg::read) this register and get [`dctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#SDIO:DCTRL)*/
pub struct DCTRLrs;
impl crate::RegisterSpec for DCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`dctrl::R`](R) reader structure
impl crate::Readable for DCTRLrs {}
///`write(|w| ..)` method takes [`dctrl::W`](W) writer structure
impl crate::Writable for DCTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCTRL to value 0
impl crate::Resettable for DCTRLrs {}
