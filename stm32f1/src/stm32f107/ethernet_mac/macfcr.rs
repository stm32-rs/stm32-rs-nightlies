#[doc = "Register `MACFCR` reader"]
pub type R = crate::R<MACFCRrs>;
#[doc = "Register `MACFCR` writer"]
pub type W = crate::W<MACFCRrs>;
#[doc = "Flow control busy/back pressure activate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCB {
    #[doc = "0: In half duplex only, deasserts back pressure"]
    DisableBackPressure = 0,
    #[doc = "1: In full duplex, initiate a Pause control frame. In half duplex, assert back pressure"]
    PauseOrBackPressure = 1,
}
impl From<FCB> for bool {
    #[inline(always)]
    fn from(variant: FCB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCB` reader - Flow control busy/back pressure activate"]
pub type FCB_R = crate::BitReader<FCB>;
impl FCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FCB {
        match self.bits {
            false => FCB::DisableBackPressure,
            true => FCB::PauseOrBackPressure,
        }
    }
    #[doc = "In half duplex only, deasserts back pressure"]
    #[inline(always)]
    pub fn is_disable_back_pressure(&self) -> bool {
        *self == FCB::DisableBackPressure
    }
    #[doc = "In full duplex, initiate a Pause control frame. In half duplex, assert back pressure"]
    #[inline(always)]
    pub fn is_pause_or_back_pressure(&self) -> bool {
        *self == FCB::PauseOrBackPressure
    }
}
#[doc = "Field `FCB` writer - Flow control busy/back pressure activate"]
pub type FCB_W<'a, REG> = crate::BitWriter<'a, REG, FCB>;
impl<'a, REG> FCB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In half duplex only, deasserts back pressure"]
    #[inline(always)]
    pub fn disable_back_pressure(self) -> &'a mut crate::W<REG> {
        self.variant(FCB::DisableBackPressure)
    }
    #[doc = "In full duplex, initiate a Pause control frame. In half duplex, assert back pressure"]
    #[inline(always)]
    pub fn pause_or_back_pressure(self) -> &'a mut crate::W<REG> {
        self.variant(FCB::PauseOrBackPressure)
    }
}
#[doc = "Transmit flow control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFCE {
    #[doc = "0: In full duplex, flow control is disabled. In half duplex, back pressure is disabled"]
    Disabled = 0,
    #[doc = "1: In full duplex, flow control is enabled. In half duplex, back pressure is enabled"]
    Enabled = 1,
}
impl From<TFCE> for bool {
    #[inline(always)]
    fn from(variant: TFCE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFCE` reader - Transmit flow control enable"]
pub type TFCE_R = crate::BitReader<TFCE>;
impl TFCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TFCE {
        match self.bits {
            false => TFCE::Disabled,
            true => TFCE::Enabled,
        }
    }
    #[doc = "In full duplex, flow control is disabled. In half duplex, back pressure is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TFCE::Disabled
    }
    #[doc = "In full duplex, flow control is enabled. In half duplex, back pressure is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TFCE::Enabled
    }
}
#[doc = "Field `TFCE` writer - Transmit flow control enable"]
pub type TFCE_W<'a, REG> = crate::BitWriter<'a, REG, TFCE>;
impl<'a, REG> TFCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In full duplex, flow control is disabled. In half duplex, back pressure is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TFCE::Disabled)
    }
    #[doc = "In full duplex, flow control is enabled. In half duplex, back pressure is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TFCE::Enabled)
    }
}
#[doc = "Receive flow control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFCE {
    #[doc = "0: Pause frames are not decoded"]
    Disabled = 0,
    #[doc = "1: MAC decodes received Pause frames and disables its transmitted for a specified time"]
    Enabled = 1,
}
impl From<RFCE> for bool {
    #[inline(always)]
    fn from(variant: RFCE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFCE` reader - Receive flow control enable"]
pub type RFCE_R = crate::BitReader<RFCE>;
impl RFCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFCE {
        match self.bits {
            false => RFCE::Disabled,
            true => RFCE::Enabled,
        }
    }
    #[doc = "Pause frames are not decoded"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RFCE::Disabled
    }
    #[doc = "MAC decodes received Pause frames and disables its transmitted for a specified time"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RFCE::Enabled
    }
}
#[doc = "Field `RFCE` writer - Receive flow control enable"]
pub type RFCE_W<'a, REG> = crate::BitWriter<'a, REG, RFCE>;
impl<'a, REG> RFCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pause frames are not decoded"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RFCE::Disabled)
    }
    #[doc = "MAC decodes received Pause frames and disables its transmitted for a specified time"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RFCE::Enabled)
    }
}
#[doc = "Unicast pause frame detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPFD {
    #[doc = "0: MAC detects only a Pause frame with the multicast address specified in the 802.3x standard"]
    Disabled = 0,
    #[doc = "1: MAC additionally detects Pause frames with the station's unicast address"]
    Enabled = 1,
}
impl From<UPFD> for bool {
    #[inline(always)]
    fn from(variant: UPFD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPFD` reader - Unicast pause frame detect"]
pub type UPFD_R = crate::BitReader<UPFD>;
impl UPFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UPFD {
        match self.bits {
            false => UPFD::Disabled,
            true => UPFD::Enabled,
        }
    }
    #[doc = "MAC detects only a Pause frame with the multicast address specified in the 802.3x standard"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPFD::Disabled
    }
    #[doc = "MAC additionally detects Pause frames with the station's unicast address"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPFD::Enabled
    }
}
#[doc = "Field `UPFD` writer - Unicast pause frame detect"]
pub type UPFD_W<'a, REG> = crate::BitWriter<'a, REG, UPFD>;
impl<'a, REG> UPFD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MAC detects only a Pause frame with the multicast address specified in the 802.3x standard"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPFD::Disabled)
    }
    #[doc = "MAC additionally detects Pause frames with the station's unicast address"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPFD::Enabled)
    }
}
#[doc = "Pause low threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLT {
    #[doc = "0: Pause time minus 4 slot times"]
    Plt4 = 0,
    #[doc = "1: Pause time minus 28 slot times"]
    Plt28 = 1,
    #[doc = "2: Pause time minus 144 slot times"]
    Plt144 = 2,
    #[doc = "3: Pause time minus 256 slot times"]
    Plt256 = 3,
}
impl From<PLT> for u8 {
    #[inline(always)]
    fn from(variant: PLT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLT {
    type Ux = u8;
}
#[doc = "Field `PLT` reader - Pause low threshold"]
pub type PLT_R = crate::FieldReader<PLT>;
impl PLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLT {
        match self.bits {
            0 => PLT::Plt4,
            1 => PLT::Plt28,
            2 => PLT::Plt144,
            3 => PLT::Plt256,
            _ => unreachable!(),
        }
    }
    #[doc = "Pause time minus 4 slot times"]
    #[inline(always)]
    pub fn is_plt4(&self) -> bool {
        *self == PLT::Plt4
    }
    #[doc = "Pause time minus 28 slot times"]
    #[inline(always)]
    pub fn is_plt28(&self) -> bool {
        *self == PLT::Plt28
    }
    #[doc = "Pause time minus 144 slot times"]
    #[inline(always)]
    pub fn is_plt144(&self) -> bool {
        *self == PLT::Plt144
    }
    #[doc = "Pause time minus 256 slot times"]
    #[inline(always)]
    pub fn is_plt256(&self) -> bool {
        *self == PLT::Plt256
    }
}
#[doc = "Field `PLT` writer - Pause low threshold"]
pub type PLT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PLT>;
impl<'a, REG> PLT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pause time minus 4 slot times"]
    #[inline(always)]
    pub fn plt4(self) -> &'a mut crate::W<REG> {
        self.variant(PLT::Plt4)
    }
    #[doc = "Pause time minus 28 slot times"]
    #[inline(always)]
    pub fn plt28(self) -> &'a mut crate::W<REG> {
        self.variant(PLT::Plt28)
    }
    #[doc = "Pause time minus 144 slot times"]
    #[inline(always)]
    pub fn plt144(self) -> &'a mut crate::W<REG> {
        self.variant(PLT::Plt144)
    }
    #[doc = "Pause time minus 256 slot times"]
    #[inline(always)]
    pub fn plt256(self) -> &'a mut crate::W<REG> {
        self.variant(PLT::Plt256)
    }
}
#[doc = "Zero-quanta pause disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZQPD {
    #[doc = "0: Normal operation with automatic zero-quanta pause control frame generation"]
    Enabled = 0,
    #[doc = "1: Automatic generation of zero-quanta pause control frames is disabled"]
    Disabled = 1,
}
impl From<ZQPD> for bool {
    #[inline(always)]
    fn from(variant: ZQPD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ZQPD` reader - Zero-quanta pause disable"]
pub type ZQPD_R = crate::BitReader<ZQPD>;
impl ZQPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ZQPD {
        match self.bits {
            false => ZQPD::Enabled,
            true => ZQPD::Disabled,
        }
    }
    #[doc = "Normal operation with automatic zero-quanta pause control frame generation"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ZQPD::Enabled
    }
    #[doc = "Automatic generation of zero-quanta pause control frames is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ZQPD::Disabled
    }
}
#[doc = "Field `ZQPD` writer - Zero-quanta pause disable"]
pub type ZQPD_W<'a, REG> = crate::BitWriter<'a, REG, ZQPD>;
impl<'a, REG> ZQPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation with automatic zero-quanta pause control frame generation"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ZQPD::Enabled)
    }
    #[doc = "Automatic generation of zero-quanta pause control frames is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ZQPD::Disabled)
    }
}
#[doc = "Field `PT` reader - Pause time"]
pub type PT_R = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - Pause time"]
pub type PT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Flow control busy/back pressure activate"]
    #[inline(always)]
    pub fn fcb(&self) -> FCB_R {
        FCB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit flow control enable"]
    #[inline(always)]
    pub fn tfce(&self) -> TFCE_R {
        TFCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive flow control enable"]
    #[inline(always)]
    pub fn rfce(&self) -> RFCE_R {
        RFCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Unicast pause frame detect"]
    #[inline(always)]
    pub fn upfd(&self) -> UPFD_R {
        UPFD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Pause low threshold"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Zero-quanta pause disable"]
    #[inline(always)]
    pub fn zqpd(&self) -> ZQPD_R {
        ZQPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pause time"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow control busy/back pressure activate"]
    #[inline(always)]
    #[must_use]
    pub fn fcb(&mut self) -> FCB_W<MACFCRrs> {
        FCB_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit flow control enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfce(&mut self) -> TFCE_W<MACFCRrs> {
        TFCE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Receive flow control enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfce(&mut self) -> RFCE_W<MACFCRrs> {
        RFCE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Unicast pause frame detect"]
    #[inline(always)]
    #[must_use]
    pub fn upfd(&mut self) -> UPFD_W<MACFCRrs> {
        UPFD_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Pause low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn plt(&mut self) -> PLT_W<MACFCRrs> {
        PLT_W::new(self, 4)
    }
    #[doc = "Bit 7 - Zero-quanta pause disable"]
    #[inline(always)]
    #[must_use]
    pub fn zqpd(&mut self) -> ZQPD_W<MACFCRrs> {
        ZQPD_W::new(self, 7)
    }
    #[doc = "Bits 16:31 - Pause time"]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<MACFCRrs> {
        PT_W::new(self, 16)
    }
}
#[doc = "Ethernet MAC flow control register (ETH_MACFCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macfcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macfcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACFCRrs;
impl crate::RegisterSpec for MACFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macfcr::R`](R) reader structure"]
impl crate::Readable for MACFCRrs {}
#[doc = "`write(|w| ..)` method takes [`macfcr::W`](W) writer structure"]
impl crate::Writable for MACFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACFCR to value 0"]
impl crate::Resettable for MACFCRrs {
    const RESET_VALUE: u32 = 0;
}
