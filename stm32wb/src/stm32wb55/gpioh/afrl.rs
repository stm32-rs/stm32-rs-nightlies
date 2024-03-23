#[doc = "Register `AFRL` reader"]
pub type R = crate::R<AFRLrs>;
#[doc = "Register `AFRL` writer"]
pub type W = crate::W<AFRLrs>;
#[doc = "Field `AFSEL0` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFSEL0_R = crate::FieldReader;
#[doc = "Field `AFSEL0` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL1` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFSEL1_R = crate::FieldReader;
#[doc = "Field `AFSEL1` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL3` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFSEL3_R = crate::FieldReader;
#[doc = "Field `AFSEL3` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel0(&self) -> AFSEL0_R {
        AFSEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel1(&self) -> AFSEL1_R {
        AFSEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel3(&self) -> AFSEL3_R {
        AFSEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel0(&mut self) -> AFSEL0_W<AFRLrs> {
        AFSEL0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel1(&mut self) -> AFSEL1_W<AFRLrs> {
        AFSEL1_W::new(self, 4)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel3(&mut self) -> AFSEL3_W<AFRLrs> {
        AFSEL3_W::new(self, 12)
    }
}
#[doc = "GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFRLrs;
impl crate::RegisterSpec for AFRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrl::R`](R) reader structure"]
impl crate::Readable for AFRLrs {}
#[doc = "`write(|w| ..)` method takes [`afrl::W`](W) writer structure"]
impl crate::Writable for AFRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::Resettable for AFRLrs {
    const RESET_VALUE: u32 = 0;
}
