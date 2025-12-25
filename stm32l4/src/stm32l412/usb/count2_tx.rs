///Register `COUNT2_TX` reader
pub type R = crate::R<COUNT2_TXrs>;
///Register `COUNT2_TX` writer
pub type W = crate::W<COUNT2_TXrs>;
///Field `COUNT2_TX` reader - Transmission byte count
pub type COUNT2_TX_R = crate::FieldReader<u16>;
///Field `COUNT2_TX` writer - Transmission byte count
pub type COUNT2_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Transmission byte count
    #[inline(always)]
    pub fn count2_tx(&self) -> COUNT2_TX_R {
        COUNT2_TX_R::new(self.bits & 0x03ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COUNT2_TX")
            .field("count2_tx", &self.count2_tx())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Transmission byte count
    #[inline(always)]
    pub fn count2_tx(&mut self) -> COUNT2_TX_W<'_, COUNT2_TXrs> {
        COUNT2_TX_W::new(self, 0)
    }
}
/**Transmission byte count 0

You can [`read`](crate::Reg::read) this register and get [`count2_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count2_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#USB:COUNT2_TX)*/
pub struct COUNT2_TXrs;
impl crate::RegisterSpec for COUNT2_TXrs {
    type Ux = u16;
}
///`read()` method returns [`count2_tx::R`](R) reader structure
impl crate::Readable for COUNT2_TXrs {}
///`write(|w| ..)` method takes [`count2_tx::W`](W) writer structure
impl crate::Writable for COUNT2_TXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COUNT2_TX to value 0
impl crate::Resettable for COUNT2_TXrs {}
