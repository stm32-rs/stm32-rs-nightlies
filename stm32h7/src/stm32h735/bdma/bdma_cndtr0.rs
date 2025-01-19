///Register `BDMA_CNDTR0` reader
pub type R = crate::R<BDMA_CNDTR0rs>;
///Register `BDMA_CNDTR0` writer
pub type W = crate::W<BDMA_CNDTR0rs>;
///Field `NDT` reader - number of data to transfer (0 to 216 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single BDMA ‘read followed by write’ transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the BDMA_CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not). Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
pub type NDT_R = crate::FieldReader<u16>;
///Field `NDT` writer - number of data to transfer (0 to 216 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single BDMA ‘read followed by write’ transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the BDMA_CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not). Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
pub type NDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - number of data to transfer (0 to 216 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single BDMA ‘read followed by write’ transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the BDMA_CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not). Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDMA_CNDTR0")
            .field("ndt", &self.ndt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - number of data to transfer (0 to 216 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single BDMA ‘read followed by write’ transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the BDMA_CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not). Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W<BDMA_CNDTR0rs> {
        NDT_W::new(self, 0)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`bdma_cndtr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_cndtr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_CNDTR0)*/
pub struct BDMA_CNDTR0rs;
impl crate::RegisterSpec for BDMA_CNDTR0rs {
    type Ux = u32;
}
///`read()` method returns [`bdma_cndtr0::R`](R) reader structure
impl crate::Readable for BDMA_CNDTR0rs {}
///`write(|w| ..)` method takes [`bdma_cndtr0::W`](W) writer structure
impl crate::Writable for BDMA_CNDTR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BDMA_CNDTR0 to value 0
impl crate::Resettable for BDMA_CNDTR0rs {
    const RESET_VALUE: u32 = 0;
}
