#[doc = "Register `DCTRL` reader"]
pub type R = crate::R<DCTRLrs>;
#[doc = "Register `DCTRL` writer"]
pub type W = crate::W<DCTRLrs>;
#[doc = "DTEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEN {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Start transfer"]
    Enabled = 1,
}
impl From<DTEN> for bool {
    #[inline(always)]
    fn from(variant: DTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTEN` reader - DTEN"]
pub type DTEN_R = crate::BitReader<DTEN>;
impl DTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTEN {
        match self.bits {
            false => DTEN::Disabled,
            true => DTEN::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTEN::Disabled
    }
    #[doc = "Start transfer"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTEN::Enabled
    }
}
#[doc = "Field `DTEN` writer - DTEN"]
pub type DTEN_W<'a, REG> = crate::BitWriter<'a, REG, DTEN>;
impl<'a, REG> DTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTEN::Disabled)
    }
    #[doc = "Start transfer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTEN::Enabled)
    }
}
#[doc = "Data transfer direction selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTDIR {
    #[doc = "0: From controller to card"]
    ControllerToCard = 0,
    #[doc = "1: From card to controller"]
    CardToController = 1,
}
impl From<DTDIR> for bool {
    #[inline(always)]
    fn from(variant: DTDIR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTDIR` reader - Data transfer direction selection"]
pub type DTDIR_R = crate::BitReader<DTDIR>;
impl DTDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTDIR {
        match self.bits {
            false => DTDIR::ControllerToCard,
            true => DTDIR::CardToController,
        }
    }
    #[doc = "From controller to card"]
    #[inline(always)]
    pub fn is_controller_to_card(&self) -> bool {
        *self == DTDIR::ControllerToCard
    }
    #[doc = "From card to controller"]
    #[inline(always)]
    pub fn is_card_to_controller(&self) -> bool {
        *self == DTDIR::CardToController
    }
}
#[doc = "Field `DTDIR` writer - Data transfer direction selection"]
pub type DTDIR_W<'a, REG> = crate::BitWriter<'a, REG, DTDIR>;
impl<'a, REG> DTDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "From controller to card"]
    #[inline(always)]
    pub fn controller_to_card(self) -> &'a mut crate::W<REG> {
        self.variant(DTDIR::ControllerToCard)
    }
    #[doc = "From card to controller"]
    #[inline(always)]
    pub fn card_to_controller(self) -> &'a mut crate::W<REG> {
        self.variant(DTDIR::CardToController)
    }
}
#[doc = "Data transfer mode selection 1: Stream or SDIO multibyte data transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTMODE {
    #[doc = "0: Bloack data transfer"]
    BlockMode = 0,
    #[doc = "1: Stream or SDIO multibyte data transfer"]
    StreamMode = 1,
}
impl From<DTMODE> for bool {
    #[inline(always)]
    fn from(variant: DTMODE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTMODE` reader - Data transfer mode selection 1: Stream or SDIO multibyte data transfer."]
pub type DTMODE_R = crate::BitReader<DTMODE>;
impl DTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTMODE {
        match self.bits {
            false => DTMODE::BlockMode,
            true => DTMODE::StreamMode,
        }
    }
    #[doc = "Bloack data transfer"]
    #[inline(always)]
    pub fn is_block_mode(&self) -> bool {
        *self == DTMODE::BlockMode
    }
    #[doc = "Stream or SDIO multibyte data transfer"]
    #[inline(always)]
    pub fn is_stream_mode(&self) -> bool {
        *self == DTMODE::StreamMode
    }
}
#[doc = "Field `DTMODE` writer - Data transfer mode selection 1: Stream or SDIO multibyte data transfer."]
pub type DTMODE_W<'a, REG> = crate::BitWriter<'a, REG, DTMODE>;
impl<'a, REG> DTMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bloack data transfer"]
    #[inline(always)]
    pub fn block_mode(self) -> &'a mut crate::W<REG> {
        self.variant(DTMODE::BlockMode)
    }
    #[doc = "Stream or SDIO multibyte data transfer"]
    #[inline(always)]
    pub fn stream_mode(self) -> &'a mut crate::W<REG> {
        self.variant(DTMODE::StreamMode)
    }
}
#[doc = "DMA enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN {
    #[doc = "0: Dma disabled"]
    Disabled = 0,
    #[doc = "1: Dma enabled"]
    Enabled = 1,
}
impl From<DMAEN> for bool {
    #[inline(always)]
    fn from(variant: DMAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA enable bit"]
pub type DMAEN_R = crate::BitReader<DMAEN>;
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN {
        match self.bits {
            false => DMAEN::Disabled,
            true => DMAEN::Enabled,
        }
    }
    #[doc = "Dma disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN::Disabled
    }
    #[doc = "Dma enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN::Enabled
    }
}
#[doc = "Field `DMAEN` writer - DMA enable bit"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dma disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Disabled)
    }
    #[doc = "Dma enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Enabled)
    }
}
#[doc = "Field `DBLOCKSIZE` reader - Data block size"]
pub type DBLOCKSIZE_R = crate::FieldReader;
#[doc = "Field `DBLOCKSIZE` writer - Data block size"]
pub type DBLOCKSIZE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Read wait start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWSTART {
    #[doc = "0: Don't start read wait operation"]
    Disabled = 0,
    #[doc = "1: Read wait operation starts"]
    Enabled = 1,
}
impl From<RWSTART> for bool {
    #[inline(always)]
    fn from(variant: RWSTART) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWSTART` reader - Read wait start"]
pub type RWSTART_R = crate::BitReader<RWSTART>;
impl RWSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RWSTART {
        match self.bits {
            false => RWSTART::Disabled,
            true => RWSTART::Enabled,
        }
    }
    #[doc = "Don't start read wait operation"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RWSTART::Disabled
    }
    #[doc = "Read wait operation starts"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RWSTART::Enabled
    }
}
#[doc = "Field `RWSTART` writer - Read wait start"]
pub type RWSTART_W<'a, REG> = crate::BitWriter<'a, REG, RWSTART>;
impl<'a, REG> RWSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Don't start read wait operation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RWSTART::Disabled)
    }
    #[doc = "Read wait operation starts"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RWSTART::Enabled)
    }
}
#[doc = "Read wait stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWSTOP {
    #[doc = "0: Read wait in progress if RWSTART is enabled"]
    Disabled = 0,
    #[doc = "1: Enable for read wait stop if RWSTART is enabled"]
    Enabled = 1,
}
impl From<RWSTOP> for bool {
    #[inline(always)]
    fn from(variant: RWSTOP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWSTOP` reader - Read wait stop"]
pub type RWSTOP_R = crate::BitReader<RWSTOP>;
impl RWSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RWSTOP {
        match self.bits {
            false => RWSTOP::Disabled,
            true => RWSTOP::Enabled,
        }
    }
    #[doc = "Read wait in progress if RWSTART is enabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RWSTOP::Disabled
    }
    #[doc = "Enable for read wait stop if RWSTART is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RWSTOP::Enabled
    }
}
#[doc = "Field `RWSTOP` writer - Read wait stop"]
pub type RWSTOP_W<'a, REG> = crate::BitWriter<'a, REG, RWSTOP>;
impl<'a, REG> RWSTOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read wait in progress if RWSTART is enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RWSTOP::Disabled)
    }
    #[doc = "Enable for read wait stop if RWSTART is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RWSTOP::Enabled)
    }
}
#[doc = "Read wait mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWMOD {
    #[doc = "0: Read wait control stopping using SDIO_D2"]
    D2 = 0,
    #[doc = "1: Read wait control using SDIO_CK"]
    Ck = 1,
}
impl From<RWMOD> for bool {
    #[inline(always)]
    fn from(variant: RWMOD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWMOD` reader - Read wait mode"]
pub type RWMOD_R = crate::BitReader<RWMOD>;
impl RWMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RWMOD {
        match self.bits {
            false => RWMOD::D2,
            true => RWMOD::Ck,
        }
    }
    #[doc = "Read wait control stopping using SDIO_D2"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == RWMOD::D2
    }
    #[doc = "Read wait control using SDIO_CK"]
    #[inline(always)]
    pub fn is_ck(&self) -> bool {
        *self == RWMOD::Ck
    }
}
#[doc = "Field `RWMOD` writer - Read wait mode"]
pub type RWMOD_W<'a, REG> = crate::BitWriter<'a, REG, RWMOD>;
impl<'a, REG> RWMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read wait control stopping using SDIO_D2"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(RWMOD::D2)
    }
    #[doc = "Read wait control using SDIO_CK"]
    #[inline(always)]
    pub fn ck(self) -> &'a mut crate::W<REG> {
        self.variant(RWMOD::Ck)
    }
}
#[doc = "SD I/O enable functions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIOEN {
    #[doc = "0: SDIO operations disabled"]
    Disabled = 0,
    #[doc = "1: SDIO operations enabled"]
    Enabled = 1,
}
impl From<SDIOEN> for bool {
    #[inline(always)]
    fn from(variant: SDIOEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIOEN` reader - SD I/O enable functions"]
pub type SDIOEN_R = crate::BitReader<SDIOEN>;
impl SDIOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDIOEN {
        match self.bits {
            false => SDIOEN::Disabled,
            true => SDIOEN::Enabled,
        }
    }
    #[doc = "SDIO operations disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDIOEN::Disabled
    }
    #[doc = "SDIO operations enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDIOEN::Enabled
    }
}
#[doc = "Field `SDIOEN` writer - SD I/O enable functions"]
pub type SDIOEN_W<'a, REG> = crate::BitWriter<'a, REG, SDIOEN>;
impl<'a, REG> SDIOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDIO operations disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDIOEN::Disabled)
    }
    #[doc = "SDIO operations enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDIOEN::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data transfer direction selection"]
    #[inline(always)]
    pub fn dtdir(&self) -> DTDIR_R {
        DTDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data transfer mode selection 1: Stream or SDIO multibyte data transfer."]
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA enable bit"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Data block size"]
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Read wait start"]
    #[inline(always)]
    pub fn rwstart(&self) -> RWSTART_R {
        RWSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read wait stop"]
    #[inline(always)]
    pub fn rwstop(&self) -> RWSTOP_R {
        RWSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Read wait mode"]
    #[inline(always)]
    pub fn rwmod(&self) -> RWMOD_R {
        RWMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD I/O enable functions"]
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<DCTRLrs> {
        DTEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data transfer direction selection"]
    #[inline(always)]
    #[must_use]
    pub fn dtdir(&mut self) -> DTDIR_W<DCTRLrs> {
        DTDIR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Data transfer mode selection 1: Stream or SDIO multibyte data transfer."]
    #[inline(always)]
    #[must_use]
    pub fn dtmode(&mut self) -> DTMODE_W<DCTRLrs> {
        DTMODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<DCTRLrs> {
        DMAEN_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - Data block size"]
    #[inline(always)]
    #[must_use]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W<DCTRLrs> {
        DBLOCKSIZE_W::new(self, 4)
    }
    #[doc = "Bit 8 - Read wait start"]
    #[inline(always)]
    #[must_use]
    pub fn rwstart(&mut self) -> RWSTART_W<DCTRLrs> {
        RWSTART_W::new(self, 8)
    }
    #[doc = "Bit 9 - Read wait stop"]
    #[inline(always)]
    #[must_use]
    pub fn rwstop(&mut self) -> RWSTOP_W<DCTRLrs> {
        RWSTOP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Read wait mode"]
    #[inline(always)]
    #[must_use]
    pub fn rwmod(&mut self) -> RWMOD_W<DCTRLrs> {
        RWMOD_W::new(self, 10)
    }
    #[doc = "Bit 11 - SD I/O enable functions"]
    #[inline(always)]
    #[must_use]
    pub fn sdioen(&mut self) -> SDIOEN_W<DCTRLrs> {
        SDIOEN_W::new(self, 11)
    }
}
#[doc = "data control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCTRLrs;
impl crate::RegisterSpec for DCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctrl::R`](R) reader structure"]
impl crate::Readable for DCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`dctrl::W`](W) writer structure"]
impl crate::Writable for DCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCTRL to value 0"]
impl crate::Resettable for DCTRLrs {
    const RESET_VALUE: u32 = 0;
}
