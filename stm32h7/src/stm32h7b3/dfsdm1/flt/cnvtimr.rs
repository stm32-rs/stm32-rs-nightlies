#[doc = "Register `CNVTIMR` reader"]
pub type R = crate::R<CNVTIMRrs>;
#[doc = "Field `CNVCNT` reader - 28-bit timer counting conversion time t = CNVCNT\\[27:0\\]
/ fDFSDMCLK The timer has an input clock from DFSDM clock (system clock fDFSDMCLK). Conversion time measurement is started on each conversion start and stopped when conversion finishes (interval between first and last serial sample). Only in case of filter bypass (FOSR\\[9:0\\]
= 0) is the conversion time measurement stopped and CNVCNT\\[27:0\\]
= 0. The counted time is: if FAST=0 (or first conversion in continuous mode if FAST=1): t = \\[FOSR * (IOSR-1 + FORD) + FORD\\]
/ fCKIN ..... for Sincx filters t = \\[FOSR * (IOSR-1 + 4) + 2\\]
/ fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \\[FOSR * IOSR\\]
/ fCKIN in case if FOSR = FOSR\\[9:0\\]+1 = 1 (filter bypassed, active only integrator): CNVCNT = 0 (counting is stopped, conversion time: t = IOSR / fCKIN) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input (from internal ADC or from CPU/DMA write) Note: When conversion is interrupted (e.g. by disable/enable selected channel) the timer counts also this interruption time."]
pub type CNVCNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 4:31 - 28-bit timer counting conversion time t = CNVCNT\\[27:0\\]
/ fDFSDMCLK The timer has an input clock from DFSDM clock (system clock fDFSDMCLK). Conversion time measurement is started on each conversion start and stopped when conversion finishes (interval between first and last serial sample). Only in case of filter bypass (FOSR\\[9:0\\]
= 0) is the conversion time measurement stopped and CNVCNT\\[27:0\\]
= 0. The counted time is: if FAST=0 (or first conversion in continuous mode if FAST=1): t = \\[FOSR * (IOSR-1 + FORD) + FORD\\]
/ fCKIN ..... for Sincx filters t = \\[FOSR * (IOSR-1 + 4) + 2\\]
/ fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \\[FOSR * IOSR\\]
/ fCKIN in case if FOSR = FOSR\\[9:0\\]+1 = 1 (filter bypassed, active only integrator): CNVCNT = 0 (counting is stopped, conversion time: t = IOSR / fCKIN) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input (from internal ADC or from CPU/DMA write) Note: When conversion is interrupted (e.g. by disable/enable selected channel) the timer counts also this interruption time."]
    #[inline(always)]
    pub fn cnvcnt(&self) -> CNVCNT_R {
        CNVCNT_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnvtimr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNVTIMRrs;
impl crate::RegisterSpec for CNVTIMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnvtimr::R`](R) reader structure"]
impl crate::Readable for CNVTIMRrs {}
#[doc = "`reset()` method sets CNVTIMR to value 0"]
impl crate::Resettable for CNVTIMRrs {
    const RESET_VALUE: u32 = 0;
}
