///Register `PSSI_IER` reader
pub type R = crate::R<PSSI_IERrs>;
///Register `PSSI_IER` writer
pub type W = crate::W<PSSI_IERrs>;
///Field `OVR_IE` reader - Data buffer overrun/underrun interrupt enable
pub type OVR_IE_R = crate::BitReader;
///Field `OVR_IE` writer - Data buffer overrun/underrun interrupt enable
pub type OVR_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Data buffer overrun/underrun interrupt enable
    #[inline(always)]
    pub fn ovr_ie(&self) -> OVR_IE_R {
        OVR_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSSI_IER")
            .field("ovr_ie", &self.ovr_ie())
            .finish()
    }
}
impl W {
    ///Bit 1 - Data buffer overrun/underrun interrupt enable
    #[inline(always)]
    pub fn ovr_ie(&mut self) -> OVR_IE_W<PSSI_IERrs> {
        OVR_IE_W::new(self, 1)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`pssi_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pssi_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#PSSI:PSSI_IER)*/
pub struct PSSI_IERrs;
impl crate::RegisterSpec for PSSI_IERrs {
    type Ux = u32;
}
///`read()` method returns [`pssi_ier::R`](R) reader structure
impl crate::Readable for PSSI_IERrs {}
///`write(|w| ..)` method takes [`pssi_ier::W`](W) writer structure
impl crate::Writable for PSSI_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PSSI_IER to value 0
impl crate::Resettable for PSSI_IERrs {
    const RESET_VALUE: u32 = 0;
}
