#[doc = "Register `MMC_TX_INTERRUPT_MASK` reader"]
pub type R = crate::R<MMC_TX_INTERRUPT_MASKrs>;
#[doc = "Register `MMC_TX_INTERRUPT_MASK` writer"]
pub type W = crate::W<MMC_TX_INTERRUPT_MASKrs>;
#[doc = "Field `TXSCOLGPIM` reader - TXSCOLGPIM"]
pub type TXSCOLGPIM_R = crate::BitReader;
#[doc = "Field `TXSCOLGPIM` writer - TXSCOLGPIM"]
pub type TXSCOLGPIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMCOLGPIM` reader - TXMCOLGPIM"]
pub type TXMCOLGPIM_R = crate::BitReader;
#[doc = "Field `TXMCOLGPIM` writer - TXMCOLGPIM"]
pub type TXMCOLGPIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXGPKTIM` reader - TXGPKTIM"]
pub type TXGPKTIM_R = crate::BitReader;
#[doc = "Field `TXGPKTIM` writer - TXGPKTIM"]
pub type TXGPKTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXLPIUSCIM` reader - TXLPIUSCIM"]
pub type TXLPIUSCIM_R = crate::BitReader;
#[doc = "Field `TXLPIUSCIM` writer - TXLPIUSCIM"]
pub type TXLPIUSCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXLPITRCIM` reader - TXLPITRCIM"]
pub type TXLPITRCIM_R = crate::BitReader;
impl R {
    #[doc = "Bit 14 - TXSCOLGPIM"]
    #[inline(always)]
    pub fn txscolgpim(&self) -> TXSCOLGPIM_R {
        TXSCOLGPIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TXMCOLGPIM"]
    #[inline(always)]
    pub fn txmcolgpim(&self) -> TXMCOLGPIM_R {
        TXMCOLGPIM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - TXGPKTIM"]
    #[inline(always)]
    pub fn txgpktim(&self) -> TXGPKTIM_R {
        TXGPKTIM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - TXLPIUSCIM"]
    #[inline(always)]
    pub fn txlpiuscim(&self) -> TXLPIUSCIM_R {
        TXLPIUSCIM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TXLPITRCIM"]
    #[inline(always)]
    pub fn txlpitrcim(&self) -> TXLPITRCIM_R {
        TXLPITRCIM_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - TXSCOLGPIM"]
    #[inline(always)]
    #[must_use]
    pub fn txscolgpim(&mut self) -> TXSCOLGPIM_W<MMC_TX_INTERRUPT_MASKrs> {
        TXSCOLGPIM_W::new(self, 14)
    }
    #[doc = "Bit 15 - TXMCOLGPIM"]
    #[inline(always)]
    #[must_use]
    pub fn txmcolgpim(&mut self) -> TXMCOLGPIM_W<MMC_TX_INTERRUPT_MASKrs> {
        TXMCOLGPIM_W::new(self, 15)
    }
    #[doc = "Bit 21 - TXGPKTIM"]
    #[inline(always)]
    #[must_use]
    pub fn txgpktim(&mut self) -> TXGPKTIM_W<MMC_TX_INTERRUPT_MASKrs> {
        TXGPKTIM_W::new(self, 21)
    }
    #[doc = "Bit 26 - TXLPIUSCIM"]
    #[inline(always)]
    #[must_use]
    pub fn txlpiuscim(&mut self) -> TXLPIUSCIM_W<MMC_TX_INTERRUPT_MASKrs> {
        TXLPIUSCIM_W::new(self, 26)
    }
}
#[doc = "This register maintains the masks for interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt Mask register maintains the masks for the interrupts generated when the transmit statistic counters reach half of their maximum value or the maximum values. This register is 32 bit wide. This register is present only when any one of the MMC Transmit Counters is selected during core configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_tx_interrupt_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_tx_interrupt_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_TX_INTERRUPT_MASKrs;
impl crate::RegisterSpec for MMC_TX_INTERRUPT_MASKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_tx_interrupt_mask::R`](R) reader structure"]
impl crate::Readable for MMC_TX_INTERRUPT_MASKrs {}
#[doc = "`write(|w| ..)` method takes [`mmc_tx_interrupt_mask::W`](W) writer structure"]
impl crate::Writable for MMC_TX_INTERRUPT_MASKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_TX_INTERRUPT_MASK to value 0"]
impl crate::Resettable for MMC_TX_INTERRUPT_MASKrs {
    const RESET_VALUE: u32 = 0;
}
