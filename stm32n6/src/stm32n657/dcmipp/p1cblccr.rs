///Register `P1CBLCCR` reader
pub type R = crate::R<P1CBLCCRrs>;
///Field `ENABLE` reader - For current black level calibration
pub type ENABLE_R = crate::BitReader;
///Field `BLCB` reader - Current black level calibration - Blue
pub type BLCB_R = crate::FieldReader;
///Field `BLCG` reader - Current black level calibration - Green
pub type BLCG_R = crate::FieldReader;
///Field `BLCR` reader - Current black level calibration - Red
pub type BLCR_R = crate::FieldReader;
impl R {
    ///Bit 0 - For current black level calibration
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:15 - Current black level calibration - Blue
    #[inline(always)]
    pub fn blcb(&self) -> BLCB_R {
        BLCB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Current black level calibration - Green
    #[inline(always)]
    pub fn blcg(&self) -> BLCG_R {
        BLCG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Current black level calibration - Red
    #[inline(always)]
    pub fn blcr(&self) -> BLCR_R {
        BLCR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CBLCCR")
            .field("enable", &self.enable())
            .field("blcb", &self.blcb())
            .field("blcg", &self.blcg())
            .field("blcr", &self.blcr())
            .finish()
    }
}
/**DCMIPP Pipe1 current black level calibration control register

You can [`read`](crate::Reg::read) this register and get [`p1cblccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CBLCCR)*/
pub struct P1CBLCCRrs;
impl crate::RegisterSpec for P1CBLCCRrs {
    type Ux = u32;
}
///`read()` method returns [`p1cblccr::R`](R) reader structure
impl crate::Readable for P1CBLCCRrs {}
///`reset()` method sets P1CBLCCR to value 0
impl crate::Resettable for P1CBLCCRrs {}
