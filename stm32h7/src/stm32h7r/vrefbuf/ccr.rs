///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `TRIM` reader - Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM\[5:0\] is automatically initialized with the VRS = 0 trimming value stored in the flash memory during the production test. VRS change: TRIM\[5:0\] is automatically initialized with the trimming value (corresponding to VRS setting) stored in the flash memory during the production test. Write in TRIM\[5:0\]: User can modify the TRIM\[5:0\] with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order.
pub type TRIM_R = crate::FieldReader;
///Field `TRIM` writer - Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM\[5:0\] is automatically initialized with the VRS = 0 trimming value stored in the flash memory during the production test. VRS change: TRIM\[5:0\] is automatically initialized with the trimming value (corresponding to VRS setting) stored in the flash memory during the production test. Write in TRIM\[5:0\]: User can modify the TRIM\[5:0\] with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order.
pub type TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM\[5:0\] is automatically initialized with the VRS = 0 trimming value stored in the flash memory during the production test. VRS change: TRIM\[5:0\] is automatically initialized with the trimming value (corresponding to VRS setting) stored in the flash memory during the production test. Write in TRIM\[5:0\]: User can modify the TRIM\[5:0\] with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order.
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR").field("trim", &self.trim()).finish()
    }
}
impl W {
    ///Bits 0:5 - Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM\[5:0\] is automatically initialized with the VRS = 0 trimming value stored in the flash memory during the production test. VRS change: TRIM\[5:0\] is automatically initialized with the trimming value (corresponding to VRS setting) stored in the flash memory during the production test. Write in TRIM\[5:0\]: User can modify the TRIM\[5:0\] with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order.
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W<'_, CCRrs> {
        TRIM_W::new(self, 0)
    }
}
/**VREFBUF calibration control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#VREFBUF:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}
