#[doc = "Register `CALR` reader"]
pub type R = crate::R<CALRrs>;
#[doc = "Register `CALR` writer"]
pub type W = crate::W<CALRrs>;
#[doc = "Field `CALM` reader - CALM"]
pub type CALM_R = crate::FieldReader<u16>;
#[doc = "Field `CALM` writer - CALM"]
pub type CALM_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CALW16` reader - CALW16"]
pub type CALW16_R = crate::BitReader;
#[doc = "Field `CALW16` writer - CALW16"]
pub type CALW16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALW8` reader - CALW8"]
pub type CALW8_R = crate::BitReader;
#[doc = "Field `CALW8` writer - CALW8"]
pub type CALW8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALP` reader - CALP"]
pub type CALP_R = crate::BitReader;
#[doc = "Field `CALP` writer - CALP"]
pub type CALP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - CALM"]
    #[inline(always)]
    pub fn calm(&self) -> CALM_R {
        CALM_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - CALW16"]
    #[inline(always)]
    pub fn calw16(&self) -> CALW16_R {
        CALW16_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CALW8"]
    #[inline(always)]
    pub fn calw8(&self) -> CALW8_R {
        CALW8_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CALP"]
    #[inline(always)]
    pub fn calp(&self) -> CALP_R {
        CALP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - CALM"]
    #[inline(always)]
    #[must_use]
    pub fn calm(&mut self) -> CALM_W<CALRrs> {
        CALM_W::new(self, 0)
    }
    #[doc = "Bit 13 - CALW16"]
    #[inline(always)]
    #[must_use]
    pub fn calw16(&mut self) -> CALW16_W<CALRrs> {
        CALW16_W::new(self, 13)
    }
    #[doc = "Bit 14 - CALW8"]
    #[inline(always)]
    #[must_use]
    pub fn calw8(&mut self) -> CALW8_W<CALRrs> {
        CALW8_W::new(self, 14)
    }
    #[doc = "Bit 15 - CALP"]
    #[inline(always)]
    #[must_use]
    pub fn calp(&mut self) -> CALP_W<CALRrs> {
        CALP_W::new(self, 15)
    }
}
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALRrs;
impl crate::RegisterSpec for CALRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calr::R`](R) reader structure"]
impl crate::Readable for CALRrs {}
#[doc = "`write(|w| ..)` method takes [`calr::W`](W) writer structure"]
impl crate::Writable for CALRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CALR to value 0"]
impl crate::Resettable for CALRrs {
    const RESET_VALUE: u32 = 0;
}
