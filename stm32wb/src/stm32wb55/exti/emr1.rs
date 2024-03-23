#[doc = "Register `EMR1` reader"]
pub type R = crate::R<EMR1rs>;
#[doc = "Register `EMR1` writer"]
pub type W = crate::W<EMR1rs>;
#[doc = "Field `EM0_15` reader - CPU(m) Wakeup with event generation Mask on Event input"]
pub type EM0_15_R = crate::FieldReader<u16>;
#[doc = "Field `EM0_15` writer - CPU(m) Wakeup with event generation Mask on Event input"]
pub type EM0_15_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EM17_21` reader - CPU(m) Wakeup with event generation Mask on Event input"]
pub type EM17_21_R = crate::FieldReader;
#[doc = "Field `EM17_21` writer - CPU(m) Wakeup with event generation Mask on Event input"]
pub type EM17_21_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:15 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em0_15(&self) -> EM0_15_R {
        EM0_15_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 17:21 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em17_21(&self) -> EM17_21_R {
        EM17_21_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn em0_15(&mut self) -> EM0_15_W<EMR1rs> {
        EM0_15_W::new(self, 0)
    }
    #[doc = "Bits 17:21 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn em17_21(&mut self) -> EM17_21_W<EMR1rs> {
        EM17_21_W::new(self, 17)
    }
}
#[doc = "CPUm wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMR1rs;
impl crate::RegisterSpec for EMR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr1::R`](R) reader structure"]
impl crate::Readable for EMR1rs {}
#[doc = "`write(|w| ..)` method takes [`emr1::W`](W) writer structure"]
impl crate::Writable for EMR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMR1 to value 0"]
impl crate::Resettable for EMR1rs {
    const RESET_VALUE: u32 = 0;
}
