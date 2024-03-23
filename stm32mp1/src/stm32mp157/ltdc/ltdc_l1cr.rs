#[doc = "Register `LTDC_L1CR` reader"]
pub type R = crate::R<LTDC_L1CRrs>;
#[doc = "Register `LTDC_L1CR` writer"]
pub type W = crate::W<LTDC_L1CRrs>;
#[doc = "Field `LEN` reader - LEN"]
pub type LEN_R = crate::BitReader;
#[doc = "Field `LEN` writer - LEN"]
pub type LEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLKEN` reader - COLKEN"]
pub type COLKEN_R = crate::BitReader;
#[doc = "Field `COLKEN` writer - COLKEN"]
pub type COLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLUTEN` reader - CLUTEN"]
pub type CLUTEN_R = crate::BitReader;
#[doc = "Field `CLUTEN` writer - CLUTEN"]
pub type CLUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LEN"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - COLKEN"]
    #[inline(always)]
    pub fn colken(&self) -> COLKEN_R {
        COLKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - CLUTEN"]
    #[inline(always)]
    pub fn cluten(&self) -> CLUTEN_R {
        CLUTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LEN"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<LTDC_L1CRrs> {
        LEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - COLKEN"]
    #[inline(always)]
    #[must_use]
    pub fn colken(&mut self) -> COLKEN_W<LTDC_L1CRrs> {
        COLKEN_W::new(self, 1)
    }
    #[doc = "Bit 4 - CLUTEN"]
    #[inline(always)]
    #[must_use]
    pub fn cluten(&mut self) -> CLUTEN_W<LTDC_L1CRrs> {
        CLUTEN_W::new(self, 4)
    }
}
#[doc = "LTDC layer 1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_l1cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_l1cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_L1CRrs;
impl crate::RegisterSpec for LTDC_L1CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_l1cr::R`](R) reader structure"]
impl crate::Readable for LTDC_L1CRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_l1cr::W`](W) writer structure"]
impl crate::Writable for LTDC_L1CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_L1CR to value 0"]
impl crate::Resettable for LTDC_L1CRrs {
    const RESET_VALUE: u32 = 0;
}
