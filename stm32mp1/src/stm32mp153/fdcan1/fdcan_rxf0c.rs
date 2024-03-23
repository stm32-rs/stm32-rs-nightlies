#[doc = "Register `FDCAN_RXF0C` reader"]
pub type R = crate::R<FDCAN_RXF0Crs>;
#[doc = "Register `FDCAN_RXF0C` writer"]
pub type W = crate::W<FDCAN_RXF0Crs>;
#[doc = "Field `F0SA` reader - F0SA"]
pub type F0SA_R = crate::FieldReader<u16>;
#[doc = "Field `F0SA` writer - F0SA"]
pub type F0SA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `F0S` reader - F0S"]
pub type F0S_R = crate::FieldReader;
#[doc = "Field `F0S` writer - F0S"]
pub type F0S_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F0WM` reader - F0WM"]
pub type F0WM_R = crate::FieldReader;
#[doc = "Field `F0WM` writer - F0WM"]
pub type F0WM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F0OM` reader - F0OM"]
pub type F0OM_R = crate::BitReader;
#[doc = "Field `F0OM` writer - F0OM"]
pub type F0OM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - F0SA"]
    #[inline(always)]
    pub fn f0sa(&self) -> F0SA_R {
        F0SA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - F0S"]
    #[inline(always)]
    pub fn f0s(&self) -> F0S_R {
        F0S_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - F0WM"]
    #[inline(always)]
    pub fn f0wm(&self) -> F0WM_R {
        F0WM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - F0OM"]
    #[inline(always)]
    pub fn f0om(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - F0SA"]
    #[inline(always)]
    #[must_use]
    pub fn f0sa(&mut self) -> F0SA_W<FDCAN_RXF0Crs> {
        F0SA_W::new(self, 2)
    }
    #[doc = "Bits 16:22 - F0S"]
    #[inline(always)]
    #[must_use]
    pub fn f0s(&mut self) -> F0S_W<FDCAN_RXF0Crs> {
        F0S_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - F0WM"]
    #[inline(always)]
    #[must_use]
    pub fn f0wm(&mut self) -> F0WM_W<FDCAN_RXF0Crs> {
        F0WM_W::new(self, 24)
    }
    #[doc = "Bit 31 - F0OM"]
    #[inline(always)]
    #[must_use]
    pub fn f0om(&mut self) -> F0OM_W<FDCAN_RXF0Crs> {
        F0OM_W::new(self, 31)
    }
}
#[doc = "FDCAN Rx FIFO 0 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf0c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf0c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_RXF0Crs;
impl crate::RegisterSpec for FDCAN_RXF0Crs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxf0c::R`](R) reader structure"]
impl crate::Readable for FDCAN_RXF0Crs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rxf0c::W`](W) writer structure"]
impl crate::Writable for FDCAN_RXF0Crs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_RXF0C to value 0"]
impl crate::Resettable for FDCAN_RXF0Crs {
    const RESET_VALUE: u32 = 0;
}
