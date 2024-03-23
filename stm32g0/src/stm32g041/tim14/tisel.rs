#[doc = "Register `TISEL` reader"]
pub type R = crate::R<TISELrs>;
#[doc = "Register `TISEL` writer"]
pub type W = crate::W<TISELrs>;
#[doc = "Field `TISEL` reader - TI1\\[0\\]
to TI1\\[15\\]
input selection"]
pub type TISEL_R = crate::FieldReader;
#[doc = "Field `TISEL` writer - TI1\\[0\\]
to TI1\\[15\\]
input selection"]
pub type TISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TI1\\[0\\]
to TI1\\[15\\]
input selection"]
    #[inline(always)]
    pub fn tisel(&self) -> TISEL_R {
        TISEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI1\\[0\\]
to TI1\\[15\\]
input selection"]
    #[inline(always)]
    #[must_use]
    pub fn tisel(&mut self) -> TISEL_W<TISELrs> {
        TISEL_W::new(self, 0)
    }
}
#[doc = "TIM timer input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tisel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tisel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TISELrs;
impl crate::RegisterSpec for TISELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tisel::R`](R) reader structure"]
impl crate::Readable for TISELrs {}
#[doc = "`write(|w| ..)` method takes [`tisel::W`](W) writer structure"]
impl crate::Writable for TISELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TISEL to value 0"]
impl crate::Resettable for TISELrs {
    const RESET_VALUE: u32 = 0;
}
