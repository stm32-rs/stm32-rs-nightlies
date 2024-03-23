#[doc = "Register `FLT0AWHTR` reader"]
pub type R = crate::R<FLT0AWHTRrs>;
#[doc = "Register `FLT0AWHTR` writer"]
pub type W = crate::W<FLT0AWHTRrs>;
#[doc = "Field `BKAWH` reader - Break signal assignment to analog watchdog high threshold event"]
pub type BKAWH_R = crate::FieldReader;
#[doc = "Field `BKAWH` writer - Break signal assignment to analog watchdog high threshold event"]
pub type BKAWH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AWHT` reader - Analog watchdog high threshold"]
pub type AWHT_R = crate::FieldReader<u32>;
#[doc = "Field `AWHT` writer - Analog watchdog high threshold"]
pub type AWHT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event"]
    #[inline(always)]
    pub fn bkawh(&self) -> BKAWH_R {
        BKAWH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - Analog watchdog high threshold"]
    #[inline(always)]
    pub fn awht(&self) -> AWHT_R {
        AWHT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event"]
    #[inline(always)]
    #[must_use]
    pub fn bkawh(&mut self) -> BKAWH_W<FLT0AWHTRrs> {
        BKAWH_W::new(self, 0)
    }
    #[doc = "Bits 8:31 - Analog watchdog high threshold"]
    #[inline(always)]
    #[must_use]
    pub fn awht(&mut self) -> AWHT_W<FLT0AWHTRrs> {
        AWHT_W::new(self, 8)
    }
}
#[doc = "analog watchdog high threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flt0awhtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flt0awhtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLT0AWHTRrs;
impl crate::RegisterSpec for FLT0AWHTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flt0awhtr::R`](R) reader structure"]
impl crate::Readable for FLT0AWHTRrs {}
#[doc = "`write(|w| ..)` method takes [`flt0awhtr::W`](W) writer structure"]
impl crate::Writable for FLT0AWHTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLT0AWHTR to value 0"]
impl crate::Resettable for FLT0AWHTRrs {
    const RESET_VALUE: u32 = 0;
}
