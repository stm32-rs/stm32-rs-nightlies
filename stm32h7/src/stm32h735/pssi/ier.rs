///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
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
        f.debug_struct("IER")
            .field("ovr_ie", &self.ovr_ie())
            .finish()
    }
}
impl W {
    ///Bit 1 - Data buffer overrun/underrun interrupt enable
    #[inline(always)]
    pub fn ovr_ie(&mut self) -> OVR_IE_W<IERrs> {
        OVR_IE_W::new(self, 1)
    }
}
/**PSSI interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#PSSI:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
