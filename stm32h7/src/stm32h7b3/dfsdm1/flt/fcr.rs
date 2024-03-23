#[doc = "Register `FCR` reader"]
pub type R = crate::R<FCRrs>;
#[doc = "Register `FCR` writer"]
pub type W = crate::W<FCRrs>;
#[doc = "Field `IOSR` reader - Integrator oversampling ratio (averaging length) from Sinc filter will be summed into one output data sample from the integrator. The output data rate from the integrator will be decreased by this number (additional data decimation ratio). This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If IOSR = 0, then the Integrator has no effect (Integrator bypass). 0- 255: The length of the Integrator in the range 1 - 256 (IOSR + 1). Defines how many samples"]
pub type IOSR_R = crate::FieldReader;
#[doc = "Field `IOSR` writer - Integrator oversampling ratio (averaging length) from Sinc filter will be summed into one output data sample from the integrator. The output data rate from the integrator will be decreased by this number (additional data decimation ratio). This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If IOSR = 0, then the Integrator has no effect (Integrator bypass). 0- 255: The length of the Integrator in the range 1 - 256 (IOSR + 1). Defines how many samples"]
pub type IOSR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FOSR` reader - Sinc filter oversampling ratio (decimation rate) number is also the decimation ratio of the output data rate from filter. This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If FOSR = 0, then the filter has no effect (filter bypass). 0 - 1023: Defines the length of the Sinc type filter in the range 1 - 1024 (FOSR = FOSR\\[9:0\\]
+1). This"]
pub type FOSR_R = crate::FieldReader<u16>;
#[doc = "Field `FOSR` writer - Sinc filter oversampling ratio (decimation rate) number is also the decimation ratio of the output data rate from filter. This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If FOSR = 0, then the filter has no effect (filter bypass). 0 - 1023: Defines the length of the Sinc type filter in the range 1 - 1024 (FOSR = FOSR\\[9:0\\]
+1). This"]
pub type FOSR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `FORD` reader - Sinc filter order 2: Sinc2 filter type 3: Sinc3 filter type 4: Sinc4 filter type 5: Sinc5 filter type 6-7: Reserved Sincx filter type transfer function: FastSinc filter type transfer function: This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1)."]
pub type FORD_R = crate::FieldReader;
#[doc = "Field `FORD` writer - Sinc filter order 2: Sinc2 filter type 3: Sinc3 filter type 4: Sinc4 filter type 5: Sinc5 filter type 6-7: Reserved Sincx filter type transfer function: FastSinc filter type transfer function: This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1)."]
pub type FORD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - Integrator oversampling ratio (averaging length) from Sinc filter will be summed into one output data sample from the integrator. The output data rate from the integrator will be decreased by this number (additional data decimation ratio). This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If IOSR = 0, then the Integrator has no effect (Integrator bypass). 0- 255: The length of the Integrator in the range 1 - 256 (IOSR + 1). Defines how many samples"]
    #[inline(always)]
    pub fn iosr(&self) -> IOSR_R {
        IOSR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate) number is also the decimation ratio of the output data rate from filter. This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If FOSR = 0, then the filter has no effect (filter bypass). 0 - 1023: Defines the length of the Sinc type filter in the range 1 - 1024 (FOSR = FOSR\\[9:0\\]
+1). This"]
    #[inline(always)]
    pub fn fosr(&self) -> FOSR_R {
        FOSR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:31 - Sinc filter order 2: Sinc2 filter type 3: Sinc3 filter type 4: Sinc4 filter type 5: Sinc5 filter type 6-7: Reserved Sincx filter type transfer function: FastSinc filter type transfer function: This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1)."]
    #[inline(always)]
    pub fn ford(&self) -> FORD_R {
        FORD_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Integrator oversampling ratio (averaging length) from Sinc filter will be summed into one output data sample from the integrator. The output data rate from the integrator will be decreased by this number (additional data decimation ratio). This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If IOSR = 0, then the Integrator has no effect (Integrator bypass). 0- 255: The length of the Integrator in the range 1 - 256 (IOSR + 1). Defines how many samples"]
    #[inline(always)]
    #[must_use]
    pub fn iosr(&mut self) -> IOSR_W<FCRrs> {
        IOSR_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate) number is also the decimation ratio of the output data rate from filter. This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If FOSR = 0, then the filter has no effect (filter bypass). 0 - 1023: Defines the length of the Sinc type filter in the range 1 - 1024 (FOSR = FOSR\\[9:0\\]
+1). This"]
    #[inline(always)]
    #[must_use]
    pub fn fosr(&mut self) -> FOSR_W<FCRrs> {
        FOSR_W::new(self, 16)
    }
    #[doc = "Bits 29:31 - Sinc filter order 2: Sinc2 filter type 3: Sinc3 filter type 4: Sinc4 filter type 5: Sinc5 filter type 6-7: Reserved Sincx filter type transfer function: FastSinc filter type transfer function: This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1)."]
    #[inline(always)]
    #[must_use]
    pub fn ford(&mut self) -> FORD_W<FCRrs> {
        FORD_W::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCRrs;
impl crate::RegisterSpec for FCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcr::R`](R) reader structure"]
impl crate::Readable for FCRrs {}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCRrs {
    const RESET_VALUE: u32 = 0;
}
