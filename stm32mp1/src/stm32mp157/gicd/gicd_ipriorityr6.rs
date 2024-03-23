#[doc = "Register `GICD_IPRIORITYR6` reader"]
pub type R = crate::R<GICD_IPRIORITYR6rs>;
#[doc = "Register `GICD_IPRIORITYR6` writer"]
pub type W = crate::W<GICD_IPRIORITYR6rs>;
#[doc = "Field `PRIORITY0` reader - PRIORITY0"]
pub type PRIORITY0_R = crate::FieldReader;
#[doc = "Field `PRIORITY0` writer - PRIORITY0"]
pub type PRIORITY0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRIORITY1` reader - PRIORITY1"]
pub type PRIORITY1_R = crate::FieldReader;
#[doc = "Field `PRIORITY1` writer - PRIORITY1"]
pub type PRIORITY1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRIORITY2` reader - PRIORITY2"]
pub type PRIORITY2_R = crate::FieldReader;
#[doc = "Field `PRIORITY2` writer - PRIORITY2"]
pub type PRIORITY2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRIORITY3` reader - PRIORITY3"]
pub type PRIORITY3_R = crate::FieldReader;
#[doc = "Field `PRIORITY3` writer - PRIORITY3"]
pub type PRIORITY3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 3:7 - PRIORITY0"]
    #[inline(always)]
    pub fn priority0(&self) -> PRIORITY0_R {
        PRIORITY0_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - PRIORITY1"]
    #[inline(always)]
    pub fn priority1(&self) -> PRIORITY1_R {
        PRIORITY1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - PRIORITY2"]
    #[inline(always)]
    pub fn priority2(&self) -> PRIORITY2_R {
        PRIORITY2_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - PRIORITY3"]
    #[inline(always)]
    pub fn priority3(&self) -> PRIORITY3_R {
        PRIORITY3_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - PRIORITY0"]
    #[inline(always)]
    #[must_use]
    pub fn priority0(&mut self) -> PRIORITY0_W<GICD_IPRIORITYR6rs> {
        PRIORITY0_W::new(self, 3)
    }
    #[doc = "Bits 11:15 - PRIORITY1"]
    #[inline(always)]
    #[must_use]
    pub fn priority1(&mut self) -> PRIORITY1_W<GICD_IPRIORITYR6rs> {
        PRIORITY1_W::new(self, 11)
    }
    #[doc = "Bits 19:23 - PRIORITY2"]
    #[inline(always)]
    #[must_use]
    pub fn priority2(&mut self) -> PRIORITY2_W<GICD_IPRIORITYR6rs> {
        PRIORITY2_W::new(self, 19)
    }
    #[doc = "Bits 27:31 - PRIORITY3"]
    #[inline(always)]
    #[must_use]
    pub fn priority3(&mut self) -> PRIORITY3_W<GICD_IPRIORITYR6rs> {
        PRIORITY3_W::new(self, 27)
    }
}
#[doc = "GICD interrupt priority register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR6rs;
impl crate::RegisterSpec for GICD_IPRIORITYR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr6::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR6rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr6::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR6 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR6rs {
    const RESET_VALUE: u32 = 0;
}
