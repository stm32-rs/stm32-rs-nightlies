#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Field `PE` reader - Parity error"]
pub type PE_R = crate::BitReader;
#[doc = "Field `FE` reader - Framing error"]
pub type FE_R = crate::BitReader;
#[doc = "Field `NF` reader - Noise detected flag"]
pub type NF_R = crate::BitReader;
#[doc = "Field `ORE` reader - Overrun error"]
pub type ORE_R = crate::BitReader;
#[doc = "Field `IDLE` reader - IDLE line detected"]
pub type IDLE_R = crate::BitReader;
#[doc = "Field `RXNE` reader - Read data register not empty"]
pub type RXNE_R = crate::BitReader;
#[doc = "Field `RXNE` writer - Read data register not empty"]
pub type RXNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - Transmission complete"]
pub type TC_R = crate::BitReader;
#[doc = "Field `TC` writer - Transmission complete"]
pub type TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE` reader - Transmit data register empty"]
pub type TXE_R = crate::BitReader;
#[doc = "Field `LBD` reader - LIN break detection flag"]
pub type LBD_R = crate::BitReader;
#[doc = "Field `LBD` writer - LIN break detection flag"]
pub type LBD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS` reader - CTS flag"]
pub type CTS_R = crate::BitReader;
#[doc = "Field `CTS` writer - CTS flag"]
pub type CTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Parity error"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing error"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise detected flag"]
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error"]
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE line detected"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read data register not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data register empty"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LIN break detection flag"]
    #[inline(always)]
    pub fn lbd(&self) -> LBD_R {
        LBD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS flag"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Read data register not empty"]
    #[inline(always)]
    #[must_use]
    pub fn rxne(&mut self) -> RXNE_W<SRrs> {
        RXNE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<SRrs> {
        TC_W::new(self, 6)
    }
    #[doc = "Bit 8 - LIN break detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn lbd(&mut self) -> LBD_W<SRrs> {
        LBD_W::new(self, 8)
    }
    #[doc = "Bit 9 - CTS flag"]
    #[inline(always)]
    #[must_use]
    pub fn cts(&mut self) -> CTS_W<SRrs> {
        CTS_W::new(self, 9)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0xc0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0xc0;
}
