///Register `STGENR_PIDR1` reader
pub type R = crate::R<STGENR_PIDR1rs>;
///Field `PART_1` reader - PART_1
pub type PART_1_R = crate::FieldReader;
///Field `DES_0` reader - DES_0
pub type DES_0_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - PART_1
    #[inline(always)]
    pub fn part_1(&self) -> PART_1_R {
        PART_1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - DES_0
    #[inline(always)]
    pub fn des_0(&self) -> DES_0_R {
        DES_0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STGENR_PIDR1")
            .field("part_1", &self.part_1())
            .field("des_0", &self.des_0())
            .finish()
    }
}
/**STGENR peripheral ID1 register

You can [`read`](crate::Reg::read) this register and get [`stgenr_pidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENR:STGENR_PIDR1)*/
pub struct STGENR_PIDR1rs;
impl crate::RegisterSpec for STGENR_PIDR1rs {
    type Ux = u32;
}
///`read()` method returns [`stgenr_pidr1::R`](R) reader structure
impl crate::Readable for STGENR_PIDR1rs {}
///`reset()` method sets STGENR_PIDR1 to value 0xb1
impl crate::Resettable for STGENR_PIDR1rs {
    const RESET_VALUE: u32 = 0xb1;
}
