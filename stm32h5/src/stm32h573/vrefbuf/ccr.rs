#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "Field `TRIM` reader - Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM\\[5:0\\]
is automatically initialized with the VRS = 0 trimming value stored in the Flash memory during the production test. VRS change: TRIM\\[5:0\\]
is automatically initialized with the trimming value (corresponding to VRS setting) stored in the Flash memory during the production test. Write in TRIM\\[5:0\\]: User can modify the TRIM\\[5:0\\]
with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order."]
pub type TRIM_R = crate::FieldReader;
#[doc = "Field `TRIM` writer - Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM\\[5:0\\]
is automatically initialized with the VRS = 0 trimming value stored in the Flash memory during the production test. VRS change: TRIM\\[5:0\\]
is automatically initialized with the trimming value (corresponding to VRS setting) stored in the Flash memory during the production test. Write in TRIM\\[5:0\\]: User can modify the TRIM\\[5:0\\]
with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order."]
pub type TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM\\[5:0\\]
is automatically initialized with the VRS = 0 trimming value stored in the Flash memory during the production test. VRS change: TRIM\\[5:0\\]
is automatically initialized with the trimming value (corresponding to VRS setting) stored in the Flash memory during the production test. Write in TRIM\\[5:0\\]: User can modify the TRIM\\[5:0\\]
with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM\\[5:0\\]
is automatically initialized with the VRS = 0 trimming value stored in the Flash memory during the production test. VRS change: TRIM\\[5:0\\]
is automatically initialized with the trimming value (corresponding to VRS setting) stored in the Flash memory during the production test. Write in TRIM\\[5:0\\]: User can modify the TRIM\\[5:0\\]
with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order."]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<CCRrs> {
        TRIM_W::new(self, 0)
    }
}
#[doc = "VREFBUF calibration control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCRrs {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0;
}
