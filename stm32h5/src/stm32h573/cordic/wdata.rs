#[doc = "Register `WDATA` writer"]
pub type W = crate::W<WDATArs>;
#[doc = "Field `ARG` writer - Function input arguments This register is programmed with the input arguments for the function selected in the CORDIC_CSR register FUNC field. If 32-bit format is selected (CORDIC_CSR.ARGSIZE = 0) and two input arguments are required (CORDIC_CSR.NARGS = 1), two successive writes are required to this register. The first writes the primary argument (ARG1), the second writes the secondary argument (ARG2). If 32-bit format is selected and only one input argument is required (NARGS = 0), only one write is required to this register, containing the primary argument (ARG1). If 16-bit format is selected (CORDIC_CSR.ARGSIZE = 1), one write to this register contains both arguments. The primary argument (ARG1) is in the lower half, ARG\\[15:0\\], and the secondary argument (ARG2) is in the upper half, ARG\\[31:16\\]. In this case, NARGS must be set to 0. Refer to for the arguments required by each function, and their permitted range. When the required number of arguments has been written, the CORDIC evaluates the function designated by CORDIC_CSR.FUNC using the supplied input arguments, provided any previous calculation has completed. If a calculation is ongoing, the ARG1 and ARG 2 values are held pending until the calculation is completed and the results read. During this time, a write to the register cancels the pending operation and overwrite the argument data."]
pub type ARG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Function input arguments This register is programmed with the input arguments for the function selected in the CORDIC_CSR register FUNC field. If 32-bit format is selected (CORDIC_CSR.ARGSIZE = 0) and two input arguments are required (CORDIC_CSR.NARGS = 1), two successive writes are required to this register. The first writes the primary argument (ARG1), the second writes the secondary argument (ARG2). If 32-bit format is selected and only one input argument is required (NARGS = 0), only one write is required to this register, containing the primary argument (ARG1). If 16-bit format is selected (CORDIC_CSR.ARGSIZE = 1), one write to this register contains both arguments. The primary argument (ARG1) is in the lower half, ARG\\[15:0\\], and the secondary argument (ARG2) is in the upper half, ARG\\[31:16\\]. In this case, NARGS must be set to 0. Refer to for the arguments required by each function, and their permitted range. When the required number of arguments has been written, the CORDIC evaluates the function designated by CORDIC_CSR.FUNC using the supplied input arguments, provided any previous calculation has completed. If a calculation is ongoing, the ARG1 and ARG 2 values are held pending until the calculation is completed and the results read. During this time, a write to the register cancels the pending operation and overwrite the argument data."]
    #[inline(always)]
    #[must_use]
    pub fn arg(&mut self) -> ARG_W<WDATArs> {
        ARG_W::new(self, 0)
    }
}
#[doc = "CORDIC argument register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDATArs;
impl crate::RegisterSpec for WDATArs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wdata::W`](W) writer structure"]
impl crate::Writable for WDATArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDATA to value 0"]
impl crate::Resettable for WDATArs {
    const RESET_VALUE: u32 = 0;
}
