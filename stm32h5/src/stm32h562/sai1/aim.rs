#[doc = "Register `AIM` reader"]
pub type R = crate::R<AIMrs>;
#[doc = "Register `AIM` writer"]
pub type W = crate::W<AIMrs>;
#[doc = "Overrun/underrun interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the OVRUDR bit in the SAI_xSR register is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRUDRIE {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is enabled"]
    Enabled = 1,
}
impl From<OVRUDRIE> for bool {
    #[inline(always)]
    fn from(variant: OVRUDRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRUDRIE` reader - Overrun/underrun interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the OVRUDR bit in the SAI_xSR register is set."]
pub type OVRUDRIE_R = crate::BitReader<OVRUDRIE>;
impl OVRUDRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVRUDRIE {
        match self.bits {
            false => OVRUDRIE::Disabled,
            true => OVRUDRIE::Enabled,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRUDRIE::Disabled
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRUDRIE::Enabled
    }
}
#[doc = "Field `OVRUDRIE` writer - Overrun/underrun interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the OVRUDR bit in the SAI_xSR register is set."]
pub type OVRUDRIE_W<'a, REG> = crate::BitWriter<'a, REG, OVRUDRIE>;
impl<'a, REG> OVRUDRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVRUDRIE::Disabled)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVRUDRIE::Enabled)
    }
}
#[doc = "Mute detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the MUTEDET bit in the SAI_xSR register is set. This bit has a meaning only if the audio block is configured in receiver mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUTEDETIE {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is enabled"]
    Enabled = 1,
}
impl From<MUTEDETIE> for bool {
    #[inline(always)]
    fn from(variant: MUTEDETIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUTEDETIE` reader - Mute detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the MUTEDET bit in the SAI_xSR register is set. This bit has a meaning only if the audio block is configured in receiver mode."]
pub type MUTEDETIE_R = crate::BitReader<MUTEDETIE>;
impl MUTEDETIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MUTEDETIE {
        match self.bits {
            false => MUTEDETIE::Disabled,
            true => MUTEDETIE::Enabled,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MUTEDETIE::Disabled
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MUTEDETIE::Enabled
    }
}
#[doc = "Field `MUTEDETIE` writer - Mute detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the MUTEDET bit in the SAI_xSR register is set. This bit has a meaning only if the audio block is configured in receiver mode."]
pub type MUTEDETIE_W<'a, REG> = crate::BitWriter<'a, REG, MUTEDETIE>;
impl<'a, REG> MUTEDETIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MUTEDETIE::Disabled)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MUTEDETIE::Enabled)
    }
}
#[doc = "Wrong clock configuration interrupt enable. This bit is set and cleared by software. This bit is taken into account only if the audio block is configured as a master (MODE\\[1\\]
= 0) and NODIV = 0. It generates an interrupt if the WCKCFG flag in the SAI_xSR register is set. Note: This bit is used only in Free protocol mode and is meaningless in other modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WCKCFGIE {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is enabled"]
    Enabled = 1,
}
impl From<WCKCFGIE> for bool {
    #[inline(always)]
    fn from(variant: WCKCFGIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCKCFGIE` reader - Wrong clock configuration interrupt enable. This bit is set and cleared by software. This bit is taken into account only if the audio block is configured as a master (MODE\\[1\\]
= 0) and NODIV = 0. It generates an interrupt if the WCKCFG flag in the SAI_xSR register is set. Note: This bit is used only in Free protocol mode and is meaningless in other modes."]
pub type WCKCFGIE_R = crate::BitReader<WCKCFGIE>;
impl WCKCFGIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WCKCFGIE {
        match self.bits {
            false => WCKCFGIE::Disabled,
            true => WCKCFGIE::Enabled,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WCKCFGIE::Disabled
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WCKCFGIE::Enabled
    }
}
#[doc = "Field `WCKCFGIE` writer - Wrong clock configuration interrupt enable. This bit is set and cleared by software. This bit is taken into account only if the audio block is configured as a master (MODE\\[1\\]
= 0) and NODIV = 0. It generates an interrupt if the WCKCFG flag in the SAI_xSR register is set. Note: This bit is used only in Free protocol mode and is meaningless in other modes."]
pub type WCKCFGIE_W<'a, REG> = crate::BitWriter<'a, REG, WCKCFGIE>;
impl<'a, REG> WCKCFGIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WCKCFGIE::Disabled)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WCKCFGIE::Enabled)
    }
}
#[doc = "FIFO request interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the FREQ bit in the SAI_xSR register is set. Since the audio block defaults to operate as a transmitter after reset, the MODE bit must be configured before setting FREQIE to avoid a parasitic interrupt in receiver mode,\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FREQIE {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is enabled"]
    Enabled = 1,
}
impl From<FREQIE> for bool {
    #[inline(always)]
    fn from(variant: FREQIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREQIE` reader - FIFO request interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the FREQ bit in the SAI_xSR register is set. Since the audio block defaults to operate as a transmitter after reset, the MODE bit must be configured before setting FREQIE to avoid a parasitic interrupt in receiver mode,"]
pub type FREQIE_R = crate::BitReader<FREQIE>;
impl FREQIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FREQIE {
        match self.bits {
            false => FREQIE::Disabled,
            true => FREQIE::Enabled,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FREQIE::Disabled
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FREQIE::Enabled
    }
}
#[doc = "Field `FREQIE` writer - FIFO request interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the FREQ bit in the SAI_xSR register is set. Since the audio block defaults to operate as a transmitter after reset, the MODE bit must be configured before setting FREQIE to avoid a parasitic interrupt in receiver mode,"]
pub type FREQIE_W<'a, REG> = crate::BitWriter<'a, REG, FREQIE>;
impl<'a, REG> FREQIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FREQIE::Disabled)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FREQIE::Enabled)
    }
}
#[doc = "Codec not ready interrupt enable (AC’97). This bit is set and cleared by software. When the interrupt is enabled, the audio block detects in the slot 0 (tag0) of the AC’97 frame if the Codec connected to this line is ready or not. If it is not ready, the CNRDY flag in the SAI_xSR register is set and an interrupt is generated. This bit has a meaning only if the AC’97 mode is selected through PRTCFG\\[1:0\\]
bits and the audio block is operates as a receiver.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNRDYIE {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is enabled"]
    Enabled = 1,
}
impl From<CNRDYIE> for bool {
    #[inline(always)]
    fn from(variant: CNRDYIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNRDYIE` reader - Codec not ready interrupt enable (AC’97). This bit is set and cleared by software. When the interrupt is enabled, the audio block detects in the slot 0 (tag0) of the AC’97 frame if the Codec connected to this line is ready or not. If it is not ready, the CNRDY flag in the SAI_xSR register is set and an interrupt is generated. This bit has a meaning only if the AC’97 mode is selected through PRTCFG\\[1:0\\]
bits and the audio block is operates as a receiver."]
pub type CNRDYIE_R = crate::BitReader<CNRDYIE>;
impl CNRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CNRDYIE {
        match self.bits {
            false => CNRDYIE::Disabled,
            true => CNRDYIE::Enabled,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CNRDYIE::Disabled
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CNRDYIE::Enabled
    }
}
#[doc = "Field `CNRDYIE` writer - Codec not ready interrupt enable (AC’97). This bit is set and cleared by software. When the interrupt is enabled, the audio block detects in the slot 0 (tag0) of the AC’97 frame if the Codec connected to this line is ready or not. If it is not ready, the CNRDY flag in the SAI_xSR register is set and an interrupt is generated. This bit has a meaning only if the AC’97 mode is selected through PRTCFG\\[1:0\\]
bits and the audio block is operates as a receiver."]
pub type CNRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, CNRDYIE>;
impl<'a, REG> CNRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CNRDYIE::Disabled)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CNRDYIE::Enabled)
    }
}
#[doc = "Anticipated frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the AFSDET bit in the SAI_xSR register is set. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFSDETIE {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is enabled"]
    Enabled = 1,
}
impl From<AFSDETIE> for bool {
    #[inline(always)]
    fn from(variant: AFSDETIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AFSDETIE` reader - Anticipated frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the AFSDET bit in the SAI_xSR register is set. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master."]
pub type AFSDETIE_R = crate::BitReader<AFSDETIE>;
impl AFSDETIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSDETIE {
        match self.bits {
            false => AFSDETIE::Disabled,
            true => AFSDETIE::Enabled,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AFSDETIE::Disabled
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AFSDETIE::Enabled
    }
}
#[doc = "Field `AFSDETIE` writer - Anticipated frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the AFSDET bit in the SAI_xSR register is set. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master."]
pub type AFSDETIE_W<'a, REG> = crate::BitWriter<'a, REG, AFSDETIE>;
impl<'a, REG> AFSDETIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AFSDETIE::Disabled)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AFSDETIE::Enabled)
    }
}
#[doc = "Late frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the LFSDET bit is set in the SAI_xSR register. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFSDETIE {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is enabled"]
    Enabled = 1,
}
impl From<LFSDETIE> for bool {
    #[inline(always)]
    fn from(variant: LFSDETIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFSDETIE` reader - Late frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the LFSDET bit is set in the SAI_xSR register. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master."]
pub type LFSDETIE_R = crate::BitReader<LFSDETIE>;
impl LFSDETIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LFSDETIE {
        match self.bits {
            false => LFSDETIE::Disabled,
            true => LFSDETIE::Enabled,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFSDETIE::Disabled
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LFSDETIE::Enabled
    }
}
#[doc = "Field `LFSDETIE` writer - Late frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the LFSDET bit is set in the SAI_xSR register. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master."]
pub type LFSDETIE_W<'a, REG> = crate::BitWriter<'a, REG, LFSDETIE>;
impl<'a, REG> LFSDETIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFSDETIE::Disabled)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFSDETIE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Overrun/underrun interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the OVRUDR bit in the SAI_xSR register is set."]
    #[inline(always)]
    pub fn ovrudrie(&self) -> OVRUDRIE_R {
        OVRUDRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mute detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the MUTEDET bit in the SAI_xSR register is set. This bit has a meaning only if the audio block is configured in receiver mode."]
    #[inline(always)]
    pub fn mutedetie(&self) -> MUTEDETIE_R {
        MUTEDETIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wrong clock configuration interrupt enable. This bit is set and cleared by software. This bit is taken into account only if the audio block is configured as a master (MODE\\[1\\]
= 0) and NODIV = 0. It generates an interrupt if the WCKCFG flag in the SAI_xSR register is set. Note: This bit is used only in Free protocol mode and is meaningless in other modes."]
    #[inline(always)]
    pub fn wckcfgie(&self) -> WCKCFGIE_R {
        WCKCFGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO request interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the FREQ bit in the SAI_xSR register is set. Since the audio block defaults to operate as a transmitter after reset, the MODE bit must be configured before setting FREQIE to avoid a parasitic interrupt in receiver mode,"]
    #[inline(always)]
    pub fn freqie(&self) -> FREQIE_R {
        FREQIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Codec not ready interrupt enable (AC’97). This bit is set and cleared by software. When the interrupt is enabled, the audio block detects in the slot 0 (tag0) of the AC’97 frame if the Codec connected to this line is ready or not. If it is not ready, the CNRDY flag in the SAI_xSR register is set and an interrupt is generated. This bit has a meaning only if the AC’97 mode is selected through PRTCFG\\[1:0\\]
bits and the audio block is operates as a receiver."]
    #[inline(always)]
    pub fn cnrdyie(&self) -> CNRDYIE_R {
        CNRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the AFSDET bit in the SAI_xSR register is set. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master."]
    #[inline(always)]
    pub fn afsdetie(&self) -> AFSDETIE_R {
        AFSDETIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Late frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the LFSDET bit is set in the SAI_xSR register. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master."]
    #[inline(always)]
    pub fn lfsdetie(&self) -> LFSDETIE_R {
        LFSDETIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overrun/underrun interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the OVRUDR bit in the SAI_xSR register is set."]
    #[inline(always)]
    #[must_use]
    pub fn ovrudrie(&mut self) -> OVRUDRIE_W<AIMrs> {
        OVRUDRIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Mute detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the MUTEDET bit in the SAI_xSR register is set. This bit has a meaning only if the audio block is configured in receiver mode."]
    #[inline(always)]
    #[must_use]
    pub fn mutedetie(&mut self) -> MUTEDETIE_W<AIMrs> {
        MUTEDETIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wrong clock configuration interrupt enable. This bit is set and cleared by software. This bit is taken into account only if the audio block is configured as a master (MODE\\[1\\]
= 0) and NODIV = 0. It generates an interrupt if the WCKCFG flag in the SAI_xSR register is set. Note: This bit is used only in Free protocol mode and is meaningless in other modes."]
    #[inline(always)]
    #[must_use]
    pub fn wckcfgie(&mut self) -> WCKCFGIE_W<AIMrs> {
        WCKCFGIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - FIFO request interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the FREQ bit in the SAI_xSR register is set. Since the audio block defaults to operate as a transmitter after reset, the MODE bit must be configured before setting FREQIE to avoid a parasitic interrupt in receiver mode,"]
    #[inline(always)]
    #[must_use]
    pub fn freqie(&mut self) -> FREQIE_W<AIMrs> {
        FREQIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Codec not ready interrupt enable (AC’97). This bit is set and cleared by software. When the interrupt is enabled, the audio block detects in the slot 0 (tag0) of the AC’97 frame if the Codec connected to this line is ready or not. If it is not ready, the CNRDY flag in the SAI_xSR register is set and an interrupt is generated. This bit has a meaning only if the AC’97 mode is selected through PRTCFG\\[1:0\\]
bits and the audio block is operates as a receiver."]
    #[inline(always)]
    #[must_use]
    pub fn cnrdyie(&mut self) -> CNRDYIE_W<AIMrs> {
        CNRDYIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the AFSDET bit in the SAI_xSR register is set. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master."]
    #[inline(always)]
    #[must_use]
    pub fn afsdetie(&mut self) -> AFSDETIE_W<AIMrs> {
        AFSDETIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Late frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the LFSDET bit is set in the SAI_xSR register. This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master."]
    #[inline(always)]
    #[must_use]
    pub fn lfsdetie(&mut self) -> LFSDETIE_W<AIMrs> {
        LFSDETIE_W::new(self, 6)
    }
}
#[doc = "SAI interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AIMrs;
impl crate::RegisterSpec for AIMrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aim::R`](R) reader structure"]
impl crate::Readable for AIMrs {}
#[doc = "`write(|w| ..)` method takes [`aim::W`](W) writer structure"]
impl crate::Writable for AIMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIM to value 0"]
impl crate::Resettable for AIMrs {
    const RESET_VALUE: u32 = 0;
}
