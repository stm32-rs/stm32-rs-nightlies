///Register `CNDTR3` reader
pub type R = crate::R<CNDTR3rs>;
///Register `CNDTR3` writer
pub type W = crate::W<CNDTR3rs>;
///Field `NDT` reader - NDT\[15:0\]: Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not.
pub type NDT_R = crate::FieldReader<u16>;
///Field `NDT` writer - NDT\[15:0\]: Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not.
pub type NDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - NDT\[15:0\]: Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not.
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNDTR3").field("ndt", &self.ndt()).finish()
    }
}
impl W {
    ///Bits 0:15 - NDT\[15:0\]: Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not.
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W<'_, CNDTR3rs> {
        NDT_W::new(self, 0)
    }
}
/**DMA_CNDTRx register

You can [`read`](crate::Reg::read) this register and get [`cndtr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CNDTR3)*/
pub struct CNDTR3rs;
impl crate::RegisterSpec for CNDTR3rs {
    type Ux = u32;
}
///`read()` method returns [`cndtr3::R`](R) reader structure
impl crate::Readable for CNDTR3rs {}
///`write(|w| ..)` method takes [`cndtr3::W`](W) writer structure
impl crate::Writable for CNDTR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CNDTR3 to value 0
impl crate::Resettable for CNDTR3rs {}
