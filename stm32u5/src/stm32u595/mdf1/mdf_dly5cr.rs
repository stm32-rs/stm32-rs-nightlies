#[doc = "Register `MDF_DLY5CR` reader"]
pub type R = crate::R<MDF_DLY5CRrs>;
#[doc = "Register `MDF_DLY5CR` writer"]
pub type W = crate::W<MDF_DLY5CRrs>;
#[doc = "Field `SKPDLY` reader - Delay to apply to a bitstream Set and cleared by software. Defines the number of input samples that will be skipped. Skipping is applied immediately after writing to this field, if SKPBF = 0 , and the corresponding bit DFLTEN = 1 . If SKPBF = 1 the value written into the register is ignored by the delay state machine. - 0: No input sample skipped, - 1: 1 input sample skipped, ... - 127: 127 input sample skipped,"]
pub type SKPDLY_R = crate::FieldReader;
#[doc = "Field `SKPDLY` writer - Delay to apply to a bitstream Set and cleared by software. Defines the number of input samples that will be skipped. Skipping is applied immediately after writing to this field, if SKPBF = 0 , and the corresponding bit DFLTEN = 1 . If SKPBF = 1 the value written into the register is ignored by the delay state machine. - 0: No input sample skipped, - 1: 1 input sample skipped, ... - 127: 127 input sample skipped,"]
pub type SKPDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SKPBF` reader - Skip Busy flag Set and cleared by hardware. Shall be used in order to control if the delay sequence is completed. - 0: Reading 0 means that the MDF is ready to accept a new value into SKPDLY\\[6:0\\]. - 1: Reading 1 means that last valid SKPDLY\\[6:0\\]
is still under precessing."]
pub type SKPBF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - Delay to apply to a bitstream Set and cleared by software. Defines the number of input samples that will be skipped. Skipping is applied immediately after writing to this field, if SKPBF = 0 , and the corresponding bit DFLTEN = 1 . If SKPBF = 1 the value written into the register is ignored by the delay state machine. - 0: No input sample skipped, - 1: 1 input sample skipped, ... - 127: 127 input sample skipped,"]
    #[inline(always)]
    pub fn skpdly(&self) -> SKPDLY_R {
        SKPDLY_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Skip Busy flag Set and cleared by hardware. Shall be used in order to control if the delay sequence is completed. - 0: Reading 0 means that the MDF is ready to accept a new value into SKPDLY\\[6:0\\]. - 1: Reading 1 means that last valid SKPDLY\\[6:0\\]
is still under precessing."]
    #[inline(always)]
    pub fn skpbf(&self) -> SKPBF_R {
        SKPBF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Delay to apply to a bitstream Set and cleared by software. Defines the number of input samples that will be skipped. Skipping is applied immediately after writing to this field, if SKPBF = 0 , and the corresponding bit DFLTEN = 1 . If SKPBF = 1 the value written into the register is ignored by the delay state machine. - 0: No input sample skipped, - 1: 1 input sample skipped, ... - 127: 127 input sample skipped,"]
    #[inline(always)]
    #[must_use]
    pub fn skpdly(&mut self) -> SKPDLY_W<MDF_DLY5CRrs> {
        SKPDLY_W::new(self, 0)
    }
}
#[doc = "This register is used for the adjustment stream delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdf_dly5cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdf_dly5cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDF_DLY5CRrs;
impl crate::RegisterSpec for MDF_DLY5CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdf_dly5cr::R`](R) reader structure"]
impl crate::Readable for MDF_DLY5CRrs {}
#[doc = "`write(|w| ..)` method takes [`mdf_dly5cr::W`](W) writer structure"]
impl crate::Writable for MDF_DLY5CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDF_DLY5CR to value 0"]
impl crate::Resettable for MDF_DLY5CRrs {
    const RESET_VALUE: u32 = 0;
}
