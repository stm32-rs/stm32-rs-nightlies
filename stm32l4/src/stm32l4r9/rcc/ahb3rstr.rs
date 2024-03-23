#[doc = "Register `AHB3RSTR` reader"]
pub type R = crate::R<AHB3RSTRrs>;
#[doc = "Register `AHB3RSTR` writer"]
pub type W = crate::W<AHB3RSTRrs>;
#[doc = "Field `FMCRST` reader - Flexible memory controller reset"]
pub type FMCRST_R = crate::BitReader;
#[doc = "Field `FMCRST` writer - Flexible memory controller reset"]
pub type FMCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPI2RST` reader - OctOSPI2 memory interface reset"]
pub type OSPI2RST_R = crate::BitReader;
#[doc = "Field `OSPI2RST` writer - OctOSPI2 memory interface reset"]
pub type OSPI2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flexible memory controller reset"]
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - OctOSPI2 memory interface reset"]
    #[inline(always)]
    pub fn ospi2rst(&self) -> OSPI2RST_R {
        OSPI2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn fmcrst(&mut self) -> FMCRST_W<AHB3RSTRrs> {
        FMCRST_W::new(self, 0)
    }
    #[doc = "Bit 9 - OctOSPI2 memory interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn ospi2rst(&mut self) -> OSPI2RST_W<AHB3RSTRrs> {
        OSPI2RST_W::new(self, 9)
    }
}
#[doc = "AHB3 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB3RSTRrs;
impl crate::RegisterSpec for AHB3RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3rstr::R`](R) reader structure"]
impl crate::Readable for AHB3RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb3rstr::W`](W) writer structure"]
impl crate::Writable for AHB3RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB3RSTR to value 0"]
impl crate::Resettable for AHB3RSTRrs {
    const RESET_VALUE: u32 = 0;
}
