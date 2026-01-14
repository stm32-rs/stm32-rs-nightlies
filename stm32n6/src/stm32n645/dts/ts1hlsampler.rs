///Register `TS1HLSAMPLER` reader
pub type R = crate::R<TS1HLSAMPLERrs>;
///Field `SMPL_LO` reader - Lowest valid data sample value received
pub type SMPL_LO_R = crate::FieldReader<u16>;
///Field `SMPL_HI` reader - Highest valid data sample value received
pub type SMPL_HI_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Lowest valid data sample value received
    #[inline(always)]
    pub fn smpl_lo(&self) -> SMPL_LO_R {
        SMPL_LO_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Highest valid data sample value received
    #[inline(always)]
    pub fn smpl_hi(&self) -> SMPL_HI_R {
        SMPL_HI_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TS1HLSAMPLER")
            .field("smpl_lo", &self.smpl_lo())
            .field("smpl_hi", &self.smpl_hi())
            .finish()
    }
}
/**DTS TS1 high/low sample register

You can [`read`](crate::Reg::read) this register and get [`ts1hlsampler::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DTS:TS1HLSAMPLER)*/
pub struct TS1HLSAMPLERrs;
impl crate::RegisterSpec for TS1HLSAMPLERrs {
    type Ux = u32;
}
///`read()` method returns [`ts1hlsampler::R`](R) reader structure
impl crate::Readable for TS1HLSAMPLERrs {}
///`reset()` method sets TS1HLSAMPLER to value 0xffff
impl crate::Resettable for TS1HLSAMPLERrs {
    const RESET_VALUE: u32 = 0xffff;
}
