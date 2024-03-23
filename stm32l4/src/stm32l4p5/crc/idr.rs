#[doc = "Register `IDR` reader"]
pub type R = crate::R<IDRrs>;
#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDRrs>;
#[doc = "Field `IDR` reader - General-purpose 8-bit data register bits"]
pub type IDR_R = crate::FieldReader;
#[doc = "Field `IDR` writer - General-purpose 8-bit data register bits"]
pub type IDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - General-purpose 8-bit data register bits"]
    #[inline(always)]
    pub fn idr(&self) -> IDR_R {
        IDR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - General-purpose 8-bit data register bits"]
    #[inline(always)]
    #[must_use]
    pub fn idr(&mut self) -> IDR_W<IDRrs> {
        IDR_W::new(self, 0)
    }
}
#[doc = "Independent data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr::R`](R) reader structure"]
impl crate::Readable for IDRrs {}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDRrs {
    const RESET_VALUE: u32 = 0;
}
