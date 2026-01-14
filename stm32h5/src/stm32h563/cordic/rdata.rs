///Register `RDATA` reader
pub type R = crate::R<RDATArs>;
///Field `RES` reader - Function result If 32-bit format is selected (CORDIC_CSR.RESSIZE = 0) and two output values are expected (CORDIC_CSR.NRES = 1), this register must be read twice when the RRDY flag is set. The first read fetches the primary result (RES1). The second read fetches the secondary result (RES2) and resets RRDY. If 32-bit format is selected and only one output value is expected (NRES = 0), only one read of this register is required to fetch the primary result (RES1) and reset the RRDY flag. If 16-bit format is selected (CORDIC_CSR.RESSIZE = 1), this register contains the primary result (RES1) in the lower half, RES\[15:0\], and the secondary result (RES2) in the upper half, RES\[31:16\]. In this case, NRES must be set to 0, and only one read performed. A read from this register resets the RRDY flag in the CORDIC_CSR register.
pub type RES_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Function result If 32-bit format is selected (CORDIC_CSR.RESSIZE = 0) and two output values are expected (CORDIC_CSR.NRES = 1), this register must be read twice when the RRDY flag is set. The first read fetches the primary result (RES1). The second read fetches the secondary result (RES2) and resets RRDY. If 32-bit format is selected and only one output value is expected (NRES = 0), only one read of this register is required to fetch the primary result (RES1) and reset the RRDY flag. If 16-bit format is selected (CORDIC_CSR.RESSIZE = 1), this register contains the primary result (RES1) in the lower half, RES\[15:0\], and the secondary result (RES2) in the upper half, RES\[31:16\]. In this case, NRES must be set to 0, and only one read performed. A read from this register resets the RRDY flag in the CORDIC_CSR register.
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDATA").field("res", &self.res()).finish()
    }
}
/**CORDIC result register

You can [`read`](crate::Reg::read) this register and get [`rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#CORDIC:RDATA)*/
pub struct RDATArs;
impl crate::RegisterSpec for RDATArs {
    type Ux = u32;
}
///`read()` method returns [`rdata::R`](R) reader structure
impl crate::Readable for RDATArs {}
///`reset()` method sets RDATA to value 0
impl crate::Resettable for RDATArs {}
