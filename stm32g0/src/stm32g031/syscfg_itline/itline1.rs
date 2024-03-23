#[doc = "Register `ITLINE1` reader"]
pub type R = crate::R<ITLINE1rs>;
#[doc = "Field `PVDOUT` reader - PVD supply monitoring interrupt request pending (EXTI line 16)."]
pub type PVDOUT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PVD supply monitoring interrupt request pending (EXTI line 16)."]
    #[inline(always)]
    pub fn pvdout(&self) -> PVDOUT_R {
        PVDOUT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE1rs;
impl crate::RegisterSpec for ITLINE1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline1::R`](R) reader structure"]
impl crate::Readable for ITLINE1rs {}
#[doc = "`reset()` method sets ITLINE1 to value 0"]
impl crate::Resettable for ITLINE1rs {
    const RESET_VALUE: u32 = 0;
}
