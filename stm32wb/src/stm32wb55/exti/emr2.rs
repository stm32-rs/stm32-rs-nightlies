#[doc = "Register `EMR2` reader"]
pub type R = crate::R<EMR2rs>;
#[doc = "Register `EMR2` writer"]
pub type W = crate::W<EMR2rs>;
#[doc = "Field `EM` reader - CPU(m) Wakeup with event generation Mask on Event input"]
pub type EM_R = crate::FieldReader;
#[doc = "Field `EM` writer - CPU(m) Wakeup with event generation Mask on Event input"]
pub type EM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 8:9 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em(&self) -> EM_R {
        EM_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn em(&mut self) -> EM_W<EMR2rs> {
        EM_W::new(self, 8)
    }
}
#[doc = "CPUm wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMR2rs;
impl crate::RegisterSpec for EMR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr2::R`](R) reader structure"]
impl crate::Readable for EMR2rs {}
#[doc = "`write(|w| ..)` method takes [`emr2::W`](W) writer structure"]
impl crate::Writable for EMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMR2 to value 0"]
impl crate::Resettable for EMR2rs {
    const RESET_VALUE: u32 = 0;
}
