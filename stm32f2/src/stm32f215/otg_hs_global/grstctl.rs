#[doc = "Register `GRSTCTL` reader"]
pub type R = crate::R<GRSTCTLrs>;
#[doc = "Register `GRSTCTL` writer"]
pub type W = crate::W<GRSTCTLrs>;
#[doc = "Field `CSRST` reader - Core soft reset"]
pub type CSRST_R = crate::BitReader;
#[doc = "Field `CSRST` writer - Core soft reset"]
pub type CSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSRST` reader - HCLK soft reset"]
pub type HSRST_R = crate::BitReader;
#[doc = "Field `HSRST` writer - HCLK soft reset"]
pub type HSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCRST` reader - Host frame counter reset"]
pub type FCRST_R = crate::BitReader;
#[doc = "Field `FCRST` writer - Host frame counter reset"]
pub type FCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFLSH` reader - RxFIFO flush"]
pub type RXFFLSH_R = crate::BitReader;
#[doc = "Field `RXFFLSH` writer - RxFIFO flush"]
pub type RXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFLSH` reader - TxFIFO flush"]
pub type TXFFLSH_R = crate::BitReader;
#[doc = "Field `TXFFLSH` writer - TxFIFO flush"]
pub type TXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - TxFIFO number"]
pub type TXFNUM_R = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - TxFIFO number"]
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DMAREQ` reader - DMA request signal"]
pub type DMAREQ_R = crate::BitReader;
#[doc = "Field `AHBIDL` reader - AHB master idle"]
pub type AHBIDL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Core soft reset"]
    #[inline(always)]
    pub fn csrst(&self) -> CSRST_R {
        CSRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HCLK soft reset"]
    #[inline(always)]
    pub fn hsrst(&self) -> HSRST_R {
        HSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host frame counter reset"]
    #[inline(always)]
    pub fn fcrst(&self) -> FCRST_R {
        FCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO flush"]
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TxFIFO flush"]
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - TxFIFO number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - DMA request signal"]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AHB master idle"]
    #[inline(always)]
    pub fn ahbidl(&self) -> AHBIDL_R {
        AHBIDL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core soft reset"]
    #[inline(always)]
    #[must_use]
    pub fn csrst(&mut self) -> CSRST_W<GRSTCTLrs> {
        CSRST_W::new(self, 0)
    }
    #[doc = "Bit 1 - HCLK soft reset"]
    #[inline(always)]
    #[must_use]
    pub fn hsrst(&mut self) -> HSRST_W<GRSTCTLrs> {
        HSRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Host frame counter reset"]
    #[inline(always)]
    #[must_use]
    pub fn fcrst(&mut self) -> FCRST_W<GRSTCTLrs> {
        FCRST_W::new(self, 2)
    }
    #[doc = "Bit 4 - RxFIFO flush"]
    #[inline(always)]
    #[must_use]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W<GRSTCTLrs> {
        RXFFLSH_W::new(self, 4)
    }
    #[doc = "Bit 5 - TxFIFO flush"]
    #[inline(always)]
    #[must_use]
    pub fn txfflsh(&mut self) -> TXFFLSH_W<GRSTCTLrs> {
        TXFFLSH_W::new(self, 5)
    }
    #[doc = "Bits 6:10 - TxFIFO number"]
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TXFNUM_W<GRSTCTLrs> {
        TXFNUM_W::new(self, 6)
    }
}
#[doc = "OTG_HS reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRSTCTLrs;
impl crate::RegisterSpec for GRSTCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grstctl::R`](R) reader structure"]
impl crate::Readable for GRSTCTLrs {}
#[doc = "`write(|w| ..)` method takes [`grstctl::W`](W) writer structure"]
impl crate::Writable for GRSTCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRSTCTL to value 0x2000_0000"]
impl crate::Resettable for GRSTCTLrs {
    const RESET_VALUE: u32 = 0x2000_0000;
}
