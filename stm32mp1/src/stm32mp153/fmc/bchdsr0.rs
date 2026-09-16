///Register `BCHDSR0` reader
pub type R = crate::R<BCHDSR0rs>;
///Field `DUE` reader - DUE
pub type DUE_R = crate::BitReader;
///Field `DEF` reader - DEF
pub type DEF_R = crate::BitReader;
///Field `DEN` reader - DEN
pub type DEN_R = crate::FieldReader;
impl R {
    ///Bit 0 - DUE
    #[inline(always)]
    pub fn due(&self) -> DUE_R {
        DUE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DEF
    #[inline(always)]
    pub fn def(&self) -> DEF_R {
        DEF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 4:7 - DEN
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
/**This register contains some fields already available in other registers but that require to be saved when error correction is performed on several sectors at a time (for example a whole NAND Flash page). This allows a DMA channel to transfer the content of FMC_BCHDSR0..4 to a decoding status buffer. .

You can [`read`](crate::Reg::read) this register and get [`bchdsr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCHDSR0)*/
pub struct BCHDSR0rs;
impl crate::RegisterSpec for BCHDSR0rs {
    type Ux = u32;
}
///`read()` method returns [`bchdsr0::R`](R) reader structure
impl crate::Readable for BCHDSR0rs {}
///`reset()` method sets BCHDSR0 to value 0
impl crate::Resettable for BCHDSR0rs {}
