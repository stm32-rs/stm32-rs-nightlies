///Register `BCHDSR0` reader
pub type R = crate::R<BCHDSR0rs>;
///Field `DUE` reader - Decoder uncorrectable error
pub type DUE_R = crate::BitReader;
///Field `DEF` reader - Decoder error found
pub type DEF_R = crate::BitReader;
///Field `DEN` reader - Decoder error number
pub type DEN_R = crate::FieldReader;
impl R {
    ///Bit 0 - Decoder uncorrectable error
    #[inline(always)]
    pub fn due(&self) -> DUE_R {
        DUE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Decoder error found
    #[inline(always)]
    pub fn def(&self) -> DEF_R {
        DEF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 4:7 - Decoder error number
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCHDSR0")
            .field("due", &self.due())
            .field("def", &self.def())
            .field("den", &self.den())
            .finish()
    }
}
/**FMC BCH decoder status register 0

You can [`read`](crate::Reg::read) this register and get [`bchdsr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#FMC1:BCHDSR0)*/
pub struct BCHDSR0rs;
impl crate::RegisterSpec for BCHDSR0rs {
    type Ux = u32;
}
///`read()` method returns [`bchdsr0::R`](R) reader structure
impl crate::Readable for BCHDSR0rs {}
///`reset()` method sets BCHDSR0 to value 0
impl crate::Resettable for BCHDSR0rs {}
