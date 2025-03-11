///Register `UR13` reader
pub type R = crate::R<UR13rs>;
///Field `SDRS` reader - Secured DTCM RAM Size
pub type SDRS_R = crate::FieldReader;
///Field `D1SBRST` reader - D1 Standby reset
pub type D1SBRST_R = crate::BitReader;
impl R {
    ///Bits 0:1 - Secured DTCM RAM Size
    #[inline(always)]
    pub fn sdrs(&self) -> SDRS_R {
        SDRS_R::new((self.bits & 3) as u8)
    }
    ///Bit 16 - D1 Standby reset
    #[inline(always)]
    pub fn d1sbrst(&self) -> D1SBRST_R {
        D1SBRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR13")
            .field("sdrs", &self.sdrs())
            .field("d1sbrst", &self.d1sbrst())
            .finish()
    }
}
/**SYSCFG user register 13

You can [`read`](crate::Reg::read) this register and get [`ur13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#SYSCFG:UR13)*/
pub struct UR13rs;
impl crate::RegisterSpec for UR13rs {
    type Ux = u32;
}
///`read()` method returns [`ur13::R`](R) reader structure
impl crate::Readable for UR13rs {}
///`reset()` method sets UR13 to value 0
impl crate::Resettable for UR13rs {}
