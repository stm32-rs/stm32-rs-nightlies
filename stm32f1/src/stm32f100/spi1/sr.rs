#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Receive buffer not empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNE {
    #[doc = "0: Rx buffer empty"]
    Empty = 0,
    #[doc = "1: Rx buffer not empty"]
    NotEmpty = 1,
}
impl From<RXNE> for bool {
    #[inline(always)]
    fn from(variant: RXNE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNE` reader - Receive buffer not empty"]
pub type RXNE_R = crate::BitReader<RXNE>;
impl RXNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXNE {
        match self.bits {
            false => RXNE::Empty,
            true => RXNE::NotEmpty,
        }
    }
    #[doc = "Rx buffer empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXNE::Empty
    }
    #[doc = "Rx buffer not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXNE::NotEmpty
    }
}
#[doc = "Transmit buffer empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXE {
    #[doc = "0: Tx buffer not empty"]
    NotEmpty = 0,
    #[doc = "1: Tx buffer empty"]
    Empty = 1,
}
impl From<TXE> for bool {
    #[inline(always)]
    fn from(variant: TXE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXE` reader - Transmit buffer empty"]
pub type TXE_R = crate::BitReader<TXE>;
impl TXE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXE {
        match self.bits {
            false => TXE::NotEmpty,
            true => TXE::Empty,
        }
    }
    #[doc = "Tx buffer not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXE::NotEmpty
    }
    #[doc = "Tx buffer empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXE::Empty
    }
}
#[doc = "CRC error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCERRR {
    #[doc = "0: CRC value received matches the SPIx_RXCRCR value"]
    Match = 0,
    #[doc = "1: CRC value received does not match the SPIx_RXCRCR value"]
    NoMatch = 1,
}
impl From<CRCERRR> for bool {
    #[inline(always)]
    fn from(variant: CRCERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERR` reader - CRC error flag"]
pub type CRCERR_R = crate::BitReader<CRCERRR>;
impl CRCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCERRR {
        match self.bits {
            false => CRCERRR::Match,
            true => CRCERRR::NoMatch,
        }
    }
    #[doc = "CRC value received matches the SPIx_RXCRCR value"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == CRCERRR::Match
    }
    #[doc = "CRC value received does not match the SPIx_RXCRCR value"]
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == CRCERRR::NoMatch
    }
}
#[doc = "CRC error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCERRW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<CRCERRW> for bool {
    #[inline(always)]
    fn from(variant: CRCERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERR` writer - CRC error flag"]
pub type CRCERR_W<'a, REG> = crate::BitWriter0C<'a, REG, CRCERRW>;
impl<'a, REG> CRCERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CRCERRW::Clear)
    }
}
#[doc = "Mode fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODFR {
    #[doc = "0: No mode fault occurred"]
    NoFault = 0,
    #[doc = "1: Mode fault occurred"]
    Fault = 1,
}
impl From<MODFR> for bool {
    #[inline(always)]
    fn from(variant: MODFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODF` reader - Mode fault"]
pub type MODF_R = crate::BitReader<MODFR>;
impl MODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODFR {
        match self.bits {
            false => MODFR::NoFault,
            true => MODFR::Fault,
        }
    }
    #[doc = "No mode fault occurred"]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == MODFR::NoFault
    }
    #[doc = "Mode fault occurred"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == MODFR::Fault
    }
}
#[doc = "Overrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRR {
    #[doc = "0: No overrun occurred"]
    NoOverrun = 0,
    #[doc = "1: Overrun occurred"]
    Overrun = 1,
}
impl From<OVRR> for bool {
    #[inline(always)]
    fn from(variant: OVRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` reader - Overrun flag"]
pub type OVR_R = crate::BitReader<OVRR>;
impl OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVRR {
        match self.bits {
            false => OVRR::NoOverrun,
            true => OVRR::Overrun,
        }
    }
    #[doc = "No overrun occurred"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVRR::NoOverrun
    }
    #[doc = "Overrun occurred"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVRR::Overrun
    }
}
#[doc = "Busy flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYR {
    #[doc = "0: SPI not busy"]
    NotBusy = 0,
    #[doc = "1: SPI busy"]
    Busy = 1,
}
impl From<BSYR> for bool {
    #[inline(always)]
    fn from(variant: BSYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSY` reader - Busy flag"]
pub type BSY_R = crate::BitReader<BSYR>;
impl BSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BSYR {
        match self.bits {
            false => BSYR::NotBusy,
            true => BSYR::Busy,
        }
    }
    #[doc = "SPI not busy"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BSYR::NotBusy
    }
    #[doc = "SPI busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BSYR::Busy
    }
}
impl R {
    #[doc = "Bit 0 - Receive buffer not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC error flag"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode fault"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun flag"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Busy flag"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - CRC error flag"]
    #[inline(always)]
    #[must_use]
    pub fn crcerr(&mut self) -> CRCERR_W<SRrs> {
        CRCERR_W::new(self, 4)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x10;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0x02"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x02;
}
