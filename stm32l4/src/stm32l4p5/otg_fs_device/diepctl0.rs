#[doc = "Register `DIEPCTL0` reader"]
pub type R = crate::R<DIEPCTL0rs>;
#[doc = "Register `DIEPCTL0` writer"]
pub type W = crate::W<DIEPCTL0rs>;
#[doc = "Field `MPSIZ` reader - Maximum packet size"]
pub type MPSIZ_R = crate::FieldReader;
#[doc = "Field `MPSIZ` writer - Maximum packet size"]
pub type MPSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USBAEP` reader - USB active endpoint"]
pub type USBAEP_R = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK status"]
pub type NAKSTS_R = crate::BitReader;
#[doc = "Field `EPTYP` reader - Endpoint type"]
pub type EPTYP_R = crate::FieldReader;
#[doc = "Field `STALL` reader - STALL handshake"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - STALL handshake"]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - TxFIFO number"]
pub type TXFNUM_R = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - TxFIFO number"]
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - Endpoint disable"]
pub type EPDIS_R = crate::BitReader;
#[doc = "Field `EPENA` reader - Endpoint enable"]
pub type EPENA_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Maximum packet size"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - USB active endpoint"]
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK status"]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - TxFIFO number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Maximum packet size"]
    #[inline(always)]
    #[must_use]
    pub fn mpsiz(&mut self) -> MPSIZ_W<DIEPCTL0rs> {
        MPSIZ_W::new(self, 0)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<DIEPCTL0rs> {
        STALL_W::new(self, 21)
    }
    #[doc = "Bits 22:25 - TxFIFO number"]
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TXFNUM_W<DIEPCTL0rs> {
        TXFNUM_W::new(self, 22)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<DIEPCTL0rs> {
        CNAK_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<DIEPCTL0rs> {
        SNAK_W::new(self, 27)
    }
}
#[doc = "OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPCTL0rs;
impl crate::RegisterSpec for DIEPCTL0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepctl0::R`](R) reader structure"]
impl crate::Readable for DIEPCTL0rs {}
#[doc = "`write(|w| ..)` method takes [`diepctl0::W`](W) writer structure"]
impl crate::Writable for DIEPCTL0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPCTL0 to value 0"]
impl crate::Resettable for DIEPCTL0rs {
    const RESET_VALUE: u32 = 0;
}
