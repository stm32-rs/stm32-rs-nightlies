///Register `CCMR2_Input` reader
pub type R = crate::R<CCMR2_INPUTrs>;
///Register `CCMR2_Input` writer
pub type W = crate::W<CCMR2_INPUTrs>;
///Field `CC3S` reader - Capture/compare 3 selection
pub type CC3S_R = crate::FieldReader;
///Field `CC3S` writer - Capture/compare 3 selection
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC3PSC` reader - Input capture 3 prescaler
pub type IC3PSC_R = crate::FieldReader;
///Field `IC3PSC` writer - Input capture 3 prescaler
pub type IC3PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC3F` reader - Input capture 3 filter
pub type IC3F_R = crate::FieldReader;
///Field `IC3F` writer - Input capture 3 filter
pub type IC3F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CC4S` reader - Capture/Compare 4 selection
pub type CC4S_R = crate::FieldReader;
///Field `CC4S` writer - Capture/Compare 4 selection
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC4PSC` reader - Input capture 4 prescaler
pub type IC4PSC_R = crate::FieldReader;
///Field `IC4PSC` writer - Input capture 4 prescaler
pub type IC4PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC4F` reader - Input capture 4 filter
pub type IC4F_R = crate::FieldReader;
///Field `IC4F` writer - Input capture 4 filter
pub type IC4F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:1 - Capture/compare 3 selection
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Input capture 3 prescaler
    #[inline(always)]
    pub fn ic3psc(&self) -> IC3PSC_R {
        IC3PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Input capture 3 filter
    #[inline(always)]
    pub fn ic3f(&self) -> IC3F_R {
        IC3F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - Capture/Compare 4 selection
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Input capture 4 prescaler
    #[inline(always)]
    pub fn ic4psc(&self) -> IC4PSC_R {
        IC4PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:15 - Input capture 4 filter
    #[inline(always)]
    pub fn ic4f(&self) -> IC4F_R {
        IC4F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR2_Input")
            .field("ic4f", &self.ic4f())
            .field("ic4psc", &self.ic4psc())
            .field("cc4s", &self.cc4s())
            .field("ic3f", &self.ic3f())
            .field("ic3psc", &self.ic3psc())
            .field("cc3s", &self.cc3s())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Capture/compare 3 selection
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W<'_, CCMR2_INPUTrs> {
        CC3S_W::new(self, 0)
    }
    ///Bits 2:3 - Input capture 3 prescaler
    #[inline(always)]
    pub fn ic3psc(&mut self) -> IC3PSC_W<'_, CCMR2_INPUTrs> {
        IC3PSC_W::new(self, 2)
    }
    ///Bits 4:7 - Input capture 3 filter
    #[inline(always)]
    pub fn ic3f(&mut self) -> IC3F_W<'_, CCMR2_INPUTrs> {
        IC3F_W::new(self, 4)
    }
    ///Bits 8:9 - Capture/Compare 4 selection
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W<'_, CCMR2_INPUTrs> {
        CC4S_W::new(self, 8)
    }
    ///Bits 10:11 - Input capture 4 prescaler
    #[inline(always)]
    pub fn ic4psc(&mut self) -> IC4PSC_W<'_, CCMR2_INPUTrs> {
        IC4PSC_W::new(self, 10)
    }
    ///Bits 12:15 - Input capture 4 filter
    #[inline(always)]
    pub fn ic4f(&mut self) -> IC4F_W<'_, CCMR2_INPUTrs> {
        IC4F_W::new(self, 12)
    }
}
/**capture/compare mode register 2 (input mode)

You can [`read`](crate::Reg::read) this register and get [`ccmr2_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#TIM5:CCMR2_Input)*/
pub struct CCMR2_INPUTrs;
impl crate::RegisterSpec for CCMR2_INPUTrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr2_input::R`](R) reader structure
impl crate::Readable for CCMR2_INPUTrs {}
///`write(|w| ..)` method takes [`ccmr2_input::W`](W) writer structure
impl crate::Writable for CCMR2_INPUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCMR2_Input to value 0
impl crate::Resettable for CCMR2_INPUTrs {}
