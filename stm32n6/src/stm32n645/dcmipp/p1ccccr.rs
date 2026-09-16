///Register `P1CCCCR` reader
pub type R = crate::R<P1CCCCRrs>;
///Field `ENABLE` reader - Current value applied
pub type ENABLE_R = crate::BitReader;
///Field `TYPE` reader - Output samples type used while CLAMP is activated
pub type TYPE_R = crate::BitReader;
///Field `CLAMP` reader - Clamp the output samples
pub type CLAMP_R = crate::BitReader;
impl R {
    ///Bit 0 - Current value applied
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Output samples type used while CLAMP is activated
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clamp the output samples
    #[inline(always)]
    pub fn clamp(&self) -> CLAMP_R {
        CLAMP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CCCCR")
            .field("enable", &self.enable())
            .field("type_", &self.type_())
            .field("clamp", &self.clamp())
            .finish()
    }
}
/**DCMIPP Pipe1 current ColorConv configuration register

You can [`read`](crate::Reg::read) this register and get [`p1ccccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P1CCCCR)*/
pub struct P1CCCCRrs;
impl crate::RegisterSpec for P1CCCCRrs {
    type Ux = u32;
}
///`read()` method returns [`p1ccccr::R`](R) reader structure
impl crate::Readable for P1CCCCRrs {}
///`reset()` method sets P1CCCCR to value 0
impl crate::Resettable for P1CCCCRrs {}
