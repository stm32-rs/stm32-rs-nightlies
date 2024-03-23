#[doc = "Register `OR` reader"]
pub type R = crate::R<ORrs>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<ORrs>;
#[doc = "Field `SWP_TBYP` reader - SWP transceiver bypass"]
pub type SWP_TBYP_R = crate::BitReader;
#[doc = "Field `SWP_TBYP` writer - SWP transceiver bypass"]
pub type SWP_TBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWP_CLASS` reader - SWP class selection"]
pub type SWP_CLASS_R = crate::BitReader;
#[doc = "Field `SWP_CLASS` writer - SWP class selection"]
pub type SWP_CLASS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SWP transceiver bypass"]
    #[inline(always)]
    pub fn swp_tbyp(&self) -> SWP_TBYP_R {
        SWP_TBYP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SWP class selection"]
    #[inline(always)]
    pub fn swp_class(&self) -> SWP_CLASS_R {
        SWP_CLASS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SWP transceiver bypass"]
    #[inline(always)]
    #[must_use]
    pub fn swp_tbyp(&mut self) -> SWP_TBYP_W<ORrs> {
        SWP_TBYP_W::new(self, 0)
    }
    #[doc = "Bit 1 - SWP class selection"]
    #[inline(always)]
    #[must_use]
    pub fn swp_class(&mut self) -> SWP_CLASS_W<ORrs> {
        SWP_CLASS_W::new(self, 1)
    }
}
#[doc = "SWPMI Option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for ORrs {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for ORrs {
    const RESET_VALUE: u32 = 0;
}
