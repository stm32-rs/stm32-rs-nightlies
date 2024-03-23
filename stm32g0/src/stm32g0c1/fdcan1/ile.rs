#[doc = "Register `ILE` reader"]
pub type R = crate::R<ILErs>;
#[doc = "Register `ILE` writer"]
pub type W = crate::W<ILErs>;
#[doc = "Field `EINT0` reader - Enable interrupt line 0"]
pub type EINT0_R = crate::BitReader;
#[doc = "Field `EINT0` writer - Enable interrupt line 0"]
pub type EINT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EINT1` reader - Enable interrupt line 1"]
pub type EINT1_R = crate::BitReader;
#[doc = "Field `EINT1` writer - Enable interrupt line 1"]
pub type EINT1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable interrupt line 0"]
    #[inline(always)]
    pub fn eint0(&self) -> EINT0_R {
        EINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable interrupt line 1"]
    #[inline(always)]
    pub fn eint1(&self) -> EINT1_R {
        EINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt line 0"]
    #[inline(always)]
    #[must_use]
    pub fn eint0(&mut self) -> EINT0_W<ILErs> {
        EINT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable interrupt line 1"]
    #[inline(always)]
    #[must_use]
    pub fn eint1(&mut self) -> EINT1_W<ILErs> {
        EINT1_W::new(self, 1)
    }
}
#[doc = "FDCAN interrupt line enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ile::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ile::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ILErs;
impl crate::RegisterSpec for ILErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ile::R`](R) reader structure"]
impl crate::Readable for ILErs {}
#[doc = "`write(|w| ..)` method takes [`ile::W`](W) writer structure"]
impl crate::Writable for ILErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ILE to value 0"]
impl crate::Resettable for ILErs {
    const RESET_VALUE: u32 = 0;
}
