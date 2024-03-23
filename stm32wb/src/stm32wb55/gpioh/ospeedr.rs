#[doc = "Register `OSPEEDR` reader"]
pub type R = crate::R<OSPEEDRrs>;
#[doc = "Register `OSPEEDR` writer"]
pub type W = crate::W<OSPEEDRrs>;
#[doc = "Field `OSPEEDR0` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR0_R = crate::FieldReader;
#[doc = "Field `OSPEEDR0` writer - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEEDR1` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR1_R = crate::FieldReader;
#[doc = "Field `OSPEEDR1` writer - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEEDR3` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR3_R = crate::FieldReader;
#[doc = "Field `OSPEEDR3` writer - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr0(&self) -> OSPEEDR0_R {
        OSPEEDR0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr1(&self) -> OSPEEDR1_R {
        OSPEEDR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr3(&self) -> OSPEEDR3_R {
        OSPEEDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr0(&mut self) -> OSPEEDR0_W<OSPEEDRrs> {
        OSPEEDR0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr1(&mut self) -> OSPEEDR1_W<OSPEEDRrs> {
        OSPEEDR1_W::new(self, 2)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr3(&mut self) -> OSPEEDR3_W<OSPEEDRrs> {
        OSPEEDR3_W::new(self, 6)
    }
}
#[doc = "GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospeedr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospeedr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSPEEDRrs;
impl crate::RegisterSpec for OSPEEDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospeedr::R`](R) reader structure"]
impl crate::Readable for OSPEEDRrs {}
#[doc = "`write(|w| ..)` method takes [`ospeedr::W`](W) writer structure"]
impl crate::Writable for OSPEEDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPEEDR to value 0"]
impl crate::Resettable for OSPEEDRrs {
    const RESET_VALUE: u32 = 0;
}
