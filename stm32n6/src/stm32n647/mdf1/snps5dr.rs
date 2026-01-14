///Register `SNPS5DR` reader
pub type R = crate::R<SNPS5DRrs>;
///Field `MCICDC` reader - Contains the MCIC decimation counter value when the last trigger event occurs (MCIC_CNT)
pub type MCICDC_R = crate::FieldReader<u16>;
///Field `EXTSDR` reader - Extended data size
pub type EXTSDR_R = crate::FieldReader;
///Field `SDR` reader - Contains the 16 MSB of the last valid data processed by the digital filter.
pub type SDR_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:8 - Contains the MCIC decimation counter value when the last trigger event occurs (MCIC_CNT)
    #[inline(always)]
    pub fn mcicdc(&self) -> MCICDC_R {
        MCICDC_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:15 - Extended data size
    #[inline(always)]
    pub fn extsdr(&self) -> EXTSDR_R {
        EXTSDR_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    ///Bits 16:31 - Contains the 16 MSB of the last valid data processed by the digital filter.
    #[inline(always)]
    pub fn sdr(&self) -> SDR_R {
        SDR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SNPS5DR")
            .field("mcicdc", &self.mcicdc())
            .field("extsdr", &self.extsdr())
            .field("sdr", &self.sdr())
            .finish()
    }
}
/**MDF snapshot data register 5

You can [`read`](crate::Reg::read) this register and get [`snps5dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#MDF1:SNPS5DR)*/
pub struct SNPS5DRrs;
impl crate::RegisterSpec for SNPS5DRrs {
    type Ux = u32;
}
///`read()` method returns [`snps5dr::R`](R) reader structure
impl crate::Readable for SNPS5DRrs {}
///`reset()` method sets SNPS5DR to value 0
impl crate::Resettable for SNPS5DRrs {}
