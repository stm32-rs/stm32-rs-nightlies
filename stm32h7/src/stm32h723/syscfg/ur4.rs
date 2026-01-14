///Register `UR4` reader
pub type R = crate::R<UR4rs>;
///Field `MEPAD_1` reader - Mass Erase Protected Area Disabled for bank 1
pub type MEPAD_1_R = crate::BitReader;
impl R {
    ///Bit 16 - Mass Erase Protected Area Disabled for bank 1
    #[inline(always)]
    pub fn mepad_1(&self) -> MEPAD_1_R {
        MEPAD_1_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR4")
            .field("mepad_1", &self.mepad_1())
            .finish()
    }
}
/**SYSCFG user register 4

You can [`read`](crate::Reg::read) this register and get [`ur4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#SYSCFG:UR4)*/
pub struct UR4rs;
impl crate::RegisterSpec for UR4rs {
    type Ux = u32;
}
///`read()` method returns [`ur4::R`](R) reader structure
impl crate::Readable for UR4rs {}
///`reset()` method sets UR4 to value 0
impl crate::Resettable for UR4rs {}
