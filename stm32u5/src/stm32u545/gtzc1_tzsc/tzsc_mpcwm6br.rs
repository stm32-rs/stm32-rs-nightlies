#[doc = "Register `TZSC_MPCWM6BR` reader"]
pub type R = crate::R<TZSC_MPCWM6BRrs>;
#[doc = "Register `TZSC_MPCWM6BR` writer"]
pub type W = crate::W<TZSC_MPCWM6BRrs>;
#[doc = "Field `SUBB_START` reader - Start of sub-region A"]
pub type SUBB_START_R = crate::FieldReader<u16>;
#[doc = "Field `SUBB_START` writer - Start of sub-region A"]
pub type SUBB_START_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SUBB_LENGTH` reader - Length of sub-region A"]
pub type SUBB_LENGTH_R = crate::FieldReader<u16>;
#[doc = "Field `SUBB_LENGTH` writer - Length of sub-region A"]
pub type SUBB_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:10 - Start of sub-region A"]
    #[inline(always)]
    pub fn subb_start(&self) -> SUBB_START_R {
        SUBB_START_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - Length of sub-region A"]
    #[inline(always)]
    pub fn subb_length(&self) -> SUBB_LENGTH_R {
        SUBB_LENGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Start of sub-region A"]
    #[inline(always)]
    #[must_use]
    pub fn subb_start(&mut self) -> SUBB_START_W<TZSC_MPCWM6BRrs> {
        SUBB_START_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Length of sub-region A"]
    #[inline(always)]
    #[must_use]
    pub fn subb_length(&mut self) -> SUBB_LENGTH_W<TZSC_MPCWM6BRrs> {
        SUBB_LENGTH_W::new(self, 16)
    }
}
#[doc = "TZSC memory 6 sub-region B watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_mpcwm6br::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_mpcwm6br::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZSC_MPCWM6BRrs;
impl crate::RegisterSpec for TZSC_MPCWM6BRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzsc_mpcwm6br::R`](R) reader structure"]
impl crate::Readable for TZSC_MPCWM6BRrs {}
#[doc = "`write(|w| ..)` method takes [`tzsc_mpcwm6br::W`](W) writer structure"]
impl crate::Writable for TZSC_MPCWM6BRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZSC_MPCWM6BR to value 0"]
impl crate::Resettable for TZSC_MPCWM6BRrs {
    const RESET_VALUE: u32 = 0;
}
