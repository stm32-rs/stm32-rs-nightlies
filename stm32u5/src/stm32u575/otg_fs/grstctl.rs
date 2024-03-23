#[doc = "Register `GRSTCTL` reader"]
pub type R = crate::R<GRSTCTLrs>;
#[doc = "Register `GRSTCTL` writer"]
pub type W = crate::W<GRSTCTLrs>;
#[doc = "Field `CSRST` reader - CSRST"]
pub type CSRST_R = crate::BitReader;
#[doc = "Field `PSRST` reader - PSRST"]
pub type PSRST_R = crate::BitReader;
#[doc = "Field `PSRST` writer - PSRST"]
pub type PSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSRST` reader - FSRST"]
pub type FSRST_R = crate::BitReader;
#[doc = "Field `FSRST` writer - FSRST"]
pub type FSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - FSRST"]
    #[inline(always)]
    pub fn fsrst(&self) -> FSRST_R {
        FSRST_R::new(((self.bits >> 2) & 1) != 0)
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
    #[doc = "Bit 31 - AHBIDL"]
    #[inline(always)]
    pub fn ahbidl(&self) -> AHBIDL_R {
        AHBIDL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - PSRST"]
    #[inline(always)]
    #[must_use]
    pub fn psrst(&mut self) -> PSRST_W<GRSTCTLrs> {
        PSRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - FSRST"]
    #[inline(always)]
    #[must_use]
    pub fn fsrst(&mut self) -> FSRST_W<GRSTCTLrs> {
        FSRST_W::new(self, 2)
    }
    #[doc = "Bit 4 - RXFFLSH"]
    #[inline(always)]
    #[must_use]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W<GRSTCTLrs> {
        RXFFLSH_W::new(self, 4)
    }
    #[doc = "Bit 5 - TXFFLSH"]
    #[inline(always)]
    #[must_use]
    pub fn txfflsh(&mut self) -> TXFFLSH_W<GRSTCTLrs> {
        TXFFLSH_W::new(self, 5)
    }
    #[doc = "Bits 6:10 - TXFNUM"]
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TXFNUM_W<GRSTCTLrs> {
        TXFNUM_W::new(self, 6)
    }
}
#[doc = "The application uses this register to reset various hardware features inside the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets GRSTCTL to value 0x8000_0000"]
impl crate::Resettable for GRSTCTLrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
