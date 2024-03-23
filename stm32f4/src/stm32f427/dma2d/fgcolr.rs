#[doc = "Register `FGCOLR` reader"]
pub type R = crate::R<FGCOLRrs>;
#[doc = "Register `FGCOLR` writer"]
pub type W = crate::W<FGCOLRrs>;
#[doc = "Field `BLUE` reader - Blue Value These bits defines the blue value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
pub type BLUE_R = crate::FieldReader;
#[doc = "Field `BLUE` writer - Blue Value These bits defines the blue value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GREEN` reader - Green Value These bits defines the green value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
pub type GREEN_R = crate::FieldReader;
#[doc = "Field `GREEN` writer - Green Value These bits defines the green value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED` reader - Red Value These bits defines the red value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type RED_R = crate::FieldReader;
#[doc = "Field `RED` writer - Red Value These bits defines the red value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Blue Value These bits defines the blue value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Green Value These bits defines the green value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Red Value These bits defines the red value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Blue Value These bits defines the blue value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
    #[inline(always)]
    #[must_use]
    pub fn blue(&mut self) -> BLUE_W<FGCOLRrs> {
        BLUE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Green Value These bits defines the green value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
    #[inline(always)]
    #[must_use]
    pub fn green(&mut self) -> GREEN_W<FGCOLRrs> {
        GREEN_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Red Value These bits defines the red value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RED_W<FGCOLRrs> {
        RED_W::new(self, 16)
    }
}
#[doc = "DMA2D foreground color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgcolr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgcolr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FGCOLRrs;
impl crate::RegisterSpec for FGCOLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fgcolr::R`](R) reader structure"]
impl crate::Readable for FGCOLRrs {}
#[doc = "`write(|w| ..)` method takes [`fgcolr::W`](W) writer structure"]
impl crate::Writable for FGCOLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FGCOLR to value 0"]
impl crate::Resettable for FGCOLRrs {
    const RESET_VALUE: u32 = 0;
}
