///Register `TSR` reader
pub type R = crate::R<TSRrs>;
///Field `AFCS` reader - absolute frame counter status
pub type AFCS_R = crate::BitReader;
///Field `ALCS` reader - absolute line counter status
pub type ALCS_R = crate::BitReader;
///Field `RFC1S` reader - relative frame counter 1 status
pub type RFC1S_R = crate::BitReader;
///Field `RFC2S` reader - relative frame counter 2 status
pub type RFC2S_R = crate::BitReader;
impl R {
    ///Bit 0 - absolute frame counter status
    #[inline(always)]
    pub fn afcs(&self) -> AFCS_R {
        AFCS_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - absolute line counter status
    #[inline(always)]
    pub fn alcs(&self) -> ALCS_R {
        ALCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 16 - relative frame counter 1 status
    #[inline(always)]
    pub fn rfc1s(&self) -> RFC1S_R {
        RFC1S_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - relative frame counter 2 status
    #[inline(always)]
    pub fn rfc2s(&self) -> RFC2S_R {
        RFC2S_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSR")
            .field("afcs", &self.afcs())
            .field("alcs", &self.alcs())
            .field("rfc1s", &self.rfc1s())
            .field("rfc2s", &self.rfc2s())
            .finish()
    }
}
/**GFXTIM timers status register

You can [`read`](crate::Reg::read) this register and get [`tsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GFXTIM:TSR)*/
pub struct TSRrs;
impl crate::RegisterSpec for TSRrs {
    type Ux = u32;
}
///`read()` method returns [`tsr::R`](R) reader structure
impl crate::Readable for TSRrs {}
///`reset()` method sets TSR to value 0
impl crate::Resettable for TSRrs {}
