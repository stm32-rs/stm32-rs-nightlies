#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "Field `RXBRIE` reader - Rx-Byte Received Interrupt Enable The RXBRIE bit is set and cleared by software."]
pub type RXBRIE_R = crate::BitReader;
#[doc = "Field `RXBRIE` writer - Rx-Byte Received Interrupt Enable The RXBRIE bit is set and cleared by software."]
pub type RXBRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXENDIE` reader - End Of Reception Interrupt Enable The RXENDIE bit is set and cleared by software."]
pub type RXENDIE_R = crate::BitReader;
#[doc = "Field `RXENDIE` writer - End Of Reception Interrupt Enable The RXENDIE bit is set and cleared by software."]
pub type RXENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVRIE` reader - Rx-Buffer Overrun Interrupt Enable The RXOVRIE bit is set and cleared by software."]
pub type RXOVRIE_R = crate::BitReader;
#[doc = "Field `RXOVRIE` writer - Rx-Buffer Overrun Interrupt Enable The RXOVRIE bit is set and cleared by software."]
pub type RXOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BREIE` reader - Bit Rising Error Interrupt Enable The BREIE bit is set and cleared by software."]
pub type BREIE_R = crate::BitReader;
#[doc = "Field `BREIE` writer - Bit Rising Error Interrupt Enable The BREIE bit is set and cleared by software."]
pub type BREIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBPEIE` reader - Short Bit Period Error Interrupt Enable The SBPEIE bit is set and cleared by software."]
pub type SBPEIE_R = crate::BitReader;
#[doc = "Field `SBPEIE` writer - Short Bit Period Error Interrupt Enable The SBPEIE bit is set and cleared by software."]
pub type SBPEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBPEIE` reader - Long Bit Period Error Interrupt Enable The LBPEIE bit is set and cleared by software."]
pub type LBPEIE_R = crate::BitReader;
#[doc = "Field `LBPEIE` writer - Long Bit Period Error Interrupt Enable The LBPEIE bit is set and cleared by software."]
pub type LBPEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXACKIE` reader - Rx-Missing Acknowledge Error Interrupt Enable The RXACKIE bit is set and cleared by software."]
pub type RXACKIE_R = crate::BitReader;
#[doc = "Field `RXACKIE` writer - Rx-Missing Acknowledge Error Interrupt Enable The RXACKIE bit is set and cleared by software."]
pub type RXACKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLSTIE` reader - Arbitration Lost Interrupt Enable The ARBLSTIE bit is set and cleared by software."]
pub type ARBLSTIE_R = crate::BitReader;
#[doc = "Field `ARBLSTIE` writer - Arbitration Lost Interrupt Enable The ARBLSTIE bit is set and cleared by software."]
pub type ARBLSTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBRIE` reader - Tx-Byte Request Interrupt Enable The TXBRIE bit is set and cleared by software."]
pub type TXBRIE_R = crate::BitReader;
#[doc = "Field `TXBRIE` writer - Tx-Byte Request Interrupt Enable The TXBRIE bit is set and cleared by software."]
pub type TXBRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXENDIE` reader - Tx-End Of Message Interrupt Enable The TXENDIE bit is set and cleared by software."]
pub type TXENDIE_R = crate::BitReader;
#[doc = "Field `TXENDIE` writer - Tx-End Of Message Interrupt Enable The TXENDIE bit is set and cleared by software."]
pub type TXENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUDRIE` reader - Tx-Underrun Interrupt Enable The TXUDRIE bit is set and cleared by software."]
pub type TXUDRIE_R = crate::BitReader;
#[doc = "Field `TXUDRIE` writer - Tx-Underrun Interrupt Enable The TXUDRIE bit is set and cleared by software."]
pub type TXUDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRIE` reader - Tx-Error Interrupt Enable The TXERRIE bit is set and cleared by software."]
pub type TXERRIE_R = crate::BitReader;
#[doc = "Field `TXERRIE` writer - Tx-Error Interrupt Enable The TXERRIE bit is set and cleared by software."]
pub type TXERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXACKIE` reader - Tx-Missing Acknowledge Error Interrupt Enable The TXACKEIE bit is set and cleared by software."]
pub type TXACKIE_R = crate::BitReader;
#[doc = "Field `TXACKIE` writer - Tx-Missing Acknowledge Error Interrupt Enable The TXACKEIE bit is set and cleared by software."]
pub type TXACKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx-Byte Received Interrupt Enable The RXBRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxbrie(&self) -> RXBRIE_R {
        RXBRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End Of Reception Interrupt Enable The RXENDIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxendie(&self) -> RXENDIE_R {
        RXENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx-Buffer Overrun Interrupt Enable The RXOVRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit Rising Error Interrupt Enable The BREIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn breie(&self) -> BREIE_R {
        BREIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Short Bit Period Error Interrupt Enable The SBPEIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn sbpeie(&self) -> SBPEIE_R {
        SBPEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Long Bit Period Error Interrupt Enable The LBPEIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn lbpeie(&self) -> LBPEIE_R {
        LBPEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx-Missing Acknowledge Error Interrupt Enable The RXACKIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxackie(&self) -> RXACKIE_R {
        RXACKIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Enable The ARBLSTIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn arblstie(&self) -> ARBLSTIE_R {
        ARBLSTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Tx-Byte Request Interrupt Enable The TXBRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txbrie(&self) -> TXBRIE_R {
        TXBRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tx-End Of Message Interrupt Enable The TXENDIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txendie(&self) -> TXENDIE_R {
        TXENDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx-Underrun Interrupt Enable The TXUDRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txudrie(&self) -> TXUDRIE_R {
        TXUDRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx-Error Interrupt Enable The TXERRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txerrie(&self) -> TXERRIE_R {
        TXERRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx-Missing Acknowledge Error Interrupt Enable The TXACKEIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txackie(&self) -> TXACKIE_R {
        TXACKIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx-Byte Received Interrupt Enable The RXBRIE bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rxbrie(&mut self) -> RXBRIE_W<IERrs> {
        RXBRIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - End Of Reception Interrupt Enable The RXENDIE bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rxendie(&mut self) -> RXENDIE_W<IERrs> {
        RXENDIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Rx-Buffer Overrun Interrupt Enable The RXOVRIE bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rxovrie(&mut self) -> RXOVRIE_W<IERrs> {
        RXOVRIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit Rising Error Interrupt Enable The BREIE bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn breie(&mut self) -> BREIE_W<IERrs> {
        BREIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Short Bit Period Error Interrupt Enable The SBPEIE bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sbpeie(&mut self) -> SBPEIE_W<IERrs> {
        SBPEIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Long Bit Period Error Interrupt Enable The LBPEIE bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lbpeie(&mut self) -> LBPEIE_W<IERrs> {
        LBPEIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Rx-Missing Acknowledge Error Interrupt Enable The RXACKIE bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rxackie(&mut self) -> RXACKIE_W<IERrs> {
        RXACKIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Enable The ARBLSTIE bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn arblstie(&mut self) -> ARBLSTIE_W<IERrs> {
        ARBLSTIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Tx-Byte Request Interrupt Enable The TXBRIE bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn txbrie(&mut self) -> TXBRIE_W<IERrs> {
        TXBRIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Tx-End Of Message Interrupt Enable The TXENDIE bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn txendie(&mut self) -> TXENDIE_W<IERrs> {
        TXENDIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Tx-Underrun Interrupt Enable The TXUDRIE bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn txudrie(&mut self) -> TXUDRIE_W<IERrs> {
        TXUDRIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Tx-Error Interrupt Enable The TXERRIE bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn txerrie(&mut self) -> TXERRIE_W<IERrs> {
        TXERRIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Tx-Missing Acknowledge Error Interrupt Enable The TXACKEIE bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn txackie(&mut self) -> TXACKIE_W<IERrs> {
        TXACKIE_W::new(self, 12)
    }
}
#[doc = "CEC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
