#[doc = "Register `FDCAN_RXGFC` reader"]
pub type R = crate::R<FDCAN_RXGFCrs>;
#[doc = "Register `FDCAN_RXGFC` writer"]
pub type W = crate::W<FDCAN_RXGFCrs>;
#[doc = "Field `RRFE` reader - Reject Remote Frames Extended"]
pub type RRFE_R = crate::BitReader;
#[doc = "Field `RRFE` writer - Reject Remote Frames Extended"]
pub type RRFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRFS` reader - Reject Remote Frames Standard"]
pub type RRFS_R = crate::BitReader;
#[doc = "Field `RRFS` writer - Reject Remote Frames Standard"]
pub type RRFS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANFE` reader - Accept Non-matching Frames Extended"]
pub type ANFE_R = crate::FieldReader;
#[doc = "Field `ANFE` writer - Accept Non-matching Frames Extended"]
pub type ANFE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ANFS` reader - Accept Non-matching Frames Standard"]
pub type ANFS_R = crate::FieldReader;
#[doc = "Field `ANFS` writer - Accept Non-matching Frames Standard"]
pub type ANFS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `F1OM` reader - F1OM"]
pub type F1OM_R = crate::BitReader;
#[doc = "Field `F1OM` writer - F1OM"]
pub type F1OM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F0OM` reader - F0OM"]
pub type F0OM_R = crate::BitReader;
#[doc = "Field `F0OM` writer - F0OM"]
pub type F0OM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSS` reader - LSS"]
pub type LSS_R = crate::FieldReader;
#[doc = "Field `LSS` writer - LSS"]
pub type LSS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LSE` reader - LSE"]
pub type LSE_R = crate::FieldReader;
#[doc = "Field `LSE` writer - LSE"]
pub type LSE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Reject Remote Frames Extended"]
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard"]
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended"]
    #[inline(always)]
    pub fn anfe(&self) -> ANFE_R {
        ANFE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard"]
    #[inline(always)]
    pub fn anfs(&self) -> ANFS_R {
        ANFS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - F1OM"]
    #[inline(always)]
    pub fn f1om(&self) -> F1OM_R {
        F1OM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - F0OM"]
    #[inline(always)]
    pub fn f0om(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:20 - LSS"]
    #[inline(always)]
    pub fn lss(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - LSE"]
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reject Remote Frames Extended"]
    #[inline(always)]
    #[must_use]
    pub fn rrfe(&mut self) -> RRFE_W<FDCAN_RXGFCrs> {
        RRFE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard"]
    #[inline(always)]
    #[must_use]
    pub fn rrfs(&mut self) -> RRFS_W<FDCAN_RXGFCrs> {
        RRFS_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended"]
    #[inline(always)]
    #[must_use]
    pub fn anfe(&mut self) -> ANFE_W<FDCAN_RXGFCrs> {
        ANFE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard"]
    #[inline(always)]
    #[must_use]
    pub fn anfs(&mut self) -> ANFS_W<FDCAN_RXGFCrs> {
        ANFS_W::new(self, 4)
    }
    #[doc = "Bit 8 - F1OM"]
    #[inline(always)]
    #[must_use]
    pub fn f1om(&mut self) -> F1OM_W<FDCAN_RXGFCrs> {
        F1OM_W::new(self, 8)
    }
    #[doc = "Bit 9 - F0OM"]
    #[inline(always)]
    #[must_use]
    pub fn f0om(&mut self) -> F0OM_W<FDCAN_RXGFCrs> {
        F0OM_W::new(self, 9)
    }
    #[doc = "Bits 16:20 - LSS"]
    #[inline(always)]
    #[must_use]
    pub fn lss(&mut self) -> LSS_W<FDCAN_RXGFCrs> {
        LSS_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - LSE"]
    #[inline(always)]
    #[must_use]
    pub fn lse(&mut self) -> LSE_W<FDCAN_RXGFCrs> {
        LSE_W::new(self, 24)
    }
}
#[doc = "FDCAN Global Filter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxgfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxgfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_RXGFCrs;
impl crate::RegisterSpec for FDCAN_RXGFCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxgfc::R`](R) reader structure"]
impl crate::Readable for FDCAN_RXGFCrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rxgfc::W`](W) writer structure"]
impl crate::Writable for FDCAN_RXGFCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_RXGFC to value 0"]
impl crate::Resettable for FDCAN_RXGFCrs {
    const RESET_VALUE: u32 = 0;
}
