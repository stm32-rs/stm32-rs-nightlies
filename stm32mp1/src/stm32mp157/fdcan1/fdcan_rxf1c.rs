#[doc = "Register `FDCAN_RXF1C` reader"]
pub type R = crate::R<FDCAN_RXF1Crs>;
#[doc = "Register `FDCAN_RXF1C` writer"]
pub type W = crate::W<FDCAN_RXF1Crs>;
#[doc = "Field `F1SA` reader - F1SA"]
pub type F1SA_R = crate::FieldReader<u16>;
#[doc = "Field `F1SA` writer - F1SA"]
pub type F1SA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `F1S` reader - F1S"]
pub type F1S_R = crate::FieldReader;
#[doc = "Field `F1S` writer - F1S"]
pub type F1S_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F1WM` reader - F1WM"]
pub type F1WM_R = crate::FieldReader;
#[doc = "Field `F1WM` writer - F1WM"]
pub type F1WM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F1OM` reader - F1OM"]
pub type F1OM_R = crate::BitReader;
#[doc = "Field `F1OM` writer - F1OM"]
pub type F1OM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - F1SA"]
    #[inline(always)]
    pub fn f1sa(&self) -> F1SA_R {
        F1SA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - F1S"]
    #[inline(always)]
    pub fn f1s(&self) -> F1S_R {
        F1S_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - F1WM"]
    #[inline(always)]
    pub fn f1wm(&self) -> F1WM_R {
        F1WM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - F1OM"]
    #[inline(always)]
    pub fn f1om(&self) -> F1OM_R {
        F1OM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - F1SA"]
    #[inline(always)]
    #[must_use]
    pub fn f1sa(&mut self) -> F1SA_W<FDCAN_RXF1Crs> {
        F1SA_W::new(self, 2)
    }
    #[doc = "Bits 16:22 - F1S"]
    #[inline(always)]
    #[must_use]
    pub fn f1s(&mut self) -> F1S_W<FDCAN_RXF1Crs> {
        F1S_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - F1WM"]
    #[inline(always)]
    #[must_use]
    pub fn f1wm(&mut self) -> F1WM_W<FDCAN_RXF1Crs> {
        F1WM_W::new(self, 24)
    }
    #[doc = "Bit 31 - F1OM"]
    #[inline(always)]
    #[must_use]
    pub fn f1om(&mut self) -> F1OM_W<FDCAN_RXF1Crs> {
        F1OM_W::new(self, 31)
    }
}
#[doc = "FDCAN Rx FIFO 1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf1c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf1c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_RXF1Crs;
impl crate::RegisterSpec for FDCAN_RXF1Crs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxf1c::R`](R) reader structure"]
impl crate::Readable for FDCAN_RXF1Crs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rxf1c::W`](W) writer structure"]
impl crate::Writable for FDCAN_RXF1Crs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_RXF1C to value 0"]
impl crate::Resettable for FDCAN_RXF1Crs {
    const RESET_VALUE: u32 = 0;
}
