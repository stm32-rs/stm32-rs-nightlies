///Register `STGENR_PIDR0` reader
pub type R = crate::R<STGENR_PIDR0rs>;
///Field `PART_0` reader - PART_0
pub type PART_0_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PART_0
    #[inline(always)]
    pub fn part_0(&self) -> PART_0_R {
        PART_0_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STGENR_PIDR0")
            .field("part_0", &self.part_0())
            .finish()
    }
}
/**STGENR peripheral ID0 register

You can [`read`](crate::Reg::read) this register and get [`stgenr_pidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENR:STGENR_PIDR0)*/
pub struct STGENR_PIDR0rs;
impl crate::RegisterSpec for STGENR_PIDR0rs {
    type Ux = u32;
}
///`read()` method returns [`stgenr_pidr0::R`](R) reader structure
impl crate::Readable for STGENR_PIDR0rs {}
///`reset()` method sets STGENR_PIDR0 to value 0x01
impl crate::Resettable for STGENR_PIDR0rs {
    const RESET_VALUE: u32 = 0x01;
}
