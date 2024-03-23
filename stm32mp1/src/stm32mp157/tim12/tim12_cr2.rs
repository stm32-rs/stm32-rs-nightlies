#[doc = "Register `TIM12_CR2` reader"]
pub type R = crate::R<TIM12_CR2rs>;
#[doc = "Register `TIM12_CR2` writer"]
pub type W = crate::W<TIM12_CR2rs>;
#[doc = "Field `MMS` reader - MMS"]
pub type MMS_R = crate::FieldReader;
#[doc = "Field `MMS` writer - MMS"]
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TI1S` reader - TI1S"]
pub type TI1S_R = crate::BitReader;
#[doc = "Field `TI1S` writer - TI1S"]
pub type TI1S_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:6 - MMS"]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - TI1S"]
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - MMS"]
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<TIM12_CR2rs> {
        MMS_W::new(self, 4)
    }
    #[doc = "Bit 7 - TI1S"]
    #[inline(always)]
    #[must_use]
    pub fn ti1s(&mut self) -> TI1S_W<TIM12_CR2rs> {
        TI1S_W::new(self, 7)
    }
}
#[doc = "TIM12 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim12_cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim12_cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM12_CR2rs;
impl crate::RegisterSpec for TIM12_CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim12_cr2::R`](R) reader structure"]
impl crate::Readable for TIM12_CR2rs {}
#[doc = "`write(|w| ..)` method takes [`tim12_cr2::W`](W) writer structure"]
impl crate::Writable for TIM12_CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM12_CR2 to value 0"]
impl crate::Resettable for TIM12_CR2rs {
    const RESET_VALUE: u32 = 0;
}
