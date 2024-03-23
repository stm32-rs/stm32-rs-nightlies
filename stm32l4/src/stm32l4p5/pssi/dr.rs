#[doc = "Register `DR` reader"]
pub type R = crate::R<DRrs>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DRrs>;
#[doc = "Field `Byte0` reader - "]
pub type BYTE0_R = crate::FieldReader;
#[doc = "Field `Byte0` writer - "]
pub type BYTE0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Byte1` reader - "]
pub type BYTE1_R = crate::FieldReader;
#[doc = "Field `Byte1` writer - "]
pub type BYTE1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Byte2` reader - "]
pub type BYTE2_R = crate::FieldReader;
#[doc = "Field `Byte2` writer - "]
pub type BYTE2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Byte3` reader - "]
pub type BYTE3_R = crate::FieldReader;
#[doc = "Field `Byte3` writer - "]
pub type BYTE3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn byte0(&self) -> BYTE0_R {
        BYTE0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn byte1(&self) -> BYTE1_R {
        BYTE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn byte2(&self) -> BYTE2_R {
        BYTE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn byte3(&self) -> BYTE3_R {
        BYTE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn byte0(&mut self) -> BYTE0_W<DRrs> {
        BYTE0_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn byte1(&mut self) -> BYTE1_W<DRrs> {
        BYTE1_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn byte2(&mut self) -> BYTE2_W<DRrs> {
        BYTE2_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn byte3(&mut self) -> BYTE3_W<DRrs> {
        BYTE3_W::new(self, 24)
    }
}
#[doc = "PSSI data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DRrs {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DRrs {
    const RESET_VALUE: u32 = 0;
}
