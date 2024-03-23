#[doc = "Register `GAHBCFG` reader"]
pub type R = crate::R<GAHBCFGrs>;
#[doc = "Register `GAHBCFG` writer"]
pub type W = crate::W<GAHBCFGrs>;
#[doc = "Field `GINT` reader - Global interrupt mask"]
pub type GINT_R = crate::BitReader;
#[doc = "Field `GINT` writer - Global interrupt mask"]
pub type GINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBSTLEN` reader - Burst length/type"]
pub type HBSTLEN_R = crate::FieldReader;
#[doc = "Field `HBSTLEN` writer - Burst length/type"]
pub type HBSTLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFELVL` reader - TxFIFO empty level"]
pub type TXFELVL_R = crate::BitReader;
#[doc = "Field `TXFELVL` writer - TxFIFO empty level"]
pub type TXFELVL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFELVL` reader - Periodic TxFIFO empty level"]
pub type PTXFELVL_R = crate::BitReader;
#[doc = "Field `PTXFELVL` writer - Periodic TxFIFO empty level"]
pub type PTXFELVL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    pub fn gint(&self) -> GINT_R {
        GINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Burst length/type"]
    #[inline(always)]
    pub fn hbstlen(&self) -> HBSTLEN_R {
        HBSTLEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - TxFIFO empty level"]
    #[inline(always)]
    pub fn txfelvl(&self) -> TXFELVL_R {
        TXFELVL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    pub fn ptxfelvl(&self) -> PTXFELVL_R {
        PTXFELVL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn gint(&mut self) -> GINT_W<GAHBCFGrs> {
        GINT_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - Burst length/type"]
    #[inline(always)]
    #[must_use]
    pub fn hbstlen(&mut self) -> HBSTLEN_W<GAHBCFGrs> {
        HBSTLEN_W::new(self, 1)
    }
    #[doc = "Bit 5 - DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<GAHBCFGrs> {
        DMAEN_W::new(self, 5)
    }
    #[doc = "Bit 7 - TxFIFO empty level"]
    #[inline(always)]
    #[must_use]
    pub fn txfelvl(&mut self) -> TXFELVL_W<GAHBCFGrs> {
        TXFELVL_W::new(self, 7)
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfelvl(&mut self) -> PTXFELVL_W<GAHBCFGrs> {
        PTXFELVL_W::new(self, 8)
    }
}
#[doc = "OTG_HS AHB configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAHBCFGrs;
impl crate::RegisterSpec for GAHBCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gahbcfg::R`](R) reader structure"]
impl crate::Readable for GAHBCFGrs {}
#[doc = "`write(|w| ..)` method takes [`gahbcfg::W`](W) writer structure"]
impl crate::Writable for GAHBCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GAHBCFGrs {
    const RESET_VALUE: u32 = 0;
}
