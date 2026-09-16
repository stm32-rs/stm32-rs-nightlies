///Register `UR12` reader
pub type R = crate::R<UR12rs>;
///Field `SECURE` reader - Secure mode
pub type SECURE_R = crate::BitReader;
impl R {
    ///Bit 16 - Secure mode
    #[inline(always)]
    pub fn secure(&self) -> SECURE_R {
        SECURE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR12")
            .field("secure", &self.secure())
            .finish()
    }
}
/**SYSCFG user register 12

You can [`read`](crate::Reg::read) this register and get [`ur12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#SYSCFG:UR12)*/
pub struct UR12rs;
impl crate::RegisterSpec for UR12rs {
    type Ux = u32;
}
///`read()` method returns [`ur12::R`](R) reader structure
impl crate::Readable for UR12rs {}
///`reset()` method sets UR12 to value 0
impl crate::Resettable for UR12rs {}
