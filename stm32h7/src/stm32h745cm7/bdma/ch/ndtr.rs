///Register `NDTR` reader
pub type R = crate::R<NDTRrs>;
///Register `NDTR` writer
pub type W = crate::W<NDTRrs>;
///Field `NDT` reader - Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not.
pub type NDT_R = crate::FieldReader<u16>;
///Field `NDT` writer - Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not.
pub type NDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not.
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NDTR").field("ndt", &self.ndt()).finish()
    }
}
impl W {
    ///Bits 0:15 - Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not.
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W<'_, NDTRrs> {
        NDT_W::new(self, 0)
    }
}
/**DMA channel x number of data register

You can [`read`](crate::Reg::read) this register and get [`ndtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NDTRrs;
impl crate::RegisterSpec for NDTRrs {
    type Ux = u32;
}
///`read()` method returns [`ndtr::R`](R) reader structure
impl crate::Readable for NDTRrs {}
///`write(|w| ..)` method takes [`ndtr::W`](W) writer structure
impl crate::Writable for NDTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NDTR to value 0
impl crate::Resettable for NDTRrs {}
