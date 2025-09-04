///Register `FCR` reader
pub type R = crate::R<FCRrs>;
///Register `FCR` writer
pub type W = crate::W<FCRrs>;
///Field `IOSR` reader - Integrator oversampling ratio (averaging length) from Sinc filter will be summed into one output data sample from the integrator. The output data rate from the integrator will be decreased by this number (additional data decimation ratio). This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If IOSR = 0, then the Integrator has no effect (Integrator bypass). 0- 255: The length of the Integrator in the range 1 - 256 (IOSR + 1). Defines how many samples
pub type IOSR_R = crate::FieldReader;
///Field `IOSR` writer - Integrator oversampling ratio (averaging length) from Sinc filter will be summed into one output data sample from the integrator. The output data rate from the integrator will be decreased by this number (additional data decimation ratio). This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If IOSR = 0, then the Integrator has no effect (Integrator bypass). 0- 255: The length of the Integrator in the range 1 - 256 (IOSR + 1). Defines how many samples
pub type IOSR_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `FOSR` reader - Sinc filter oversampling ratio (decimation rate) number is also the decimation ratio of the output data rate from filter. This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If FOSR = 0, then the filter has no effect (filter bypass). 0 - 1023: Defines the length of the Sinc type filter in the range 1 - 1024 (FOSR = FOSR\[9:0\] +1). This
pub type FOSR_R = crate::FieldReader<u16>;
///Field `FOSR` writer - Sinc filter oversampling ratio (decimation rate) number is also the decimation ratio of the output data rate from filter. This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If FOSR = 0, then the filter has no effect (filter bypass). 0 - 1023: Defines the length of the Sinc type filter in the range 1 - 1024 (FOSR = FOSR\[9:0\] +1). This
pub type FOSR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
/**Sinc filter order 2: Sinc2 filter type 3: Sinc3 filter type 4: Sinc4 filter type 5: Sinc5 filter type 6-7: Reserved Sincx filter type transfer function: FastSinc filter type transfer function: This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORD {
    ///0: FastSinc filter type
    FastSinc = 0,
    ///1: Sinc1 filter type
    Sinc1 = 1,
    ///2: Sinc2 filter type
    Sinc2 = 2,
    ///3: Sinc3 filter type
    Sinc3 = 3,
    ///4: Sinc4 filter type
    Sinc4 = 4,
    ///5: Sinc5 filter type
    Sinc5 = 5,
}
impl From<FORD> for u8 {
    #[inline(always)]
    fn from(variant: FORD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FORD {
    type Ux = u8;
}
impl crate::IsEnum for FORD {}
///Field `FORD` reader - Sinc filter order 2: Sinc2 filter type 3: Sinc3 filter type 4: Sinc4 filter type 5: Sinc5 filter type 6-7: Reserved Sincx filter type transfer function: FastSinc filter type transfer function: This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1).
pub type FORD_R = crate::FieldReader<FORD>;
impl FORD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<FORD> {
        match self.bits {
            0 => Some(FORD::FastSinc),
            1 => Some(FORD::Sinc1),
            2 => Some(FORD::Sinc2),
            3 => Some(FORD::Sinc3),
            4 => Some(FORD::Sinc4),
            5 => Some(FORD::Sinc5),
            _ => None,
        }
    }
    ///FastSinc filter type
    #[inline(always)]
    pub fn is_fast_sinc(&self) -> bool {
        *self == FORD::FastSinc
    }
    ///Sinc1 filter type
    #[inline(always)]
    pub fn is_sinc1(&self) -> bool {
        *self == FORD::Sinc1
    }
    ///Sinc2 filter type
    #[inline(always)]
    pub fn is_sinc2(&self) -> bool {
        *self == FORD::Sinc2
    }
    ///Sinc3 filter type
    #[inline(always)]
    pub fn is_sinc3(&self) -> bool {
        *self == FORD::Sinc3
    }
    ///Sinc4 filter type
    #[inline(always)]
    pub fn is_sinc4(&self) -> bool {
        *self == FORD::Sinc4
    }
    ///Sinc5 filter type
    #[inline(always)]
    pub fn is_sinc5(&self) -> bool {
        *self == FORD::Sinc5
    }
}
///Field `FORD` writer - Sinc filter order 2: Sinc2 filter type 3: Sinc3 filter type 4: Sinc4 filter type 5: Sinc5 filter type 6-7: Reserved Sincx filter type transfer function: FastSinc filter type transfer function: This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1).
pub type FORD_W<'a, REG> = crate::FieldWriter<'a, REG, 3, FORD>;
impl<'a, REG> FORD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///FastSinc filter type
    #[inline(always)]
    pub fn fast_sinc(self) -> &'a mut crate::W<REG> {
        self.variant(FORD::FastSinc)
    }
    ///Sinc1 filter type
    #[inline(always)]
    pub fn sinc1(self) -> &'a mut crate::W<REG> {
        self.variant(FORD::Sinc1)
    }
    ///Sinc2 filter type
    #[inline(always)]
    pub fn sinc2(self) -> &'a mut crate::W<REG> {
        self.variant(FORD::Sinc2)
    }
    ///Sinc3 filter type
    #[inline(always)]
    pub fn sinc3(self) -> &'a mut crate::W<REG> {
        self.variant(FORD::Sinc3)
    }
    ///Sinc4 filter type
    #[inline(always)]
    pub fn sinc4(self) -> &'a mut crate::W<REG> {
        self.variant(FORD::Sinc4)
    }
    ///Sinc5 filter type
    #[inline(always)]
    pub fn sinc5(self) -> &'a mut crate::W<REG> {
        self.variant(FORD::Sinc5)
    }
}
impl R {
    ///Bits 0:7 - Integrator oversampling ratio (averaging length) from Sinc filter will be summed into one output data sample from the integrator. The output data rate from the integrator will be decreased by this number (additional data decimation ratio). This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If IOSR = 0, then the Integrator has no effect (Integrator bypass). 0- 255: The length of the Integrator in the range 1 - 256 (IOSR + 1). Defines how many samples
    #[inline(always)]
    pub fn iosr(&self) -> IOSR_R {
        IOSR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:25 - Sinc filter oversampling ratio (decimation rate) number is also the decimation ratio of the output data rate from filter. This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If FOSR = 0, then the filter has no effect (filter bypass). 0 - 1023: Defines the length of the Sinc type filter in the range 1 - 1024 (FOSR = FOSR\[9:0\] +1). This
    #[inline(always)]
    pub fn fosr(&self) -> FOSR_R {
        FOSR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bits 29:31 - Sinc filter order 2: Sinc2 filter type 3: Sinc3 filter type 4: Sinc4 filter type 5: Sinc5 filter type 6-7: Reserved Sincx filter type transfer function: FastSinc filter type transfer function: This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn ford(&self) -> FORD_R {
        FORD_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCR")
            .field("iosr", &self.iosr())
            .field("fosr", &self.fosr())
            .field("ford", &self.ford())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Integrator oversampling ratio (averaging length) from Sinc filter will be summed into one output data sample from the integrator. The output data rate from the integrator will be decreased by this number (additional data decimation ratio). This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If IOSR = 0, then the Integrator has no effect (Integrator bypass). 0- 255: The length of the Integrator in the range 1 - 256 (IOSR + 1). Defines how many samples
    #[inline(always)]
    pub fn iosr(&mut self) -> IOSR_W<FCRrs> {
        IOSR_W::new(self, 0)
    }
    ///Bits 16:25 - Sinc filter oversampling ratio (decimation rate) number is also the decimation ratio of the output data rate from filter. This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If FOSR = 0, then the filter has no effect (filter bypass). 0 - 1023: Defines the length of the Sinc type filter in the range 1 - 1024 (FOSR = FOSR\[9:0\] +1). This
    #[inline(always)]
    pub fn fosr(&mut self) -> FOSR_W<FCRrs> {
        FOSR_W::new(self, 16)
    }
    ///Bits 29:31 - Sinc filter order 2: Sinc2 filter type 3: Sinc3 filter type 4: Sinc4 filter type 5: Sinc5 filter type 6-7: Reserved Sincx filter type transfer function: FastSinc filter type transfer function: This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn ford(&mut self) -> FORD_W<FCRrs> {
        FORD_W::new(self, 29)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FCRrs;
impl crate::RegisterSpec for FCRrs {
    type Ux = u32;
}
///`read()` method returns [`fcr::R`](R) reader structure
impl crate::Readable for FCRrs {}
///`write(|w| ..)` method takes [`fcr::W`](W) writer structure
impl crate::Writable for FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR to value 0
impl crate::Resettable for FCRrs {}
