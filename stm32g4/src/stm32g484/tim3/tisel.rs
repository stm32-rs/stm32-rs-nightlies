#[doc = "Register `TISEL` reader"]
pub type R = crate::R<TISELrs>;
#[doc = "Register `TISEL` writer"]
pub type W = crate::W<TISELrs>;
#[doc = "Field `TI1SEL` reader - TI1\\[0\\]
to TI1\\[15\\]
input selection"]
pub type TI1SEL_R = crate::FieldReader;
#[doc = "Field `TI1SEL` writer - TI1\\[0\\]
to TI1\\[15\\]
input selection"]
pub type TI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TI2SEL` reader - TI2\\[0\\]
to TI2\\[15\\]
input selection"]
pub type TI2SEL_R = crate::FieldReader;
#[doc = "Field `TI2SEL` writer - TI2\\[0\\]
to TI2\\[15\\]
input selection"]
pub type TI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TI3SEL` reader - TI3\\[0\\]
to TI3\\[15\\]
input selection"]
pub type TI3SEL_R = crate::FieldReader;
#[doc = "Field `TI3SEL` writer - TI3\\[0\\]
to TI3\\[15\\]
input selection"]
pub type TI3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TI4SEL` reader - TI4\\[0\\]
to TI4\\[15\\]
input selection"]
pub type TI4SEL_R = crate::FieldReader;
#[doc = "Field `TI4SEL` writer - TI4\\[0\\]
to TI4\\[15\\]
input selection"]
pub type TI4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TI1\\[0\\]
to TI1\\[15\\]
input selection"]
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TI2\\[0\\]
to TI2\\[15\\]
input selection"]
    #[inline(always)]
    pub fn ti2sel(&self) -> TI2SEL_R {
        TI2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - TI3\\[0\\]
to TI3\\[15\\]
input selection"]
    #[inline(always)]
    pub fn ti3sel(&self) -> TI3SEL_R {
        TI3SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - TI4\\[0\\]
to TI4\\[15\\]
input selection"]
    #[inline(always)]
    pub fn ti4sel(&self) -> TI4SEL_R {
        TI4SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI1\\[0\\]
to TI1\\[15\\]
input selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti1sel(&mut self) -> TI1SEL_W<TISELrs> {
        TI1SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - TI2\\[0\\]
to TI2\\[15\\]
input selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti2sel(&mut self) -> TI2SEL_W<TISELrs> {
        TI2SEL_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - TI3\\[0\\]
to TI3\\[15\\]
input selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti3sel(&mut self) -> TI3SEL_W<TISELrs> {
        TI3SEL_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - TI4\\[0\\]
to TI4\\[15\\]
input selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti4sel(&mut self) -> TI4SEL_W<TISELrs> {
        TI4SEL_W::new(self, 24)
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
