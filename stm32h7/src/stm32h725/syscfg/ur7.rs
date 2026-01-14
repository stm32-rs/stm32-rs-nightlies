///Register `UR7` reader
pub type R = crate::R<UR7rs>;
///Field `SA_BEG_1` reader - Secured area start address for bank 1
pub type SA_BEG_1_R = crate::FieldReader<u16>;
///Field `SA_END_1` reader - Secured area end address for bank 1
pub type SA_END_1_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - Secured area start address for bank 1
    #[inline(always)]
    pub fn sa_beg_1(&self) -> SA_BEG_1_R {
        SA_BEG_1_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Secured area end address for bank 1
    #[inline(always)]
    pub fn sa_end_1(&self) -> SA_END_1_R {
        SA_END_1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR7")
            .field("sa_beg_1", &self.sa_beg_1())
            .field("sa_end_1", &self.sa_end_1())
            .finish()
    }
}
/**SYSCFG user register 7

You can [`read`](crate::Reg::read) this register and get [`ur7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#SYSCFG:UR7)*/
pub struct UR7rs;
impl crate::RegisterSpec for UR7rs {
    type Ux = u32;
}
///`read()` method returns [`ur7::R`](R) reader structure
impl crate::Readable for UR7rs {}
///`reset()` method sets UR7 to value 0
impl crate::Resettable for UR7rs {}
