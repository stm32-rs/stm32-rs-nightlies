#[doc = "Register `ARR` reader"]
pub type R = crate::R<ARRrs>;
#[doc = "Register `ARR` writer"]
pub type W = crate::W<ARRrs>;
#[doc = "Field `ARR_bit0` reader - ARR_bit0"]
pub type ARR_BIT0_R = crate::FieldReader<u16>;
#[doc = "Field `ARR_bit0` writer - ARR_bit0"]
pub type ARR_BIT0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ARR_bit0"]
    #[inline(always)]
    pub fn arr_bit0(&self) -> ARR_BIT0_R {
        ARR_BIT0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ARR_bit0"]
    #[inline(always)]
    #[must_use]
    pub fn arr_bit0(&mut self) -> ARR_BIT0_W<ARRrs> {
        ARR_BIT0_W::new(self, 0)
    }
}
#[doc = "auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARRrs;
impl crate::RegisterSpec for ARRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arr::R`](R) reader structure"]
impl crate::Readable for ARRrs {}
#[doc = "`write(|w| ..)` method takes [`arr::W`](W) writer structure"]
impl crate::Writable for ARRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARR to value 0xffff_ffff"]
impl crate::Resettable for ARRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
