///Register `UR15` reader
pub type R = crate::R<UR15rs>;
///Field `FZIWDGSTB` reader - Freeze independent watchdog in Standby mode
pub type FZIWDGSTB_R = crate::BitReader;
impl R {
    ///Bit 16 - Freeze independent watchdog in Standby mode
    #[inline(always)]
    pub fn fziwdgstb(&self) -> FZIWDGSTB_R {
        FZIWDGSTB_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR15")
            .field("fziwdgstb", &self.fziwdgstb())
            .finish()
    }
}
/**SYSCFG user register 15

You can [`read`](crate::Reg::read) this register and get [`ur15::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#SYSCFG:UR15)*/
pub struct UR15rs;
impl crate::RegisterSpec for UR15rs {
    type Ux = u32;
}
///`read()` method returns [`ur15::R`](R) reader structure
impl crate::Readable for UR15rs {}
///`reset()` method sets UR15 to value 0
impl crate::Resettable for UR15rs {}
