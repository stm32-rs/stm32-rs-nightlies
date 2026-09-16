///Register `UR16` reader
pub type R = crate::R<UR16rs>;
///Field `FZIWDGSTP` reader - Freeze independent watchdog in Stop mode
pub type FZIWDGSTP_R = crate::BitReader;
///Field `PKP` reader - Private key programmed
pub type PKP_R = crate::BitReader;
impl R {
    ///Bit 0 - Freeze independent watchdog in Stop mode
    #[inline(always)]
    pub fn fziwdgstp(&self) -> FZIWDGSTP_R {
        FZIWDGSTP_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - Private key programmed
    #[inline(always)]
    pub fn pkp(&self) -> PKP_R {
        PKP_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR16")
            .field("fziwdgstp", &self.fziwdgstp())
            .field("pkp", &self.pkp())
            .finish()
    }
}
/**SYSCFG user register 16

You can [`read`](crate::Reg::read) this register and get [`ur16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#SYSCFG:UR16)*/
pub struct UR16rs;
impl crate::RegisterSpec for UR16rs {
    type Ux = u32;
}
///`read()` method returns [`ur16::R`](R) reader structure
impl crate::Readable for UR16rs {}
///`reset()` method sets UR16 to value 0
impl crate::Resettable for UR16rs {}
