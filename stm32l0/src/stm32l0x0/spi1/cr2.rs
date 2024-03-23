#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Rx buffer DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN {
    #[doc = "0: Rx buffer DMA disabled"]
    Disabled = 0,
    #[doc = "1: Rx buffer DMA enabled"]
    Enabled = 1,
}
impl From<RXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAEN` reader - Rx buffer DMA enable"]
pub type RXDMAEN_R = crate::BitReader<RXDMAEN>;
impl RXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXDMAEN {
        match self.bits {
            false => RXDMAEN::Disabled,
            true => RXDMAEN::Enabled,
        }
    }
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAEN::Disabled
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAEN::Enabled
    }
}
#[doc = "Field `RXDMAEN` writer - Rx buffer DMA enable"]
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, RXDMAEN>;
impl<'a, REG> RXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::Disabled)
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::Enabled)
    }
}
#[doc = "Tx buffer DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN {
    #[doc = "0: Tx buffer DMA disabled"]
    Disabled = 0,
    #[doc = "1: Tx buffer DMA enabled"]
    Enabled = 1,
}
impl From<TXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAEN` reader - Tx buffer DMA enable"]
pub type TXDMAEN_R = crate::BitReader<TXDMAEN>;
impl TXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXDMAEN {
        match self.bits {
            false => TXDMAEN::Disabled,
            true => TXDMAEN::Enabled,
        }
    }
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAEN::Disabled
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAEN::Enabled
    }
}
#[doc = "Field `TXDMAEN` writer - Tx buffer DMA enable"]
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, TXDMAEN>;
impl<'a, REG> TXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::Disabled)
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::Enabled)
    }
}
#[doc = "SS output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSOE {
    #[doc = "0: SS output is disabled in master mode"]
    Disabled = 0,
    #[doc = "1: SS output is enabled in master mode"]
    Enabled = 1,
}
impl From<SSOE> for bool {
    #[inline(always)]
    fn from(variant: SSOE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSOE` reader - SS output enable"]
pub type SSOE_R = crate::BitReader<SSOE>;
impl SSOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSOE {
        match self.bits {
            false => SSOE::Disabled,
            true => SSOE::Enabled,
        }
    }
    #[doc = "SS output is disabled in master mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSOE::Disabled
    }
    #[doc = "SS output is enabled in master mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSOE::Enabled
    }
}
#[doc = "Field `SSOE` writer - SS output enable"]
pub type SSOE_W<'a, REG> = crate::BitWriter<'a, REG, SSOE>;
impl<'a, REG> SSOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SS output is disabled in master mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSOE::Disabled)
    }
    #[doc = "SS output is enabled in master mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSOE::Enabled)
    }
}
#[doc = "Field `FRF` reader - Frame format"]
pub type FRF_R = crate::BitReader;
#[doc = "Field `FRF` writer - Frame format"]
pub type FRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE {
    #[doc = "0: Error interrupt masked"]
    Masked = 0,
    #[doc = "1: Error interrupt not masked"]
    NotMasked = 1,
}
impl From<ERRIE> for bool {
    #[inline(always)]
    fn from(variant: ERRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<ERRIE>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE {
        match self.bits {
            false => ERRIE::Masked,
            true => ERRIE::NotMasked,
        }
    }
    #[doc = "Error interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == ERRIE::Masked
    }
    #[doc = "Error interrupt not masked"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == ERRIE::NotMasked
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Masked)
    }
    #[doc = "Error interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::NotMasked)
    }
}
#[doc = "RX buffer not empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNEIE {
    #[doc = "0: RXE interrupt masked"]
    Masked = 0,
    #[doc = "1: RXE interrupt not masked"]
    NotMasked = 1,
}
impl From<RXNEIE> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNEIE` reader - RX buffer not empty interrupt enable"]
pub type RXNEIE_R = crate::BitReader<RXNEIE>;
impl RXNEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXNEIE {
        match self.bits {
            false => RXNEIE::Masked,
            true => RXNEIE::NotMasked,
        }
    }
    #[doc = "RXE interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RXNEIE::Masked
    }
    #[doc = "RXE interrupt not masked"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == RXNEIE::NotMasked
    }
}
#[doc = "Field `RXNEIE` writer - RX buffer not empty interrupt enable"]
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG, RXNEIE>;
impl<'a, REG> RXNEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RXE interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE::Masked)
    }
    #[doc = "RXE interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE::NotMasked)
    }
}
#[doc = "Tx buffer empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEIE {
    #[doc = "0: TXE interrupt masked"]
    Masked = 0,
    #[doc = "1: TXE interrupt not masked"]
    NotMasked = 1,
}
impl From<TXEIE> for bool {
    #[inline(always)]
    fn from(variant: TXEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEIE` reader - Tx buffer empty interrupt enable"]
pub type TXEIE_R = crate::BitReader<TXEIE>;
impl TXEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXEIE {
        match self.bits {
            false => TXEIE::Masked,
            true => TXEIE::NotMasked,
        }
    }
    #[doc = "TXE interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXEIE::Masked
    }
    #[doc = "TXE interrupt not masked"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TXEIE::NotMasked
    }
}
#[doc = "Field `TXEIE` writer - Tx buffer empty interrupt enable"]
pub type TXEIE_W<'a, REG> = crate::BitWriter<'a, REG, TXEIE>;
impl<'a, REG> TXEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TXE interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE::Masked)
    }
    #[doc = "TXE interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE::NotMasked)
    }
}
impl R {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<CR2rs> {
        RXDMAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<CR2rs> {
        TXDMAEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssoe(&mut self) -> SSOE_W<CR2rs> {
        SSOE_W::new(self, 2)
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FRF_W<CR2rs> {
        FRF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CR2rs> {
        ERRIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<CR2rs> {
        RXNEIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TXEIE_W<CR2rs> {
        TXEIE_W::new(self, 7)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
