#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "Capture complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAME_IE {
    #[doc = "0: No interrupt generation"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated at the end of each received frame/crop window (in crop mode)"]
    Enabled = 1,
}
impl From<FRAME_IE> for bool {
    #[inline(always)]
    fn from(variant: FRAME_IE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAME_IE` reader - Capture complete interrupt enable"]
pub type FRAME_IE_R = crate::BitReader<FRAME_IE>;
impl FRAME_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FRAME_IE {
        match self.bits {
            false => FRAME_IE::Disabled,
            true => FRAME_IE::Enabled,
        }
    }
    #[doc = "No interrupt generation"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FRAME_IE::Disabled
    }
    #[doc = "An interrupt is generated at the end of each received frame/crop window (in crop mode)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FRAME_IE::Enabled
    }
}
#[doc = "Field `FRAME_IE` writer - Capture complete interrupt enable"]
pub type FRAME_IE_W<'a, REG> = crate::BitWriter<'a, REG, FRAME_IE>;
impl<'a, REG> FRAME_IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt generation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FRAME_IE::Disabled)
    }
    #[doc = "An interrupt is generated at the end of each received frame/crop window (in crop mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FRAME_IE::Enabled)
    }
}
#[doc = "Overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_IE {
    #[doc = "0: No interrupt generation"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated if the DMA was not able to transfer the last data before new data (32-bit) are received"]
    Enabled = 1,
}
impl From<OVR_IE> for bool {
    #[inline(always)]
    fn from(variant: OVR_IE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR_IE` reader - Overrun interrupt enable"]
pub type OVR_IE_R = crate::BitReader<OVR_IE>;
impl OVR_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVR_IE {
        match self.bits {
            false => OVR_IE::Disabled,
            true => OVR_IE::Enabled,
        }
    }
    #[doc = "No interrupt generation"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVR_IE::Disabled
    }
    #[doc = "An interrupt is generated if the DMA was not able to transfer the last data before new data (32-bit) are received"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVR_IE::Enabled
    }
}
#[doc = "Field `OVR_IE` writer - Overrun interrupt enable"]
pub type OVR_IE_W<'a, REG> = crate::BitWriter<'a, REG, OVR_IE>;
impl<'a, REG> OVR_IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt generation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVR_IE::Disabled)
    }
    #[doc = "An interrupt is generated if the DMA was not able to transfer the last data before new data (32-bit) are received"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVR_IE::Enabled)
    }
}
#[doc = "Synchronization error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_IE {
    #[doc = "0: No interrupt generation"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated if the embedded synchronization codes are not received in the correct order"]
    Enabled = 1,
}
impl From<ERR_IE> for bool {
    #[inline(always)]
    fn from(variant: ERR_IE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR_IE` reader - Synchronization error interrupt enable"]
pub type ERR_IE_R = crate::BitReader<ERR_IE>;
impl ERR_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERR_IE {
        match self.bits {
            false => ERR_IE::Disabled,
            true => ERR_IE::Enabled,
        }
    }
    #[doc = "No interrupt generation"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERR_IE::Disabled
    }
    #[doc = "An interrupt is generated if the embedded synchronization codes are not received in the correct order"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERR_IE::Enabled
    }
}
#[doc = "Field `ERR_IE` writer - Synchronization error interrupt enable"]
pub type ERR_IE_W<'a, REG> = crate::BitWriter<'a, REG, ERR_IE>;
impl<'a, REG> ERR_IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt generation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_IE::Disabled)
    }
    #[doc = "An interrupt is generated if the embedded synchronization codes are not received in the correct order"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_IE::Enabled)
    }
}
#[doc = "VSYNC interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSYNC_IE {
    #[doc = "0: No interrupt generation"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated on each DCMI_VSYNC transition from the inactive to the active state"]
    Enabled = 1,
}
impl From<VSYNC_IE> for bool {
    #[inline(always)]
    fn from(variant: VSYNC_IE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSYNC_IE` reader - VSYNC interrupt enable"]
pub type VSYNC_IE_R = crate::BitReader<VSYNC_IE>;
impl VSYNC_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VSYNC_IE {
        match self.bits {
            false => VSYNC_IE::Disabled,
            true => VSYNC_IE::Enabled,
        }
    }
    #[doc = "No interrupt generation"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VSYNC_IE::Disabled
    }
    #[doc = "An interrupt is generated on each DCMI_VSYNC transition from the inactive to the active state"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VSYNC_IE::Enabled
    }
}
#[doc = "Field `VSYNC_IE` writer - VSYNC interrupt enable"]
pub type VSYNC_IE_W<'a, REG> = crate::BitWriter<'a, REG, VSYNC_IE>;
impl<'a, REG> VSYNC_IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt generation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VSYNC_IE::Disabled)
    }
    #[doc = "An interrupt is generated on each DCMI_VSYNC transition from the inactive to the active state"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VSYNC_IE::Enabled)
    }
}
#[doc = "Line interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINE_IE {
    #[doc = "0: No interrupt generation when the line is received"]
    Disabled = 0,
    #[doc = "1: An Interrupt is generated when a line has been completely received"]
    Enabled = 1,
}
impl From<LINE_IE> for bool {
    #[inline(always)]
    fn from(variant: LINE_IE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE_IE` reader - Line interrupt enable"]
pub type LINE_IE_R = crate::BitReader<LINE_IE>;
impl LINE_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LINE_IE {
        match self.bits {
            false => LINE_IE::Disabled,
            true => LINE_IE::Enabled,
        }
    }
    #[doc = "No interrupt generation when the line is received"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LINE_IE::Disabled
    }
    #[doc = "An Interrupt is generated when a line has been completely received"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LINE_IE::Enabled
    }
}
#[doc = "Field `LINE_IE` writer - Line interrupt enable"]
pub type LINE_IE_W<'a, REG> = crate::BitWriter<'a, REG, LINE_IE>;
impl<'a, REG> LINE_IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt generation when the line is received"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LINE_IE::Disabled)
    }
    #[doc = "An Interrupt is generated when a line has been completely received"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LINE_IE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Capture complete interrupt enable"]
    #[inline(always)]
    pub fn frame_ie(&self) -> FRAME_IE_R {
        FRAME_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun interrupt enable"]
    #[inline(always)]
    pub fn ovr_ie(&self) -> OVR_IE_R {
        OVR_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization error interrupt enable"]
    #[inline(always)]
    pub fn err_ie(&self) -> ERR_IE_R {
        ERR_IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSYNC interrupt enable"]
    #[inline(always)]
    pub fn vsync_ie(&self) -> VSYNC_IE_R {
        VSYNC_IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line interrupt enable"]
    #[inline(always)]
    pub fn line_ie(&self) -> LINE_IE_R {
        LINE_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn frame_ie(&mut self) -> FRAME_IE_W<IERrs> {
        FRAME_IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr_ie(&mut self) -> OVR_IE_W<IERrs> {
        OVR_IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronization error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_ie(&mut self) -> ERR_IE_W<IERrs> {
        ERR_IE_W::new(self, 2)
    }
    #[doc = "Bit 3 - VSYNC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn vsync_ie(&mut self) -> VSYNC_IE_W<IERrs> {
        VSYNC_IE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Line interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn line_ie(&mut self) -> LINE_IE_W<IERrs> {
        LINE_IE_W::new(self, 4)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IERrs {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
