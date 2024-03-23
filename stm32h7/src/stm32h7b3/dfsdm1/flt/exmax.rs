#[doc = "Register `EXMAX` reader"]
pub type R = crate::R<EXMAXrs>;
#[doc = "Field `EXMAXCH` reader - Extremes detector maximum data channel. These bits contains information about the channel on which the data is stored into EXMAX\\[23:0\\]. Bits are cleared by reading of this register."]
pub type EXMAXCH_R = crate::FieldReader;
#[doc = "Field `EXMAX` reader - Extremes detector maximum value These bits are set by hardware and indicate the highest value converted by DFSDM_FLTx. EXMAX\\[23:0\\]
bits are reset to value (0x800000) by reading of this register.\n\nThe field is **set** (set to ones) following a read operation."]
pub type EXMAX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - Extremes detector maximum data channel. These bits contains information about the channel on which the data is stored into EXMAX\\[23:0\\]. Bits are cleared by reading of this register."]
    #[inline(always)]
    pub fn exmaxch(&self) -> EXMAXCH_R {
        EXMAXCH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - Extremes detector maximum value These bits are set by hardware and indicate the highest value converted by DFSDM_FLTx. EXMAX\\[23:0\\]
bits are reset to value (0x800000) by reading of this register."]
    #[inline(always)]
    pub fn exmax(&self) -> EXMAX_R {
        EXMAX_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exmax::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXMAXrs;
impl crate::RegisterSpec for EXMAXrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exmax::R`](R) reader structure"]
impl crate::Readable for EXMAXrs {}
#[doc = "`reset()` method sets EXMAX to value 0x8000_0000"]
impl crate::Resettable for EXMAXrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
