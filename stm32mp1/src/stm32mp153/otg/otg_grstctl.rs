#[doc = "Register `OTG_GRSTCTL` reader"]
pub type R = crate::R<OTG_GRSTCTLrs>;
#[doc = "Register `OTG_GRSTCTL` writer"]
pub type W = crate::W<OTG_GRSTCTLrs>;
#[doc = "Field `CSRST` reader - CSRST"]
pub type CSRST_R = crate::BitReader;
#[doc = "Field `CSRST` writer - CSRST"]
pub type CSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRST` reader - PSRST"]
pub type PSRST_R = crate::BitReader;
#[doc = "Field `PSRST` writer - PSRST"]
pub type PSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFLSH` reader - RXFFLSH"]
pub type RXFFLSH_R = crate::BitReader;
#[doc = "Field `RXFFLSH` writer - RXFFLSH"]
pub type RXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFLSH` reader - TXFFLSH"]
pub type TXFFLSH_R = crate::BitReader;
#[doc = "Field `TXFFLSH` writer - TXFFLSH"]
pub type TXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - TXFNUM"]
pub type TXFNUM_R = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - TXFNUM"]
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DMAREQ` reader - DMAREQ"]
pub type DMAREQ_R = crate::BitReader;
#[doc = "Field `AHBIDL` reader - AHBIDL"]
pub type AHBIDL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CSRST"]
    #[inline(always)]
    pub fn csrst(&self) -> CSRST_R {
        CSRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PSRST"]
    #[inline(always)]
    pub fn psrst(&self) -> PSRST_R {
        PSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFFLSH"]
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXFFLSH"]
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - TXFNUM"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - DMAREQ"]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AHBIDL"]
    #[inline(always)]
    pub fn ahbidl(&self) -> AHBIDL_R {
        AHBIDL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSRST"]
    #[inline(always)]
    #[must_use]
    pub fn csrst(&mut self) -> CSRST_W<OTG_GRSTCTLrs> {
        CSRST_W::new(self, 0)
    }
    #[doc = "Bit 1 - PSRST"]
    #[inline(always)]
    #[must_use]
    pub fn psrst(&mut self) -> PSRST_W<OTG_GRSTCTLrs> {
        PSRST_W::new(self, 1)
    }
    #[doc = "Bit 4 - RXFFLSH"]
    #[inline(always)]
    #[must_use]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W<OTG_GRSTCTLrs> {
        RXFFLSH_W::new(self, 4)
    }
    #[doc = "Bit 5 - TXFFLSH"]
    #[inline(always)]
    #[must_use]
    pub fn txfflsh(&mut self) -> TXFFLSH_W<OTG_GRSTCTLrs> {
        TXFFLSH_W::new(self, 5)
    }
    #[doc = "Bits 6:10 - TXFNUM"]
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TXFNUM_W<OTG_GRSTCTLrs> {
        TXFNUM_W::new(self, 6)
    }
}
#[doc = "The application uses this register to reset various hardware features inside the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_grstctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_grstctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_GRSTCTLrs;
impl crate::RegisterSpec for OTG_GRSTCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_grstctl::R`](R) reader structure"]
impl crate::Readable for OTG_GRSTCTLrs {}
#[doc = "`write(|w| ..)` method takes [`otg_grstctl::W`](W) writer structure"]
impl crate::Writable for OTG_GRSTCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_GRSTCTL to value 0x8000_0000"]
impl crate::Resettable for OTG_GRSTCTLrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
