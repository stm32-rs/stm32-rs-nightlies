///Register `MACFCR` reader
pub type R = crate::R<MACFCRrs>;
///Register `MACFCR` writer
pub type W = crate::W<MACFCRrs>;
/**Flow control busy/back pressure activate

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCB {
    ///0: In half duplex only, deasserts back pressure
    DisableBackPressure = 0,
    ///1: In full duplex, initiate a Pause control frame. In half duplex, assert back pressure
    PauseOrBackPressure = 1,
}
impl From<FCB> for bool {
    #[inline(always)]
    fn from(variant: FCB) -> Self {
        variant as u8 != 0
    }
}
///Field `FCB` reader - Flow control busy/back pressure activate
pub type FCB_R = crate::BitReader<FCB>;
impl FCB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FCB {
        match self.bits {
            false => FCB::DisableBackPressure,
            true => FCB::PauseOrBackPressure,
        }
    }
    ///In half duplex only, deasserts back pressure
    #[inline(always)]
    pub fn is_disable_back_pressure(&self) -> bool {
        *self == FCB::DisableBackPressure
    }
    ///In full duplex, initiate a Pause control frame. In half duplex, assert back pressure
    #[inline(always)]
    pub fn is_pause_or_back_pressure(&self) -> bool {
        *self == FCB::PauseOrBackPressure
    }
}
///Field `FCB` writer - Flow control busy/back pressure activate
pub type FCB_W<'a, REG> = crate::BitWriter<'a, REG, FCB>;
impl<'a, REG> FCB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///In half duplex only, deasserts back pressure
    #[inline(always)]
    pub fn disable_back_pressure(self) -> &'a mut crate::W<REG> {
        self.variant(FCB::DisableBackPressure)
    }
    ///In full duplex, initiate a Pause control frame. In half duplex, assert back pressure
    #[inline(always)]
    pub fn pause_or_back_pressure(self) -> &'a mut crate::W<REG> {
        self.variant(FCB::PauseOrBackPressure)
    }
}
/**Transmit flow control enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFCE {
    ///0: In full duplex, flow control is disabled. In half duplex, back pressure is disabled
    Disabled = 0,
    ///1: In full duplex, flow control is enabled. In half duplex, back pressure is enabled
    Enabled = 1,
}
impl From<TFCE> for bool {
    #[inline(always)]
    fn from(variant: TFCE) -> Self {
        variant as u8 != 0
    }
}
///Field `TFCE` reader - Transmit flow control enable
pub type TFCE_R = crate::BitReader<TFCE>;
impl TFCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TFCE {
        match self.bits {
            false => TFCE::Disabled,
            true => TFCE::Enabled,
        }
    }
    ///In full duplex, flow control is disabled. In half duplex, back pressure is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TFCE::Disabled
    }
    ///In full duplex, flow control is enabled. In half duplex, back pressure is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TFCE::Enabled
    }
}
///Field `TFCE` writer - Transmit flow control enable
pub type TFCE_W<'a, REG> = crate::BitWriter<'a, REG, TFCE>;
impl<'a, REG> TFCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///In full duplex, flow control is disabled. In half duplex, back pressure is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TFCE::Disabled)
    }
    ///In full duplex, flow control is enabled. In half duplex, back pressure is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TFCE::Enabled)
    }
}
/**Receive flow control enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFCE {
    ///0: Pause frames are not decoded
    Disabled = 0,
    ///1: MAC decodes received Pause frames and disables its transmitted for a specified time
    Enabled = 1,
}
impl From<RFCE> for bool {
    #[inline(always)]
    fn from(variant: RFCE) -> Self {
        variant as u8 != 0
    }
}
///Field `RFCE` reader - Receive flow control enable
pub type RFCE_R = crate::BitReader<RFCE>;
impl RFCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFCE {
        match self.bits {
            false => RFCE::Disabled,
            true => RFCE::Enabled,
        }
    }
    ///Pause frames are not decoded
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RFCE::Disabled
    }
    ///MAC decodes received Pause frames and disables its transmitted for a specified time
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RFCE::Enabled
    }
}
///Field `RFCE` writer - Receive flow control enable
pub type RFCE_W<'a, REG> = crate::BitWriter<'a, REG, RFCE>;
impl<'a, REG> RFCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pause frames are not decoded
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RFCE::Disabled)
    }
    ///MAC decodes received Pause frames and disables its transmitted for a specified time
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RFCE::Enabled)
    }
}
/**Unicast pause frame detect

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPFD {
    ///0: MAC detects only a Pause frame with the multicast address specified in the 802.3x standard
    Disabled = 0,
    ///1: MAC additionally detects Pause frames with the station's unicast address
    Enabled = 1,
}
impl From<UPFD> for bool {
    #[inline(always)]
    fn from(variant: UPFD) -> Self {
        variant as u8 != 0
    }
}
///Field `UPFD` reader - Unicast pause frame detect
pub type UPFD_R = crate::BitReader<UPFD>;
impl UPFD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UPFD {
        match self.bits {
            false => UPFD::Disabled,
            true => UPFD::Enabled,
        }
    }
    ///MAC detects only a Pause frame with the multicast address specified in the 802.3x standard
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPFD::Disabled
    }
    ///MAC additionally detects Pause frames with the station's unicast address
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPFD::Enabled
    }
}
///Field `UPFD` writer - Unicast pause frame detect
pub type UPFD_W<'a, REG> = crate::BitWriter<'a, REG, UPFD>;
impl<'a, REG> UPFD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MAC detects only a Pause frame with the multicast address specified in the 802.3x standard
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPFD::Disabled)
    }
    ///MAC additionally detects Pause frames with the station's unicast address
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPFD::Enabled)
    }
}
/**Pause low threshold

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLT {
    ///0: Pause time minus 4 slot times
    Plt4 = 0,
    ///1: Pause time minus 28 slot times
    Plt28 = 1,
    ///2: Pause time minus 144 slot times
    Plt144 = 2,
    ///3: Pause time minus 256 slot times
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
impl crate::IsEnum for PLT {}
///Field `PLT` reader - Pause low threshold
pub type PLT_R = crate::FieldReader<PLT>;
impl PLT_R {
    ///Get enumerated values variant
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
    ///Pause time minus 4 slot times
    #[inline(always)]
    pub fn is_plt4(&self) -> bool {
        *self == PLT::Plt4
    }
    ///Pause time minus 28 slot times
    #[inline(always)]
    pub fn is_plt28(&self) -> bool {
        *self == PLT::Plt28
    }
    ///Pause time minus 144 slot times
    #[inline(always)]
    pub fn is_plt144(&self) -> bool {
        *self == PLT::Plt144
    }
    ///Pause time minus 256 slot times
    #[inline(always)]
    pub fn is_plt256(&self) -> bool {
        *self == PLT::Plt256
    }
}
///Field `PLT` writer - Pause low threshold
pub type PLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLT, crate::Safe>;
impl<'a, REG> PLT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Pause time minus 4 slot times
    #[inline(always)]
    pub fn plt4(self) -> &'a mut crate::W<REG> {
        self.variant(PLT::Plt4)
    }
    ///Pause time minus 28 slot times
    #[inline(always)]
    pub fn plt28(self) -> &'a mut crate::W<REG> {
        self.variant(PLT::Plt28)
    }
    ///Pause time minus 144 slot times
    #[inline(always)]
    pub fn plt144(self) -> &'a mut crate::W<REG> {
        self.variant(PLT::Plt144)
    }
    ///Pause time minus 256 slot times
    #[inline(always)]
    pub fn plt256(self) -> &'a mut crate::W<REG> {
        self.variant(PLT::Plt256)
    }
}
/**Zero-quanta pause disable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZQPD {
    ///0: Normal operation with automatic zero-quanta pause control frame generation
    Enabled = 0,
    ///1: Automatic generation of zero-quanta pause control frames is disabled
    Disabled = 1,
}
impl From<ZQPD> for bool {
    #[inline(always)]
    fn from(variant: ZQPD) -> Self {
        variant as u8 != 0
    }
}
///Field `ZQPD` reader - Zero-quanta pause disable
pub type ZQPD_R = crate::BitReader<ZQPD>;
impl ZQPD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ZQPD {
        match self.bits {
            false => ZQPD::Enabled,
            true => ZQPD::Disabled,
        }
    }
    ///Normal operation with automatic zero-quanta pause control frame generation
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ZQPD::Enabled
    }
    ///Automatic generation of zero-quanta pause control frames is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ZQPD::Disabled
    }
}
///Field `ZQPD` writer - Zero-quanta pause disable
pub type ZQPD_W<'a, REG> = crate::BitWriter<'a, REG, ZQPD>;
impl<'a, REG> ZQPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal operation with automatic zero-quanta pause control frame generation
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ZQPD::Enabled)
    }
    ///Automatic generation of zero-quanta pause control frames is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ZQPD::Disabled)
    }
}
///Field `PT` reader - Pause time
pub type PT_R = crate::FieldReader<u16>;
///Field `PT` writer - Pause time
pub type PT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bit 0 - Flow control busy/back pressure activate
    #[inline(always)]
    pub fn fcb(&self) -> FCB_R {
        FCB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit flow control enable
    #[inline(always)]
    pub fn tfce(&self) -> TFCE_R {
        TFCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receive flow control enable
    #[inline(always)]
    pub fn rfce(&self) -> RFCE_R {
        RFCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Unicast pause frame detect
    #[inline(always)]
    pub fn upfd(&self) -> UPFD_R {
        UPFD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Pause low threshold
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - Zero-quanta pause disable
    #[inline(always)]
    pub fn zqpd(&self) -> ZQPD_R {
        ZQPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 16:31 - Pause time
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACFCR")
            .field("fcb", &self.fcb())
            .field("tfce", &self.tfce())
            .field("rfce", &self.rfce())
            .field("upfd", &self.upfd())
            .field("plt", &self.plt())
            .field("zqpd", &self.zqpd())
            .field("pt", &self.pt())
            .finish()
    }
}
impl W {
    ///Bit 0 - Flow control busy/back pressure activate
    #[inline(always)]
    pub fn fcb(&mut self) -> FCB_W<'_, MACFCRrs> {
        FCB_W::new(self, 0)
    }
    ///Bit 1 - Transmit flow control enable
    #[inline(always)]
    pub fn tfce(&mut self) -> TFCE_W<'_, MACFCRrs> {
        TFCE_W::new(self, 1)
    }
    ///Bit 2 - Receive flow control enable
    #[inline(always)]
    pub fn rfce(&mut self) -> RFCE_W<'_, MACFCRrs> {
        RFCE_W::new(self, 2)
    }
    ///Bit 3 - Unicast pause frame detect
    #[inline(always)]
    pub fn upfd(&mut self) -> UPFD_W<'_, MACFCRrs> {
        UPFD_W::new(self, 3)
    }
    ///Bits 4:5 - Pause low threshold
    #[inline(always)]
    pub fn plt(&mut self) -> PLT_W<'_, MACFCRrs> {
        PLT_W::new(self, 4)
    }
    ///Bit 7 - Zero-quanta pause disable
    #[inline(always)]
    pub fn zqpd(&mut self) -> ZQPD_W<'_, MACFCRrs> {
        ZQPD_W::new(self, 7)
    }
    ///Bits 16:31 - Pause time
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W<'_, MACFCRrs> {
        PT_W::new(self, 16)
    }
}
/**Ethernet MAC flow control register

You can [`read`](crate::Reg::read) this register and get [`macfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F429.html#Ethernet_MAC:MACFCR)*/
pub struct MACFCRrs;
impl crate::RegisterSpec for MACFCRrs {
    type Ux = u32;
}
///`read()` method returns [`macfcr::R`](R) reader structure
impl crate::Readable for MACFCRrs {}
///`write(|w| ..)` method takes [`macfcr::W`](W) writer structure
impl crate::Writable for MACFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACFCR to value 0
impl crate::Resettable for MACFCRrs {}
