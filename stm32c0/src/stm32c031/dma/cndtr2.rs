#[doc = "Register `CNDTR2` reader"]
pub type R = crate::R<CNDTR2rs>;
#[doc = "Register `CNDTR2` writer"]
pub type W = crate::W<CNDTR2rs>;
#[doc = "Field `NDT` reader - number of data to transfer (0 to 216 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single DMA ‘read followed by write’ transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the DMA_CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not). Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
pub type NDT_R = crate::FieldReader<u16>;
#[doc = "Field `NDT` writer - number of data to transfer (0 to 216 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single DMA ‘read followed by write’ transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the DMA_CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not). Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
pub type NDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - number of data to transfer (0 to 216 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single DMA ‘read followed by write’ transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the DMA_CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not). Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - number of data to transfer (0 to 216 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single DMA ‘read followed by write’ transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the DMA_CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not). Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NDT_W<CNDTR2rs> {
        NDT_W::new(self, 0)
    }
}
#[doc = "DMA channel 2 number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNDTR2rs;
impl crate::RegisterSpec for CNDTR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cndtr2::R`](R) reader structure"]
impl crate::Readable for CNDTR2rs {}
#[doc = "`write(|w| ..)` method takes [`cndtr2::W`](W) writer structure"]
impl crate::Writable for CNDTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNDTR2 to value 0"]
impl crate::Resettable for CNDTR2rs {
    const RESET_VALUE: u32 = 0;
}
