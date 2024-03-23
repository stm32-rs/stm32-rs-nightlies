#[doc = "Register `CR3` reader"]
pub type R = crate::R<CR3rs>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<CR3rs>;
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EIE_R = crate::BitReader;
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IREN` reader - IrDA mode enable"]
pub type IREN_R = crate::BitReader;
#[doc = "Field `IREN` writer - IrDA mode enable"]
pub type IREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRLP` reader - IrDA low-power"]
pub type IRLP_R = crate::BitReader;
#[doc = "Field `IRLP` writer - IrDA low-power"]
pub type IRLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDSEL` reader - Half-duplex selection"]
pub type HDSEL_R = crate::BitReader;
#[doc = "Field `HDSEL` writer - Half-duplex selection"]
pub type HDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - Smartcard NACK enable"]
pub type NACK_R = crate::BitReader;
#[doc = "Field `NACK` writer - Smartcard NACK enable"]
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCEN` reader - Smartcard mode enable"]
pub type SCEN_R = crate::BitReader;
#[doc = "Field `SCEN` writer - Smartcard mode enable"]
pub type SCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAR` reader - DMA enable receiver"]
pub type DMAR_R = crate::BitReader;
#[doc = "Field `DMAR` writer - DMA enable receiver"]
pub type DMAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAT` reader - DMA enable transmitter"]
pub type DMAT_R = crate::BitReader;
#[doc = "Field `DMAT` writer - DMA enable transmitter"]
pub type DMAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSE` reader - RTS enable"]
pub type RTSE_R = crate::BitReader;
#[doc = "Field `RTSE` writer - RTS enable"]
pub type RTSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSE` reader - CTS enable"]
pub type CTSE_R = crate::BitReader;
#[doc = "Field `CTSE` writer - CTS enable"]
pub type CTSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSIE` reader - CTS interrupt enable"]
pub type CTSIE_R = crate::BitReader;
#[doc = "Field `CTSIE` writer - CTS interrupt enable"]
pub type CTSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONEBIT` reader - One sample bit method enable"]
pub type ONEBIT_R = crate::BitReader;
#[doc = "Field `ONEBIT` writer - One sample bit method enable"]
pub type ONEBIT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - One sample bit method enable"]
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<CR3rs> {
        EIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<CR3rs> {
        IREN_W::new(self, 1)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    #[must_use]
    pub fn irlp(&mut self) -> IRLP_W<CR3rs> {
        IRLP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    #[must_use]
    pub fn hdsel(&mut self) -> HDSEL_W<CR3rs> {
        HDSEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<CR3rs> {
        NACK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn scen(&mut self) -> SCEN_W<CR3rs> {
        SCEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    #[must_use]
    pub fn dmar(&mut self) -> DMAR_W<CR3rs> {
        DMAR_W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn dmat(&mut self) -> DMAT_W<CR3rs> {
        DMAT_W::new(self, 7)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtse(&mut self) -> RTSE_W<CR3rs> {
        RTSE_W::new(self, 8)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctse(&mut self) -> CTSE_W<CR3rs> {
        CTSE_W::new(self, 9)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsie(&mut self) -> CTSIE_W<CR3rs> {
        CTSIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - One sample bit method enable"]
    #[inline(always)]
    #[must_use]
    pub fn onebit(&mut self) -> ONEBIT_W<CR3rs> {
        ONEBIT_W::new(self, 11)
    }
}
#[doc = "Control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for CR3rs {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0;
}
