#[doc = "Register `CORDIC_RDATA` reader"]
pub type R = crate::R<CORDIC_RDATArs>;
#[doc = "Field `RES` reader - Function result If 32-bit format is selected (CORDIC_CSR.RESSIZE = 0) and two output values are expected (CORDIC_CSR.NRES = 1), this register must be read twice when the RRDY flag is set. The first read fetches the primary result (RES1). The second read fetches the secondary result (RES2) and resets RRDY. If 32-bit format is selected and only one output value is expected (NRES = 0), only one read of this register is required to fetch the primary result (RES1) and reset the RRDY flag. If 16-bit format is selected (CORDIC_CSR.RESSIZE = 1), this register contains the primary result (RES1) in the lower half, RES\\[15:0\\], and the secondary result (RES2) in the upper half, RES\\[31:16\\]. In this case, NRES must be set to 0, and only one read performed. A read from this register resets the RRDY flag in the CORDIC_CSR register."]
pub type RES_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Function result If 32-bit format is selected (CORDIC_CSR.RESSIZE = 0) and two output values are expected (CORDIC_CSR.NRES = 1), this register must be read twice when the RRDY flag is set. The first read fetches the primary result (RES1). The second read fetches the secondary result (RES2) and resets RRDY. If 32-bit format is selected and only one output value is expected (NRES = 0), only one read of this register is required to fetch the primary result (RES1) and reset the RRDY flag. If 16-bit format is selected (CORDIC_CSR.RESSIZE = 1), this register contains the primary result (RES1) in the lower half, RES\\[15:0\\], and the secondary result (RES2) in the upper half, RES\\[31:16\\]. In this case, NRES must be set to 0, and only one read performed. A read from this register resets the RRDY flag in the CORDIC_CSR register."]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(self.bits)
    }
}
#[doc = "CORDIC result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cordic_rdata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORDIC_RDATArs;
impl crate::RegisterSpec for CORDIC_RDATArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cordic_rdata::R`](R) reader structure"]
impl crate::Readable for CORDIC_RDATArs {}
#[doc = "`reset()` method sets CORDIC_RDATA to value 0"]
impl crate::Resettable for CORDIC_RDATArs {
    const RESET_VALUE: u32 = 0;
}
