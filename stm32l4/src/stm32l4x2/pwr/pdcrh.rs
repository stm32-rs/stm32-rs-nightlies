#[doc = "Register `PDCRH` reader"]
pub type R = crate::R<PDCRHrs>;
#[doc = "Register `PDCRH` writer"]
pub type W = crate::W<PDCRHrs>;
#[doc = "Field `PD0` reader - Port H pull-down bit y (y=0..1)"]
pub type PD0_R = crate::BitReader;
#[doc = "Field `PD0` writer - Port H pull-down bit y (y=0..1)"]
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1` reader - Port H pull-down bit y (y=0..1)"]
pub type PD1_R = crate::BitReader;
#[doc = "Field `PD1` writer - Port H pull-down bit y (y=0..1)"]
pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port H pull-down bit y (y=0..1)"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port H pull-down bit y (y=0..1)"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port H pull-down bit y (y=0..1)"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<PDCRHrs> {
        PD0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port H pull-down bit y (y=0..1)"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<PDCRHrs> {
        PD1_W::new(self, 1)
    }
}
#[doc = "Power Port H pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDCRHrs;
impl crate::RegisterSpec for PDCRHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdcrh::R`](R) reader structure"]
impl crate::Readable for PDCRHrs {}
#[doc = "`write(|w| ..)` method takes [`pdcrh::W`](W) writer structure"]
impl crate::Writable for PDCRHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDCRH to value 0"]
impl crate::Resettable for PDCRHrs {
    const RESET_VALUE: u32 = 0;
}
