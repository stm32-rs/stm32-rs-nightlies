#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<ISRrs>;
#[doc = "Field `TXE` reader - Transmit data register empty (transmitters)"]
pub type TXE_R = crate::BitReader;
#[doc = "Field `TXE` writer - Transmit data register empty (transmitters)"]
pub type TXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIS` reader - Transmit interrupt status (transmitters)"]
pub type TXIS_R = crate::BitReader;
#[doc = "Field `TXIS` writer - Transmit interrupt status (transmitters)"]
pub type TXIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNE` reader - Receive data register not empty (receivers)"]
pub type RXNE_R = crate::BitReader;
#[doc = "Field `ADDR` reader - Address matched (slave mode)"]
pub type ADDR_R = crate::BitReader;
#[doc = "Field `NACKF` reader - Not acknowledge received flag"]
pub type NACKF_R = crate::BitReader;
#[doc = "Field `STOPF` reader - Stop detection flag"]
pub type STOPF_R = crate::BitReader;
#[doc = "Field `TC` reader - Transfer Complete (master mode)"]
pub type TC_R = crate::BitReader;
#[doc = "Field `TCR` reader - Transfer Complete Reload"]
pub type TCR_R = crate::BitReader;
#[doc = "Field `BERR` reader - Bus error"]
pub type BERR_R = crate::BitReader;
#[doc = "Field `ARLO` reader - Arbitration lost"]
pub type ARLO_R = crate::BitReader;
#[doc = "Field `OVR` reader - Overrun/Underrun (slave mode)"]
pub type OVR_R = crate::BitReader;
#[doc = "Field `PECERR` reader - PEC Error in reception"]
pub type PECERR_R = crate::BitReader;
#[doc = "Field `TIMEOUT` reader - Timeout or t_low detection flag"]
pub type TIMEOUT_R = crate::BitReader;
#[doc = "Field `ALERT` reader - SMBus alert"]
pub type ALERT_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Bus busy"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `DIR` reader - Transfer direction (Slave mode)"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `ADDCODE` reader - Address match code (Slave mode)"]
pub type ADDCODE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmit data register empty (transmitters)"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt status (transmitters)"]
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive data register not empty (receivers)"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address matched (slave mode)"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Not acknowledge received flag"]
    #[inline(always)]
    pub fn nackf(&self) -> NACKF_R {
        NACKF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop detection flag"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Complete (master mode)"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transfer Complete Reload"]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost"]
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun/Underrun (slave mode)"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PEC Error in reception"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timeout or t_low detection flag"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Bus busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transfer direction (Slave mode)"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Address match code (Slave mode)"]
    #[inline(always)]
    pub fn addcode(&self) -> ADDCODE_R {
        ADDCODE_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit data register empty (transmitters)"]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<ISRrs> {
        TXE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit interrupt status (transmitters)"]
    #[inline(always)]
    #[must_use]
    pub fn txis(&mut self) -> TXIS_W<ISRrs> {
        TXIS_W::new(self, 1)
    }
}
#[doc = "Interrupt and Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISR to value 0x01"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0x01;
}
