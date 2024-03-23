#[doc = "Register `LCOLCR` reader"]
pub type R = crate::R<LCOLCRrs>;
#[doc = "Register `LCOLCR` writer"]
pub type W = crate::W<LCOLCRrs>;
#[doc = "Field `COLC` reader - Color coding"]
pub type COLC_R = crate::FieldReader;
#[doc = "Field `COLC` writer - Color coding"]
pub type COLC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPE` reader - Loosely packet enable"]
pub type LPE_R = crate::BitReader;
#[doc = "Field `LPE` writer - Loosely packet enable"]
pub type LPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Color coding"]
    #[inline(always)]
    pub fn colc(&self) -> COLC_R {
        COLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Loosely packet enable"]
    #[inline(always)]
    pub fn lpe(&self) -> LPE_R {
        LPE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Color coding"]
    #[inline(always)]
    #[must_use]
    pub fn colc(&mut self) -> COLC_W<LCOLCRrs> {
        COLC_W::new(self, 0)
    }
    #[doc = "Bit 8 - Loosely packet enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpe(&mut self) -> LPE_W<LCOLCRrs> {
        LPE_W::new(self, 8)
    }
}
#[doc = "DSI Host LTDC color coding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcolcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcolcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCOLCRrs;
impl crate::RegisterSpec for LCOLCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcolcr::R`](R) reader structure"]
impl crate::Readable for LCOLCRrs {}
#[doc = "`write(|w| ..)` method takes [`lcolcr::W`](W) writer structure"]
impl crate::Writable for LCOLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCOLCR to value 0"]
impl crate::Resettable for LCOLCRrs {
    const RESET_VALUE: u32 = 0;
}
