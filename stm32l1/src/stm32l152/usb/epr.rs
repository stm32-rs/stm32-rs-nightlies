///Register `EP%sR` reader
pub type R = crate::R<EPRrs>;
///Register `EP%sR` writer
pub type W = crate::W<EPRrs>;
///Field `EA` reader - Endpoint address
pub type EA_R = crate::FieldReader;
///Field `EA` writer - Endpoint address
pub type EA_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
/**Status bits, for transmission transfers

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STAT_TXR {
    ///0: all transmission requests addressed to this endpoint are ignored
    Disabled = 0,
    ///1: the endpoint is stalled and all transmission requests result in a STALL handshake
    Stall = 1,
    ///2: the endpoint is naked and all transmission requests result in a NAK handshake
    Nak = 2,
    ///3: this endpoint is enabled for transmission
    Valid = 3,
}
impl From<STAT_TXR> for u8 {
    #[inline(always)]
    fn from(variant: STAT_TXR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STAT_TXR {
    type Ux = u8;
}
impl crate::IsEnum for STAT_TXR {}
///Field `STAT_TX` reader - Status bits, for transmission transfers
pub type STAT_TX_R = crate::FieldReader<STAT_TXR>;
impl STAT_TX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STAT_TXR {
        match self.bits {
            0 => STAT_TXR::Disabled,
            1 => STAT_TXR::Stall,
            2 => STAT_TXR::Nak,
            3 => STAT_TXR::Valid,
            _ => unreachable!(),
        }
    }
    ///all transmission requests addressed to this endpoint are ignored
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STAT_TXR::Disabled
    }
    ///the endpoint is stalled and all transmission requests result in a STALL handshake
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == STAT_TXR::Stall
    }
    ///the endpoint is naked and all transmission requests result in a NAK handshake
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == STAT_TXR::Nak
    }
    ///this endpoint is enabled for transmission
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == STAT_TXR::Valid
    }
}
///Field `STAT_TX` writer - Status bits, for transmission transfers
pub type STAT_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STAT_TXR, crate::Safe>;
impl<'a, REG> STAT_TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///all transmission requests addressed to this endpoint are ignored
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_TXR::Disabled)
    }
    ///the endpoint is stalled and all transmission requests result in a STALL handshake
    #[inline(always)]
    pub fn stall(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_TXR::Stall)
    }
    ///the endpoint is naked and all transmission requests result in a NAK handshake
    #[inline(always)]
    pub fn nak(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_TXR::Nak)
    }
    ///this endpoint is enabled for transmission
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_TXR::Valid)
    }
}
///Field `DTOG_TX` reader - Data Toggle, for transmission transfers
pub type DTOG_TX_R = crate::BitReader;
///Field `DTOG_TX` writer - Data Toggle, for transmission transfers
pub type DTOG_TX_W<'a, REG> = crate::BitWriter1T<'a, REG>;
///Field `CTR_TX` reader - Correct Transfer for transmission
pub type CTR_TX_R = crate::BitReader;
///Field `CTR_TX` writer - Correct Transfer for transmission
pub type CTR_TX_W<'a, REG> = crate::BitWriter0C<'a, REG>;
///Field `EP_KIND` reader - Endpoint kind
pub type EP_KIND_R = crate::BitReader;
///Field `EP_KIND` writer - Endpoint kind
pub type EP_KIND_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Endpoint type

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EP_TYPE {
    ///0: Bulk endpoint
    Bulk = 0,
    ///1: Control endpoint
    Control = 1,
    ///2: Iso endpoint
    Iso = 2,
    ///3: Interrupt endpoint
    Interrupt = 3,
}
impl From<EP_TYPE> for u8 {
    #[inline(always)]
    fn from(variant: EP_TYPE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EP_TYPE {
    type Ux = u8;
}
impl crate::IsEnum for EP_TYPE {}
///Field `EP_TYPE` reader - Endpoint type
pub type EP_TYPE_R = crate::FieldReader<EP_TYPE>;
impl EP_TYPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EP_TYPE {
        match self.bits {
            0 => EP_TYPE::Bulk,
            1 => EP_TYPE::Control,
            2 => EP_TYPE::Iso,
            3 => EP_TYPE::Interrupt,
            _ => unreachable!(),
        }
    }
    ///Bulk endpoint
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == EP_TYPE::Bulk
    }
    ///Control endpoint
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == EP_TYPE::Control
    }
    ///Iso endpoint
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == EP_TYPE::Iso
    }
    ///Interrupt endpoint
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EP_TYPE::Interrupt
    }
}
///Field `EP_TYPE` writer - Endpoint type
pub type EP_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EP_TYPE, crate::Safe>;
impl<'a, REG> EP_TYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Bulk endpoint
    #[inline(always)]
    pub fn bulk(self) -> &'a mut crate::W<REG> {
        self.variant(EP_TYPE::Bulk)
    }
    ///Control endpoint
    #[inline(always)]
    pub fn control(self) -> &'a mut crate::W<REG> {
        self.variant(EP_TYPE::Control)
    }
    ///Iso endpoint
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(EP_TYPE::Iso)
    }
    ///Interrupt endpoint
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(EP_TYPE::Interrupt)
    }
}
///Field `SETUP` reader - Setup transaction completed
pub type SETUP_R = crate::BitReader;
///Field `SETUP` writer - Setup transaction completed
pub type SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Status bits, for reception transfers

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STAT_RXR {
    ///0: all reception requests addressed to this endpoint are ignored
    Disabled = 0,
    ///1: the endpoint is stalled and all reception requests result in a STALL handshake
    Stall = 1,
    ///2: the endpoint is naked and all reception requests result in a NAK handshake
    Nak = 2,
    ///3: this endpoint is enabled for reception
    Valid = 3,
}
impl From<STAT_RXR> for u8 {
    #[inline(always)]
    fn from(variant: STAT_RXR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STAT_RXR {
    type Ux = u8;
}
impl crate::IsEnum for STAT_RXR {}
///Field `STAT_RX` reader - Status bits, for reception transfers
pub type STAT_RX_R = crate::FieldReader<STAT_RXR>;
impl STAT_RX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STAT_RXR {
        match self.bits {
            0 => STAT_RXR::Disabled,
            1 => STAT_RXR::Stall,
            2 => STAT_RXR::Nak,
            3 => STAT_RXR::Valid,
            _ => unreachable!(),
        }
    }
    ///all reception requests addressed to this endpoint are ignored
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STAT_RXR::Disabled
    }
    ///the endpoint is stalled and all reception requests result in a STALL handshake
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == STAT_RXR::Stall
    }
    ///the endpoint is naked and all reception requests result in a NAK handshake
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == STAT_RXR::Nak
    }
    ///this endpoint is enabled for reception
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == STAT_RXR::Valid
    }
}
///Field `STAT_RX` writer - Status bits, for reception transfers
pub type STAT_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STAT_RXR, crate::Safe>;
impl<'a, REG> STAT_RX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///all reception requests addressed to this endpoint are ignored
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_RXR::Disabled)
    }
    ///the endpoint is stalled and all reception requests result in a STALL handshake
    #[inline(always)]
    pub fn stall(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_RXR::Stall)
    }
    ///the endpoint is naked and all reception requests result in a NAK handshake
    #[inline(always)]
    pub fn nak(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_RXR::Nak)
    }
    ///this endpoint is enabled for reception
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_RXR::Valid)
    }
}
///Field `DTOG_RX` reader - Data Toggle, for reception transfers
pub type DTOG_RX_R = crate::BitReader;
///Field `DTOG_RX` writer - Data Toggle, for reception transfers
pub type DTOG_RX_W<'a, REG> = crate::BitWriter1T<'a, REG>;
///Field `CTR_RX` reader - Correct transfer for reception
pub type CTR_RX_R = crate::BitReader;
///Field `CTR_RX` writer - Correct transfer for reception
pub type CTR_RX_W<'a, REG> = crate::BitWriter0C<'a, REG>;
impl R {
    ///Bits 0:3 - Endpoint address
    #[inline(always)]
    pub fn ea(&self) -> EA_R {
        EA_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - Status bits, for transmission transfers
    #[inline(always)]
    pub fn stat_tx(&self) -> STAT_TX_R {
        STAT_TX_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Data Toggle, for transmission transfers
    #[inline(always)]
    pub fn dtog_tx(&self) -> DTOG_TX_R {
        DTOG_TX_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Correct Transfer for transmission
    #[inline(always)]
    pub fn ctr_tx(&self) -> CTR_TX_R {
        CTR_TX_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Endpoint kind
    #[inline(always)]
    pub fn ep_kind(&self) -> EP_KIND_R {
        EP_KIND_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - Endpoint type
    #[inline(always)]
    pub fn ep_type(&self) -> EP_TYPE_R {
        EP_TYPE_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 11 - Setup transaction completed
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Status bits, for reception transfers
    #[inline(always)]
    pub fn stat_rx(&self) -> STAT_RX_R {
        STAT_RX_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - Data Toggle, for reception transfers
    #[inline(always)]
    pub fn dtog_rx(&self) -> DTOG_RX_R {
        DTOG_RX_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Correct transfer for reception
    #[inline(always)]
    pub fn ctr_rx(&self) -> CTR_RX_R {
        CTR_RX_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPR")
            .field("ea", &self.ea())
            .field("stat_tx", &self.stat_tx())
            .field("dtog_tx", &self.dtog_tx())
            .field("ctr_tx", &self.ctr_tx())
            .field("ep_kind", &self.ep_kind())
            .field("ep_type", &self.ep_type())
            .field("setup", &self.setup())
            .field("stat_rx", &self.stat_rx())
            .field("dtog_rx", &self.dtog_rx())
            .field("ctr_rx", &self.ctr_rx())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Endpoint address
    #[inline(always)]
    pub fn ea(&mut self) -> EA_W<'_, EPRrs> {
        EA_W::new(self, 0)
    }
    ///Bits 4:5 - Status bits, for transmission transfers
    #[inline(always)]
    pub fn stat_tx(&mut self) -> STAT_TX_W<'_, EPRrs> {
        STAT_TX_W::new(self, 4)
    }
    ///Bit 6 - Data Toggle, for transmission transfers
    #[inline(always)]
    pub fn dtog_tx(&mut self) -> DTOG_TX_W<'_, EPRrs> {
        DTOG_TX_W::new(self, 6)
    }
    ///Bit 7 - Correct Transfer for transmission
    #[inline(always)]
    pub fn ctr_tx(&mut self) -> CTR_TX_W<'_, EPRrs> {
        CTR_TX_W::new(self, 7)
    }
    ///Bit 8 - Endpoint kind
    #[inline(always)]
    pub fn ep_kind(&mut self) -> EP_KIND_W<'_, EPRrs> {
        EP_KIND_W::new(self, 8)
    }
    ///Bits 9:10 - Endpoint type
    #[inline(always)]
    pub fn ep_type(&mut self) -> EP_TYPE_W<'_, EPRrs> {
        EP_TYPE_W::new(self, 9)
    }
    ///Bit 11 - Setup transaction completed
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W<'_, EPRrs> {
        SETUP_W::new(self, 11)
    }
    ///Bits 12:13 - Status bits, for reception transfers
    #[inline(always)]
    pub fn stat_rx(&mut self) -> STAT_RX_W<'_, EPRrs> {
        STAT_RX_W::new(self, 12)
    }
    ///Bit 14 - Data Toggle, for reception transfers
    #[inline(always)]
    pub fn dtog_rx(&mut self) -> DTOG_RX_W<'_, EPRrs> {
        DTOG_RX_W::new(self, 14)
    }
    ///Bit 15 - Correct transfer for reception
    #[inline(always)]
    pub fn ctr_rx(&mut self) -> CTR_RX_W<'_, EPRrs> {
        CTR_RX_W::new(self, 15)
    }
}
/**endpoint %s register

You can [`read`](crate::Reg::read) this register and get [`epr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#USB:EP[0]R)*/
pub struct EPRrs;
impl crate::RegisterSpec for EPRrs {
    type Ux = u32;
}
///`read()` method returns [`epr::R`](R) reader structure
impl crate::Readable for EPRrs {}
///`write(|w| ..)` method takes [`epr::W`](W) writer structure
impl crate::Writable for EPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x8080;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x7070;
}
///`reset()` method sets EP%sR to value 0
impl crate::Resettable for EPRrs {}
