#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "7-bit Address Detection/4-bit Address Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDM7 {
    #[doc = "0: 4-bit address detection"]
    Bit4 = 0,
    #[doc = "1: 7-bit address detection"]
    Bit7 = 1,
}
impl From<ADDM7> for bool {
    #[inline(always)]
    fn from(variant: ADDM7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection"]
pub type ADDM7_R = crate::BitReader<ADDM7>;
impl ADDM7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDM7 {
        match self.bits {
            false => ADDM7::Bit4,
            true => ADDM7::Bit7,
        }
    }
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn is_bit4(&self) -> bool {
        *self == ADDM7::Bit4
    }
    #[doc = "7-bit address detection"]
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == ADDM7::Bit7
    }
}
#[doc = "Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection"]
pub type ADDM7_W<'a, REG> = crate::BitWriter<'a, REG, ADDM7>;
impl<'a, REG> ADDM7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn bit4(self) -> &'a mut crate::W<REG> {
        self.variant(ADDM7::Bit4)
    }
    #[doc = "7-bit address detection"]
    #[inline(always)]
    pub fn bit7(self) -> &'a mut crate::W<REG> {
        self.variant(ADDM7::Bit7)
    }
}
#[doc = "STOP bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOP {
    #[doc = "0: 1 stop bit"]
    Stop1 = 0,
    #[doc = "2: 2 stop bit"]
    Stop2 = 2,
}
impl From<STOP> for u8 {
    #[inline(always)]
    fn from(variant: STOP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STOP {
    type Ux = u8;
}
#[doc = "Field `STOP` reader - STOP bits"]
pub type STOP_R = crate::FieldReader<STOP>;
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<STOP> {
        match self.bits {
            0 => Some(STOP::Stop1),
            2 => Some(STOP::Stop2),
            _ => None,
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == STOP::Stop1
    }
    #[doc = "2 stop bit"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == STOP::Stop2
    }
}
#[doc = "Field `STOP` writer - STOP bits"]
pub type STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STOP>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop1)
    }
    #[doc = "2 stop bit"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop2)
    }
}
#[doc = "Swap TX/RX pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWAP {
    #[doc = "0: TX/RX pins are used as defined in standard pinout"]
    Standard = 0,
    #[doc = "1: The TX and RX pins functions are swapped"]
    Swapped = 1,
}
impl From<SWAP> for bool {
    #[inline(always)]
    fn from(variant: SWAP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAP` reader - Swap TX/RX pins"]
pub type SWAP_R = crate::BitReader<SWAP>;
impl SWAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWAP {
        match self.bits {
            false => SWAP::Standard,
            true => SWAP::Swapped,
        }
    }
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == SWAP::Standard
    }
    #[doc = "The TX and RX pins functions are swapped"]
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == SWAP::Swapped
    }
}
#[doc = "Field `SWAP` writer - Swap TX/RX pins"]
pub type SWAP_W<'a, REG> = crate::BitWriter<'a, REG, SWAP>;
impl<'a, REG> SWAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(SWAP::Standard)
    }
    #[doc = "The TX and RX pins functions are swapped"]
    #[inline(always)]
    pub fn swapped(self) -> &'a mut crate::W<REG> {
        self.variant(SWAP::Swapped)
    }
}
#[doc = "RX pin active level inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXINV {
    #[doc = "0: RX pin signal works using the standard logic levels"]
    Standard = 0,
    #[doc = "1: RX pin signal values are inverted"]
    Inverted = 1,
}
impl From<RXINV> for bool {
    #[inline(always)]
    fn from(variant: RXINV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINV` reader - RX pin active level inversion"]
pub type RXINV_R = crate::BitReader<RXINV>;
impl RXINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXINV {
        match self.bits {
            false => RXINV::Standard,
            true => RXINV::Inverted,
        }
    }
    #[doc = "RX pin signal works using the standard logic levels"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == RXINV::Standard
    }
    #[doc = "RX pin signal values are inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == RXINV::Inverted
    }
}
#[doc = "Field `RXINV` writer - RX pin active level inversion"]
pub type RXINV_W<'a, REG> = crate::BitWriter<'a, REG, RXINV>;
impl<'a, REG> RXINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX pin signal works using the standard logic levels"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(RXINV::Standard)
    }
    #[doc = "RX pin signal values are inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(RXINV::Inverted)
    }
}
#[doc = "TX pin active level inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXINV {
    #[doc = "0: TX pin signal works using the standard logic levels"]
    Standard = 0,
    #[doc = "1: TX pin signal values are inverted"]
    Inverted = 1,
}
impl From<TXINV> for bool {
    #[inline(always)]
    fn from(variant: TXINV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINV` reader - TX pin active level inversion"]
pub type TXINV_R = crate::BitReader<TXINV>;
impl TXINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXINV {
        match self.bits {
            false => TXINV::Standard,
            true => TXINV::Inverted,
        }
    }
    #[doc = "TX pin signal works using the standard logic levels"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == TXINV::Standard
    }
    #[doc = "TX pin signal values are inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TXINV::Inverted
    }
}
#[doc = "Field `TXINV` writer - TX pin active level inversion"]
pub type TXINV_W<'a, REG> = crate::BitWriter<'a, REG, TXINV>;
impl<'a, REG> TXINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX pin signal works using the standard logic levels"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(TXINV::Standard)
    }
    #[doc = "TX pin signal values are inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(TXINV::Inverted)
    }
}
#[doc = "Binary data inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATAINV {
    #[doc = "0: Logical data from the data register are send/received in positive/direct logic"]
    Positive = 0,
    #[doc = "1: Logical data from the data register are send/received in negative/inverse logic"]
    Negative = 1,
}
impl From<DATAINV> for bool {
    #[inline(always)]
    fn from(variant: DATAINV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAINV` reader - Binary data inversion"]
pub type DATAINV_R = crate::BitReader<DATAINV>;
impl DATAINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATAINV {
        match self.bits {
            false => DATAINV::Positive,
            true => DATAINV::Negative,
        }
    }
    #[doc = "Logical data from the data register are send/received in positive/direct logic"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == DATAINV::Positive
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == DATAINV::Negative
    }
}
#[doc = "Field `DATAINV` writer - Binary data inversion"]
pub type DATAINV_W<'a, REG> = crate::BitWriter<'a, REG, DATAINV>;
impl<'a, REG> DATAINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Logical data from the data register are send/received in positive/direct logic"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(DATAINV::Positive)
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(DATAINV::Negative)
    }
}
#[doc = "Most significant bit first\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSBFIRST {
    #[doc = "0: data is transmitted/received with data bit 0 first, following the start bit"]
    Lsb = 0,
    #[doc = "1: data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
    Msb = 1,
}
impl From<MSBFIRST> for bool {
    #[inline(always)]
    fn from(variant: MSBFIRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBFIRST` reader - Most significant bit first"]
pub type MSBFIRST_R = crate::BitReader<MSBFIRST>;
impl MSBFIRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSBFIRST {
        match self.bits {
            false => MSBFIRST::Lsb,
            true => MSBFIRST::Msb,
        }
    }
    #[doc = "data is transmitted/received with data bit 0 first, following the start bit"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == MSBFIRST::Lsb
    }
    #[doc = "data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == MSBFIRST::Msb
    }
}
#[doc = "Field `MSBFIRST` writer - Most significant bit first"]
pub type MSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG, MSBFIRST>;
impl<'a, REG> MSBFIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "data is transmitted/received with data bit 0 first, following the start bit"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(MSBFIRST::Lsb)
    }
    #[doc = "data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(MSBFIRST::Msb)
    }
}
#[doc = "Field `ADD` reader - Address of the LPUART node"]
pub type ADD_R = crate::FieldReader;
#[doc = "Field `ADD` writer - Address of the LPUART node"]
pub type ADD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    pub fn addm7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    pub fn datainv(&self) -> DATAINV_R {
        DATAINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    pub fn msbfirst(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Address of the LPUART node"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    #[must_use]
    pub fn addm7(&mut self) -> ADDM7_W<CR2rs> {
        ADDM7_W::new(self, 4)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CR2rs> {
        STOP_W::new(self, 12)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SWAP_W<CR2rs> {
        SWAP_W::new(self, 15)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RXINV_W<CR2rs> {
        RXINV_W::new(self, 16)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TXINV_W<CR2rs> {
        TXINV_W::new(self, 17)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    #[must_use]
    pub fn datainv(&mut self) -> DATAINV_W<CR2rs> {
        DATAINV_W::new(self, 18)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    #[must_use]
    pub fn msbfirst(&mut self) -> MSBFIRST_W<CR2rs> {
        MSBFIRST_W::new(self, 19)
    }
    #[doc = "Bits 24:31 - Address of the LPUART node"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<CR2rs> {
        ADD_W::new(self, 24)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
