#[doc = "Register `LTDC_L2DCCR` reader"]
pub type R = crate::R<LTDC_L2DCCRrs>;
#[doc = "Register `LTDC_L2DCCR` writer"]
pub type W = crate::W<LTDC_L2DCCRrs>;
#[doc = "Field `DCBLUE` reader - default color blue These bits configure the default blue value."]
pub type DCBLUE_R = crate::FieldReader;
#[doc = "Field `DCBLUE` writer - default color blue These bits configure the default blue value."]
pub type DCBLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DCGREEN` reader - default color green These bits configure the default green value."]
pub type DCGREEN_R = crate::FieldReader;
#[doc = "Field `DCGREEN` writer - default color green These bits configure the default green value."]
pub type DCGREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DCRED` reader - default color red These bits configure the default red value."]
pub type DCRED_R = crate::FieldReader;
#[doc = "Field `DCRED` writer - default color red These bits configure the default red value."]
pub type DCRED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DCALPHA` reader - default color alpha These bits configure the default alpha value."]
pub type DCALPHA_R = crate::FieldReader;
#[doc = "Field `DCALPHA` writer - default color alpha These bits configure the default alpha value."]
pub type DCALPHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - default color blue These bits configure the default blue value."]
    #[inline(always)]
    pub fn dcblue(&self) -> DCBLUE_R {
        DCBLUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - default color green These bits configure the default green value."]
    #[inline(always)]
    pub fn dcgreen(&self) -> DCGREEN_R {
        DCGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - default color red These bits configure the default red value."]
    #[inline(always)]
    pub fn dcred(&self) -> DCRED_R {
        DCRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - default color alpha These bits configure the default alpha value."]
    #[inline(always)]
    pub fn dcalpha(&self) -> DCALPHA_R {
        DCALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - default color blue These bits configure the default blue value."]
    #[inline(always)]
    #[must_use]
    pub fn dcblue(&mut self) -> DCBLUE_W<LTDC_L2DCCRrs> {
        DCBLUE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - default color green These bits configure the default green value."]
    #[inline(always)]
    #[must_use]
    pub fn dcgreen(&mut self) -> DCGREEN_W<LTDC_L2DCCRrs> {
        DCGREEN_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - default color red These bits configure the default red value."]
    #[inline(always)]
    #[must_use]
    pub fn dcred(&mut self) -> DCRED_W<LTDC_L2DCCRrs> {
        DCRED_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - default color alpha These bits configure the default alpha value."]
    #[inline(always)]
    #[must_use]
    pub fn dcalpha(&mut self) -> DCALPHA_W<LTDC_L2DCCRrs> {
        DCALPHA_W::new(self, 24)
    }
}
#[doc = "LTDC layer 2 default color configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_l2dccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_l2dccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_L2DCCRrs;
impl crate::RegisterSpec for LTDC_L2DCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_l2dccr::R`](R) reader structure"]
impl crate::Readable for LTDC_L2DCCRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_l2dccr::W`](W) writer structure"]
impl crate::Writable for LTDC_L2DCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_L2DCCR to value 0"]
impl crate::Resettable for LTDC_L2DCCRrs {
    const RESET_VALUE: u32 = 0;
}
