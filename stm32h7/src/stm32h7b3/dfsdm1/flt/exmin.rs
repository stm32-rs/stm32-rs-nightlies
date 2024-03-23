#[doc = "Register `EXMIN` reader"]
pub type R = crate::R<EXMINrs>;
#[doc = "Register `EXMIN` writer"]
pub type W = crate::W<EXMINrs>;
#[doc = "Field `EXMINCH` reader - Extremes detector minimum data channel These bits contain information about the channel on which the data is stored into EXMIN\\[23:0\\]. Bits are cleared by reading of this register."]
pub type EXMINCH_R = crate::FieldReader;
#[doc = "Field `EXMIN` reader - Extremes detector minimum value These bits are set by hardware and indicate the lowest value converted by DFSDM_FLTx. EXMIN\\[23:0\\]
bits are reset to value (0x7FFFFF) by reading of this register.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type EXMIN_R = crate::FieldReader<u32>;
#[doc = "Field `EXMIN` writer - Extremes detector minimum value These bits are set by hardware and indicate the lowest value converted by DFSDM_FLTx. EXMIN\\[23:0\\]
bits are reset to value (0x7FFFFF) by reading of this register."]
pub type EXMIN_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:2 - Extremes detector minimum data channel These bits contain information about the channel on which the data is stored into EXMIN\\[23:0\\]. Bits are cleared by reading of this register."]
    #[inline(always)]
    pub fn exminch(&self) -> EXMINCH_R {
        EXMINCH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - Extremes detector minimum value These bits are set by hardware and indicate the lowest value converted by DFSDM_FLTx. EXMIN\\[23:0\\]
bits are reset to value (0x7FFFFF) by reading of this register."]
    #[inline(always)]
    pub fn exmin(&self) -> EXMIN_R {
        EXMIN_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - Extremes detector minimum value These bits are set by hardware and indicate the lowest value converted by DFSDM_FLTx. EXMIN\\[23:0\\]
bits are reset to value (0x7FFFFF) by reading of this register."]
    #[inline(always)]
    #[must_use]
    pub fn exmin(&mut self) -> EXMIN_W<EXMINrs> {
        EXMIN_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exmin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exmin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXMINrs;
impl crate::RegisterSpec for EXMINrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exmin::R`](R) reader structure"]
impl crate::Readable for EXMINrs {}
#[doc = "`write(|w| ..)` method takes [`exmin::W`](W) writer structure"]
impl crate::Writable for EXMINrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXMIN to value 0x7fff_ff00"]
impl crate::Resettable for EXMINrs {
    const RESET_VALUE: u32 = 0x7fff_ff00;
}
