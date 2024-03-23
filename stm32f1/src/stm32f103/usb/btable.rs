#[doc = "Register `BTABLE` reader"]
pub type R = crate::R<BTABLErs>;
#[doc = "Register `BTABLE` writer"]
pub type W = crate::W<BTABLErs>;
#[doc = "Field `BTABLE` reader - Buffer table"]
pub type BTABLE_R = crate::FieldReader<u16>;
#[doc = "Field `BTABLE` writer - Buffer table"]
pub type BTABLE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 3:15 - Buffer table"]
    #[inline(always)]
    pub fn btable(&self) -> BTABLE_R {
        BTABLE_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Buffer table"]
    #[inline(always)]
    #[must_use]
    pub fn btable(&mut self) -> BTABLE_W<BTABLErs> {
        BTABLE_W::new(self, 3)
    }
}
#[doc = "Buffer table address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTABLErs;
impl crate::RegisterSpec for BTABLErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btable::R`](R) reader structure"]
impl crate::Readable for BTABLErs {}
#[doc = "`write(|w| ..)` method takes [`btable::W`](W) writer structure"]
impl crate::Writable for BTABLErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BTABLE to value 0"]
impl crate::Resettable for BTABLErs {
    const RESET_VALUE: u32 = 0;
}
