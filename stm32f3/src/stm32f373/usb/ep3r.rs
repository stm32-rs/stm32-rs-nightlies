#[doc = "Register `EP3R` reader"]
pub type R = crate::R<EP3Rrs>;
#[doc = "Register `EP3R` writer"]
pub type W = crate::W<EP3Rrs>;
#[doc = "Field `EA` reader - Endpoint address"]
pub type EA_R = crate::FieldReader;
#[doc = "Field `EA` writer - Endpoint address"]
pub type EA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Status bits, for transmission transfers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STAT_TXR {
    #[doc = "0: all transmission requests addressed to this endpoint are ignored"]
    Disabled = 0,
    #[doc = "1: the endpoint is stalled and all transmission requests result in a STALL handshake"]
    Stall = 1,
    #[doc = "2: the endpoint is naked and all transmission requests result in a NAK handshake"]
    Nak = 2,
    #[doc = "3: this endpoint is enabled for transmission"]
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
#[doc = "Field `STAT_TX` reader - Status bits, for transmission transfers"]
pub type STAT_TX_R = crate::FieldReader<STAT_TXR>;
impl STAT_TX_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "all transmission requests addressed to this endpoint are ignored"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STAT_TXR::Disabled
    }
    #[doc = "the endpoint is stalled and all transmission requests result in a STALL handshake"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == STAT_TXR::Stall
    }
    #[doc = "the endpoint is naked and all transmission requests result in a NAK handshake"]
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == STAT_TXR::Nak
    }
    #[doc = "this endpoint is enabled for transmission"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == STAT_TXR::Valid
    }
}
#[doc = "Field `STAT_TX` writer - Status bits, for transmission transfers"]
pub type STAT_TX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, STAT_TXR>;
impl<'a, REG> STAT_TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "all transmission requests addressed to this endpoint are ignored"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_TXR::Disabled)
    }
    #[doc = "the endpoint is stalled and all transmission requests result in a STALL handshake"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_TXR::Stall)
    }
    #[doc = "the endpoint is naked and all transmission requests result in a NAK handshake"]
    #[inline(always)]
    pub fn nak(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_TXR::Nak)
    }
    #[doc = "this endpoint is enabled for transmission"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_TXR::Valid)
    }
}
#[doc = "Field `DTOG_TX` reader - Data Toggle, for transmission transfers"]
pub type DTOG_TX_R = crate::BitReader;
#[doc = "Field `DTOG_TX` writer - Data Toggle, for transmission transfers"]
pub type DTOG_TX_W<'a, REG> = crate::BitWriter1T<'a, REG>;
#[doc = "Field `CTR_TX` reader - Correct Transfer for transmission"]
pub type CTR_TX_R = crate::BitReader;
#[doc = "Field `CTR_TX` writer - Correct Transfer for transmission"]
pub type CTR_TX_W<'a, REG> = crate::BitWriter0C<'a, REG>;
#[doc = "Field `EP_KIND` reader - Endpoint kind"]
pub type EP_KIND_R = crate::BitReader;
#[doc = "Field `EP_KIND` writer - Endpoint kind"]
pub type EP_KIND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Endpoint type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EP_TYPE {
    #[doc = "0: Bulk endpoint"]
    Bulk = 0,
    #[doc = "1: Control endpoint"]
    Control = 1,
    #[doc = "2: Iso endpoint"]
    Iso = 2,
    #[doc = "3: Interrupt endpoint"]
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
#[doc = "Field `EP_TYPE` reader - Endpoint type"]
pub type EP_TYPE_R = crate::FieldReader<EP_TYPE>;
impl EP_TYPE_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Bulk endpoint"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == EP_TYPE::Bulk
    }
    #[doc = "Control endpoint"]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == EP_TYPE::Control
    }
    #[doc = "Iso endpoint"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == EP_TYPE::Iso
    }
    #[doc = "Interrupt endpoint"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EP_TYPE::Interrupt
    }
}
#[doc = "Field `EP_TYPE` writer - Endpoint type"]
pub type EP_TYPE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EP_TYPE>;
impl<'a, REG> EP_TYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bulk endpoint"]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut crate::W<REG> {
        self.variant(EP_TYPE::Bulk)
    }
    #[doc = "Control endpoint"]
    #[inline(always)]
    pub fn control(self) -> &'a mut crate::W<REG> {
        self.variant(EP_TYPE::Control)
    }
    #[doc = "Iso endpoint"]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(EP_TYPE::Iso)
    }
    #[doc = "Interrupt endpoint"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(EP_TYPE::Interrupt)
    }
}
#[doc = "Field `SETUP` reader - Setup transaction completed"]
pub type SETUP_R = crate::BitReader;
#[doc = "Status bits, for reception transfers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STAT_RXR {
    #[doc = "0: all reception requests addressed to this endpoint are ignored"]
    Disabled = 0,
    #[doc = "1: the endpoint is stalled and all reception requests result in a STALL handshake"]
    Stall = 1,
    #[doc = "2: the endpoint is naked and all reception requests result in a NAK handshake"]
    Nak = 2,
    #[doc = "3: this endpoint is enabled for reception"]
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
#[doc = "Field `STAT_RX` reader - Status bits, for reception transfers"]
pub type STAT_RX_R = crate::FieldReader<STAT_RXR>;
impl STAT_RX_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "all reception requests addressed to this endpoint are ignored"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STAT_RXR::Disabled
    }
    #[doc = "the endpoint is stalled and all reception requests result in a STALL handshake"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == STAT_RXR::Stall
    }
    #[doc = "the endpoint is naked and all reception requests result in a NAK handshake"]
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == STAT_RXR::Nak
    }
    #[doc = "this endpoint is enabled for reception"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == STAT_RXR::Valid
    }
}
#[doc = "Field `STAT_RX` writer - Status bits, for reception transfers"]
pub type STAT_RX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, STAT_RXR>;
impl<'a, REG> STAT_RX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "all reception requests addressed to this endpoint are ignored"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_RXR::Disabled)
    }
    #[doc = "the endpoint is stalled and all reception requests result in a STALL handshake"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_RXR::Stall)
    }
    #[doc = "the endpoint is naked and all reception requests result in a NAK handshake"]
    #[inline(always)]
    pub fn nak(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_RXR::Nak)
    }
    #[doc = "this endpoint is enabled for reception"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(STAT_RXR::Valid)
    }
}
#[doc = "Field `DTOG_RX` reader - Data Toggle, for reception transfers"]
pub type DTOG_RX_R = crate::BitReader;
#[doc = "Field `DTOG_RX` writer - Data Toggle, for reception transfers"]
pub type DTOG_RX_W<'a, REG> = crate::BitWriter1T<'a, REG>;
#[doc = "Field `CTR_RX` reader - Correct transfer for reception"]
pub type CTR_RX_R = crate::BitReader;
#[doc = "Field `CTR_RX` writer - Correct transfer for reception"]
pub type CTR_RX_W<'a, REG> = crate::BitWriter0C<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    pub fn ea(&self) -> EA_R {
        EA_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    pub fn stat_tx(&self) -> STAT_TX_R {
        STAT_TX_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Data Toggle, for transmission transfers"]
    #[inline(always)]
    pub fn dtog_tx(&self) -> DTOG_TX_R {
        DTOG_TX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Correct Transfer for transmission"]
    #[inline(always)]
    pub fn ctr_tx(&self) -> CTR_TX_R {
        CTR_TX_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint kind"]
    #[inline(always)]
    pub fn ep_kind(&self) -> EP_KIND_R {
        EP_KIND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Endpoint type"]
    #[inline(always)]
    pub fn ep_type(&self) -> EP_TYPE_R {
        EP_TYPE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers"]
    #[inline(always)]
    pub fn stat_rx(&self) -> STAT_RX_R {
        STAT_RX_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers"]
    #[inline(always)]
    pub fn dtog_rx(&self) -> DTOG_RX_R {
        DTOG_RX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Correct transfer for reception"]
    #[inline(always)]
    pub fn ctr_rx(&self) -> CTR_RX_R {
        CTR_RX_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    #[must_use]
    pub fn ea(&mut self) -> EA_W<EP3Rrs> {
        EA_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    #[must_use]
    pub fn stat_tx(&mut self) -> STAT_TX_W<EP3Rrs> {
        STAT_TX_W::new(self, 4)
    }
    #[doc = "Bit 6 - Data Toggle, for transmission transfers"]
    #[inline(always)]
    #[must_use]
    pub fn dtog_tx(&mut self) -> DTOG_TX_W<EP3Rrs> {
        DTOG_TX_W::new(self, 6)
    }
    #[doc = "Bit 7 - Correct Transfer for transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ctr_tx(&mut self) -> CTR_TX_W<EP3Rrs> {
        CTR_TX_W::new(self, 7)
    }
    #[doc = "Bit 8 - Endpoint kind"]
    #[inline(always)]
    #[must_use]
    pub fn ep_kind(&mut self) -> EP_KIND_W<EP3Rrs> {
        EP_KIND_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - Endpoint type"]
    #[inline(always)]
    #[must_use]
    pub fn ep_type(&mut self) -> EP_TYPE_W<EP3Rrs> {
        EP_TYPE_W::new(self, 9)
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers"]
    #[inline(always)]
    #[must_use]
    pub fn stat_rx(&mut self) -> STAT_RX_W<EP3Rrs> {
        STAT_RX_W::new(self, 12)
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers"]
    #[inline(always)]
    #[must_use]
    pub fn dtog_rx(&mut self) -> DTOG_RX_W<EP3Rrs> {
        DTOG_RX_W::new(self, 14)
    }
    #[doc = "Bit 15 - Correct transfer for reception"]
    #[inline(always)]
    #[must_use]
    pub fn ctr_rx(&mut self) -> CTR_RX_W<EP3Rrs> {
        CTR_RX_W::new(self, 15)
    }
}
#[doc = "endpoint 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep3r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep3r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP3Rrs;
impl crate::RegisterSpec for EP3Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep3r::R`](R) reader structure"]
impl crate::Readable for EP3Rrs {}
#[doc = "`write(|w| ..)` method takes [`ep3r::W`](W) writer structure"]
impl crate::Writable for EP3Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x8080;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x7070;
}
#[doc = "`reset()` method sets EP3R to value 0"]
impl crate::Resettable for EP3Rrs {
    const RESET_VALUE: u32 = 0;
}
