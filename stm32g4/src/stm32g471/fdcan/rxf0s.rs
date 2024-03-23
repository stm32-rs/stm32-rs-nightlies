#[doc = "Register `RXF0S` reader"]
pub type R = crate::R<RXF0Srs>;
#[doc = "Register `RXF0S` writer"]
pub type W = crate::W<RXF0Srs>;
#[doc = "Field `F0FL` reader - F0FL"]
pub type F0FL_R = crate::FieldReader;
#[doc = "Field `F0FL` writer - F0FL"]
pub type F0FL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F0GI` reader - F0GI"]
pub type F0GI_R = crate::FieldReader;
#[doc = "Field `F0GI` writer - F0GI"]
pub type F0GI_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `F0PI` reader - F0PI"]
pub type F0PI_R = crate::FieldReader;
#[doc = "Field `F0PI` writer - F0PI"]
pub type F0PI_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `F0F` reader - F0F"]
pub type F0F_R = crate::BitReader;
#[doc = "Field `F0F` writer - F0F"]
pub type F0F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0L` reader - RF0L"]
pub type RF0L_R = crate::BitReader;
#[doc = "Field `RF0L` writer - RF0L"]
pub type RF0L_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - F0FL"]
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - F0GI"]
    #[inline(always)]
    pub fn f0gi(&self) -> F0GI_R {
        F0GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - F0PI"]
    #[inline(always)]
    pub fn f0pi(&self) -> F0PI_R {
        F0PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - F0F"]
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RF0L"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - F0FL"]
    #[inline(always)]
    #[must_use]
    pub fn f0fl(&mut self) -> F0FL_W<RXF0Srs> {
        F0FL_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - F0GI"]
    #[inline(always)]
    #[must_use]
    pub fn f0gi(&mut self) -> F0GI_W<RXF0Srs> {
        F0GI_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - F0PI"]
    #[inline(always)]
    #[must_use]
    pub fn f0pi(&mut self) -> F0PI_W<RXF0Srs> {
        F0PI_W::new(self, 16)
    }
    #[doc = "Bit 24 - F0F"]
    #[inline(always)]
    #[must_use]
    pub fn f0f(&mut self) -> F0F_W<RXF0Srs> {
        F0F_W::new(self, 24)
    }
    #[doc = "Bit 25 - RF0L"]
    #[inline(always)]
    #[must_use]
    pub fn rf0l(&mut self) -> RF0L_W<RXF0Srs> {
        RF0L_W::new(self, 25)
    }
}
#[doc = "FDCAN Rx FIFO 0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf0s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXF0Srs;
impl crate::RegisterSpec for RXF0Srs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf0s::R`](R) reader structure"]
impl crate::Readable for RXF0Srs {}
#[doc = "`write(|w| ..)` method takes [`rxf0s::W`](W) writer structure"]
impl crate::Writable for RXF0Srs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXF0S to value 0"]
impl crate::Resettable for RXF0Srs {
    const RESET_VALUE: u32 = 0;
}
