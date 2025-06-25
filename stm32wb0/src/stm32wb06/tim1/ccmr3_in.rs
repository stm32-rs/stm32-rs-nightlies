///Register `CCMR3_in` reader
pub type R = crate::R<CCMR3_INrs>;
///Register `CCMR3_in` writer
pub type W = crate::W<CCMR3_INrs>;
///Field `IC5PSC` reader - IC5PSC: Input capture 1 prescaler
pub type IC5PSC_R = crate::FieldReader;
///Field `IC5PSC` writer - IC5PSC: Input capture 1 prescaler
pub type IC5PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC5F` reader - Bits 7:4 IC1F\[3:0\]: Input capture 1 filter
pub type IC5F_R = crate::FieldReader;
///Field `IC5F` writer - Bits 7:4 IC1F\[3:0\]: Input capture 1 filter
pub type IC5F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `IC6PSC` reader - IC6PSC\[1:0\]: Input capture 2 prescaler
pub type IC6PSC_R = crate::FieldReader;
///Field `IC6PSC` writer - IC6PSC\[1:0\]: Input capture 2 prescaler
pub type IC6PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC6F` reader - IC2F: Input capture 2 filter
pub type IC6F_R = crate::FieldReader;
///Field `IC6F` writer - IC2F: Input capture 2 filter
pub type IC6F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 2:3 - IC5PSC: Input capture 1 prescaler
    #[inline(always)]
    pub fn ic5psc(&self) -> IC5PSC_R {
        IC5PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Bits 7:4 IC1F\[3:0\]: Input capture 1 filter
    #[inline(always)]
    pub fn ic5f(&self) -> IC5F_R {
        IC5F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 10:11 - IC6PSC\[1:0\]: Input capture 2 prescaler
    #[inline(always)]
    pub fn ic6psc(&self) -> IC6PSC_R {
        IC6PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:15 - IC2F: Input capture 2 filter
    #[inline(always)]
    pub fn ic6f(&self) -> IC6F_R {
        IC6F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR3_in")
            .field("ic5psc", &self.ic5psc())
            .field("ic5f", &self.ic5f())
            .field("ic6psc", &self.ic6psc())
            .field("ic6f", &self.ic6f())
            .finish()
    }
}
impl W {
    ///Bits 2:3 - IC5PSC: Input capture 1 prescaler
    #[inline(always)]
    pub fn ic5psc(&mut self) -> IC5PSC_W<CCMR3_INrs> {
        IC5PSC_W::new(self, 2)
    }
    ///Bits 4:7 - Bits 7:4 IC1F\[3:0\]: Input capture 1 filter
    #[inline(always)]
    pub fn ic5f(&mut self) -> IC5F_W<CCMR3_INrs> {
        IC5F_W::new(self, 4)
    }
    ///Bits 10:11 - IC6PSC\[1:0\]: Input capture 2 prescaler
    #[inline(always)]
    pub fn ic6psc(&mut self) -> IC6PSC_W<CCMR3_INrs> {
        IC6PSC_W::new(self, 10)
    }
    ///Bits 12:15 - IC2F: Input capture 2 filter
    #[inline(always)]
    pub fn ic6f(&mut self) -> IC6F_W<CCMR3_INrs> {
        IC6F_W::new(self, 12)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`ccmr3_in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3_in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#TIM1:CCMR3_in)*/
pub struct CCMR3_INrs;
impl crate::RegisterSpec for CCMR3_INrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr3_in::R`](R) reader structure
impl crate::Readable for CCMR3_INrs {}
///`write(|w| ..)` method takes [`ccmr3_in::W`](W) writer structure
impl crate::Writable for CCMR3_INrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCMR3_in to value 0
impl crate::Resettable for CCMR3_INrs {}
