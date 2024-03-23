#[doc = "Register `NDTR` reader"]
pub type R = crate::R<NDTRrs>;
#[doc = "Register `NDTR` writer"]
pub type W = crate::W<NDTRrs>;
#[doc = "Field `NDT` reader - Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not."]
pub type NDT_R = crate::FieldReader<u16>;
#[doc = "Field `NDT` writer - Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not."]
pub type NDT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not."]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not."]
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NDT_W<NDTRrs> {
        NDT_W::new(self, 0)
    }
}
#[doc = "DMA channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ndtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ndtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NDTRrs;
impl crate::RegisterSpec for NDTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ndtr::R`](R) reader structure"]
impl crate::Readable for NDTRrs {}
#[doc = "`write(|w| ..)` method takes [`ndtr::W`](W) writer structure"]
impl crate::Writable for NDTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NDTR to value 0"]
impl crate::Resettable for NDTRrs {
    const RESET_VALUE: u32 = 0;
}
