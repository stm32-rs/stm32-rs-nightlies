#[doc = "Register `MDF_BSMX3CR` reader"]
pub type R = crate::R<MDF_BSMX3CRrs>;
#[doc = "Register `MDF_BSMX3CR` writer"]
pub type W = crate::W<MDF_BSMX3CRrs>;
#[doc = "Field `BSSEL` reader - Bitstream Selection Set and cleared by software. This field is used to select the bitstream to be processed for the digital filter x and for the SCDx. The size of this field depends on the number of DFLTx instantiated. If the BSSEL is selecting an input which is not instantiated, the MDF will select the valid stream bs\\[x\\]_F having the higher index number. - 00000: The bitstream bs\\[0\\]_R is provided to DFLTx and SCDx - 00001: The bitstream bs\\[0\\]_F is provided to DFLTx and SCDx - 00010: The bitstream bs\\[1\\]_R is provided to DFLTx and SCDx (if instantiated) - 00011: The bitstream bs\\[1\\]_F is provided to DFLTx and SCDx (if instantiated) ... - 11110: The bitstream bs\\[15\\]_R is provided to DFLTx and SCDx (if instantiated) - 11111: The bitstream bs\\[15\\]_F is provided to DFLTx and SCDx (if instantiated) This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
pub type BSSEL_R = crate::FieldReader;
#[doc = "Field `BSSEL` writer - Bitstream Selection Set and cleared by software. This field is used to select the bitstream to be processed for the digital filter x and for the SCDx. The size of this field depends on the number of DFLTx instantiated. If the BSSEL is selecting an input which is not instantiated, the MDF will select the valid stream bs\\[x\\]_F having the higher index number. - 00000: The bitstream bs\\[0\\]_R is provided to DFLTx and SCDx - 00001: The bitstream bs\\[0\\]_F is provided to DFLTx and SCDx - 00010: The bitstream bs\\[1\\]_R is provided to DFLTx and SCDx (if instantiated) - 00011: The bitstream bs\\[1\\]_F is provided to DFLTx and SCDx (if instantiated) ... - 11110: The bitstream bs\\[15\\]_R is provided to DFLTx and SCDx (if instantiated) - 11111: The bitstream bs\\[15\\]_F is provided to DFLTx and SCDx (if instantiated) This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
pub type BSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BSMXACTIVE` reader - BSMX Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the BSMX is effectively enabled (active) or not. BSSEL\\[4:0\\]
can only be updated when the BSMXACTIVE is set to a . The BSMXACTIVE flag is a logical between OLDACTIVE, DFLTACTIVE, and SCDACTIVE flags. Both of them must be set to a in order update BSSEL\\[4:0\\]
field. - 0: The BSMX is not active, and can be configured if needed - 1: The BSMX is active, and protected fields cannot be configured."]
pub type BSMXACTIVE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Bitstream Selection Set and cleared by software. This field is used to select the bitstream to be processed for the digital filter x and for the SCDx. The size of this field depends on the number of DFLTx instantiated. If the BSSEL is selecting an input which is not instantiated, the MDF will select the valid stream bs\\[x\\]_F having the higher index number. - 00000: The bitstream bs\\[0\\]_R is provided to DFLTx and SCDx - 00001: The bitstream bs\\[0\\]_F is provided to DFLTx and SCDx - 00010: The bitstream bs\\[1\\]_R is provided to DFLTx and SCDx (if instantiated) - 00011: The bitstream bs\\[1\\]_F is provided to DFLTx and SCDx (if instantiated) ... - 11110: The bitstream bs\\[15\\]_R is provided to DFLTx and SCDx (if instantiated) - 11111: The bitstream bs\\[15\\]_F is provided to DFLTx and SCDx (if instantiated) This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
    #[inline(always)]
    pub fn bssel(&self) -> BSSEL_R {
        BSSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - BSMX Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the BSMX is effectively enabled (active) or not. BSSEL\\[4:0\\]
can only be updated when the BSMXACTIVE is set to a . The BSMXACTIVE flag is a logical between OLDACTIVE, DFLTACTIVE, and SCDACTIVE flags. Both of them must be set to a in order update BSSEL\\[4:0\\]
field. - 0: The BSMX is not active, and can be configured if needed - 1: The BSMX is active, and protected fields cannot be configured."]
    #[inline(always)]
    pub fn bsmxactive(&self) -> BSMXACTIVE_R {
        BSMXACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Bitstream Selection Set and cleared by software. This field is used to select the bitstream to be processed for the digital filter x and for the SCDx. The size of this field depends on the number of DFLTx instantiated. If the BSSEL is selecting an input which is not instantiated, the MDF will select the valid stream bs\\[x\\]_F having the higher index number. - 00000: The bitstream bs\\[0\\]_R is provided to DFLTx and SCDx - 00001: The bitstream bs\\[0\\]_F is provided to DFLTx and SCDx - 00010: The bitstream bs\\[1\\]_R is provided to DFLTx and SCDx (if instantiated) - 00011: The bitstream bs\\[1\\]_F is provided to DFLTx and SCDx (if instantiated) ... - 11110: The bitstream bs\\[15\\]_R is provided to DFLTx and SCDx (if instantiated) - 11111: The bitstream bs\\[15\\]_F is provided to DFLTx and SCDx (if instantiated) This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
    #[inline(always)]
    #[must_use]
    pub fn bssel(&mut self) -> BSSEL_W<MDF_BSMX3CRrs> {
        BSSEL_W::new(self, 0)
    }
}
#[doc = "This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdf_bsmx3cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdf_bsmx3cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDF_BSMX3CRrs;
impl crate::RegisterSpec for MDF_BSMX3CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdf_bsmx3cr::R`](R) reader structure"]
impl crate::Readable for MDF_BSMX3CRrs {}
#[doc = "`write(|w| ..)` method takes [`mdf_bsmx3cr::W`](W) writer structure"]
impl crate::Writable for MDF_BSMX3CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDF_BSMX3CR to value 0"]
impl crate::Resettable for MDF_BSMX3CRrs {
    const RESET_VALUE: u32 = 0;
}
