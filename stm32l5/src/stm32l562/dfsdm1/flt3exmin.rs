#[doc = "Register `FLT3EXMIN` reader"]
pub type R = crate::R<FLT3EXMINrs>;
#[doc = "Field `EXMINCH` reader - Extremes detector minimum data channel"]
pub type EXMINCH_R = crate::FieldReader;
#[doc = "Field `EXMIN` reader - EXMIN"]
pub type EXMIN_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - Extremes detector minimum data channel"]
    #[inline(always)]
    pub fn exminch(&self) -> EXMINCH_R {
        EXMINCH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - EXMIN"]
    #[inline(always)]
    pub fn exmin(&self) -> EXMIN_R {
        EXMIN_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "Extremes detector minimum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flt3exmin::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLT3EXMINrs;
impl crate::RegisterSpec for FLT3EXMINrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flt3exmin::R`](R) reader structure"]
impl crate::Readable for FLT3EXMINrs {}
#[doc = "`reset()` method sets FLT3EXMIN to value 0x7fff_ff00"]
impl crate::Resettable for FLT3EXMINrs {
    const RESET_VALUE: u32 = 0x7fff_ff00;
}
