///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `CLKWISE_STATE` reader - The current state of the LCSC clockwise FSM:
pub type CLKWISE_STATE_R = crate::FieldReader;
///Field `ACLKWISE_STATE` reader - The current state of the LCSC anti clockwise FSM:
pub type ACLKWISE_STATE_R = crate::FieldReader;
///Field `LAST_DIR` reader - The last direction detected:
pub type LAST_DIR_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - The current state of the LCSC clockwise FSM:
    #[inline(always)]
    pub fn clkwise_state(&self) -> CLKWISE_STATE_R {
        CLKWISE_STATE_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - The current state of the LCSC anti clockwise FSM:
    #[inline(always)]
    pub fn aclkwise_state(&self) -> ACLKWISE_STATE_R {
        ACLKWISE_STATE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - The last direction detected:
    #[inline(always)]
    pub fn last_dir(&self) -> LAST_DIR_R {
        LAST_DIR_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("clkwise_state", &self.clkwise_state())
            .field("aclkwise_state", &self.aclkwise_state())
            .field("last_dir", &self.last_dir())
            .finish()
    }
}
/**LCSC_SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
