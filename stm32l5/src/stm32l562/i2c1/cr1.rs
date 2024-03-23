#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "Field `PE` reader - Peripheral enable"]
pub type PE_R = crate::BitReader;
#[doc = "Field `PE` writer - Peripheral enable"]
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIE` reader - TX Interrupt enable"]
pub type TXIE_R = crate::BitReader;
#[doc = "Field `TXIE` writer - TX Interrupt enable"]
pub type TXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIE` reader - RX Interrupt enable"]
pub type RXIE_R = crate::BitReader;
#[doc = "Field `RXIE` writer - RX Interrupt enable"]
pub type RXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRIE` reader - Address match interrupt enable (slave only)"]
pub type ADDRIE_R = crate::BitReader;
#[doc = "Field `ADDRIE` writer - Address match interrupt enable (slave only)"]
pub type ADDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACKIE` reader - Not acknowledge received interrupt enable"]
pub type NACKIE_R = crate::BitReader;
#[doc = "Field `NACKIE` writer - Not acknowledge received interrupt enable"]
pub type NACKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPIE` reader - STOP detection Interrupt enable"]
pub type STOPIE_R = crate::BitReader;
#[doc = "Field `STOPIE` writer - STOP detection Interrupt enable"]
pub type STOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - Transfer Complete interrupt enable"]
pub type TCIE_R = crate::BitReader;
#[doc = "Field `TCIE` writer - Transfer Complete interrupt enable"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupts enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupts enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DNF` reader - Digital noise filter"]
pub type DNF_R = crate::FieldReader;
#[doc = "Field `DNF` writer - Digital noise filter"]
pub type DNF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ANFOFF` reader - Analog noise filter OFF"]
pub type ANFOFF_R = crate::BitReader;
#[doc = "Field `ANFOFF` writer - Analog noise filter OFF"]
pub type ANFOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAEN` reader - DMA transmission requests enable"]
pub type TXDMAEN_R = crate::BitReader;
#[doc = "Field `TXDMAEN` writer - DMA transmission requests enable"]
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAEN` reader - DMA reception requests enable"]
pub type RXDMAEN_R = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - DMA reception requests enable"]
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBC` reader - Slave byte control"]
pub type SBC_R = crate::BitReader;
#[doc = "Field `SBC` writer - Slave byte control"]
pub type SBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOSTRETCH` reader - Clock stretching disable"]
pub type NOSTRETCH_R = crate::BitReader;
#[doc = "Field `NOSTRETCH` writer - Clock stretching disable"]
pub type NOSTRETCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPEN` reader - Wakeup from STOP enable"]
pub type WUPEN_R = crate::BitReader;
#[doc = "Field `WUPEN` writer - Wakeup from STOP enable"]
pub type WUPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCEN` reader - General call enable"]
pub type GCEN_R = crate::BitReader;
#[doc = "Field `GCEN` writer - General call enable"]
pub type GCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBHEN` reader - SMBus Host address enable"]
pub type SMBHEN_R = crate::BitReader;
#[doc = "Field `SMBHEN` writer - SMBus Host address enable"]
pub type SMBHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBDEN` reader - SMBus Device Default address enable"]
pub type SMBDEN_R = crate::BitReader;
#[doc = "Field `SMBDEN` writer - SMBus Device Default address enable"]
pub type SMBDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERTEN` reader - SMBUS alert enable"]
pub type ALERTEN_R = crate::BitReader;
#[doc = "Field `ALERTEN` writer - SMBUS alert enable"]
pub type ALERTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECEN` reader - PEC enable"]
pub type PECEN_R = crate::BitReader;
#[doc = "Field `PECEN` writer - PEC enable"]
pub type PECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Interrupt enable"]
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Interrupt enable"]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address match interrupt enable (slave only)"]
    #[inline(always)]
    pub fn addrie(&self) -> ADDRIE_R {
        ADDRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Not acknowledge received interrupt enable"]
    #[inline(always)]
    pub fn nackie(&self) -> NACKIE_R {
        NACKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - STOP detection Interrupt enable"]
    #[inline(always)]
    pub fn stopie(&self) -> STOPIE_R {
        STOPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error interrupts enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Digital noise filter"]
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Analog noise filter OFF"]
    #[inline(always)]
    pub fn anfoff(&self) -> ANFOFF_R {
        ANFOFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA transmission requests enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA reception requests enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave byte control"]
    #[inline(always)]
    pub fn sbc(&self) -> SBC_R {
        SBC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clock stretching disable"]
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wakeup from STOP enable"]
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - General call enable"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SMBus Host address enable"]
    #[inline(always)]
    pub fn smbhen(&self) -> SMBHEN_R {
        SMBHEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBus Device Default address enable"]
    #[inline(always)]
    pub fn smbden(&self) -> SMBDEN_R {
        SMBDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SMBUS alert enable"]
    #[inline(always)]
    pub fn alerten(&self) -> ALERTEN_R {
        ALERTEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PEC enable"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<CR1rs> {
        PE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TX Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txie(&mut self) -> TXIE_W<CR1rs> {
        TXIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - RX Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxie(&mut self) -> RXIE_W<CR1rs> {
        RXIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Address match interrupt enable (slave only)"]
    #[inline(always)]
    #[must_use]
    pub fn addrie(&mut self) -> ADDRIE_W<CR1rs> {
        ADDRIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Not acknowledge received interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn nackie(&mut self) -> NACKIE_W<CR1rs> {
        NACKIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - STOP detection Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stopie(&mut self) -> STOPIE_W<CR1rs> {
        STOPIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer Complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CR1rs> {
        TCIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Error interrupts enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CR1rs> {
        ERRIE_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Digital noise filter"]
    #[inline(always)]
    #[must_use]
    pub fn dnf(&mut self) -> DNF_W<CR1rs> {
        DNF_W::new(self, 8)
    }
    #[doc = "Bit 12 - Analog noise filter OFF"]
    #[inline(always)]
    #[must_use]
    pub fn anfoff(&mut self) -> ANFOFF_W<CR1rs> {
        ANFOFF_W::new(self, 12)
    }
    #[doc = "Bit 14 - DMA transmission requests enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<CR1rs> {
        TXDMAEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - DMA reception requests enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<CR1rs> {
        RXDMAEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Slave byte control"]
    #[inline(always)]
    #[must_use]
    pub fn sbc(&mut self) -> SBC_W<CR1rs> {
        SBC_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clock stretching disable"]
    #[inline(always)]
    #[must_use]
    pub fn nostretch(&mut self) -> NOSTRETCH_W<CR1rs> {
        NOSTRETCH_W::new(self, 17)
    }
    #[doc = "Bit 18 - Wakeup from STOP enable"]
    #[inline(always)]
    #[must_use]
    pub fn wupen(&mut self) -> WUPEN_W<CR1rs> {
        WUPEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - General call enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GCEN_W<CR1rs> {
        GCEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - SMBus Host address enable"]
    #[inline(always)]
    #[must_use]
    pub fn smbhen(&mut self) -> SMBHEN_W<CR1rs> {
        SMBHEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - SMBus Device Default address enable"]
    #[inline(always)]
    #[must_use]
    pub fn smbden(&mut self) -> SMBDEN_W<CR1rs> {
        SMBDEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - SMBUS alert enable"]
    #[inline(always)]
    #[must_use]
    pub fn alerten(&mut self) -> ALERTEN_W<CR1rs> {
        ALERTEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - PEC enable"]
    #[inline(always)]
    #[must_use]
    pub fn pecen(&mut self) -> PECEN_W<CR1rs> {
        PECEN_W::new(self, 23)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0;
}
