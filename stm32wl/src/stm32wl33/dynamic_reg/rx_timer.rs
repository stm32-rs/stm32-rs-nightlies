///Register `RX_TIMER` reader
pub type R = crate::R<RX_TIMERrs>;
///Register `RX_TIMER` writer
pub type W = crate::W<RX_TIMERrs>;
///Field `RX_TIMEOUT` reader - RX timer timeout (relative duration in interpolated absolute time unit)
pub type RX_TIMEOUT_R = crate::FieldReader<u32>;
///Field `RX_TIMEOUT` writer - RX timer timeout (relative duration in interpolated absolute time unit)
pub type RX_TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
///Field `RX_CS_TIMEOUT_MASK` reader - - 0: CS flag does not contribute to timeout disabling
pub type RX_CS_TIMEOUT_MASK_R = crate::BitReader;
///Field `RX_CS_TIMEOUT_MASK` writer - - 0: CS flag does not contribute to timeout disabling
pub type RX_CS_TIMEOUT_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_PQI_TIMEOUT_MASK` reader - - 0: PREAMBLE valid flag does not contribute to timeout disabling
pub type RX_PQI_TIMEOUT_MASK_R = crate::BitReader;
///Field `RX_PQI_TIMEOUT_MASK` writer - - 0: PREAMBLE valid flag does not contribute to timeout disabling
pub type RX_PQI_TIMEOUT_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_SQI_TIMEOUT_MASK` reader - - 0: SYNC valid flag does not contribute to timeout disabling
pub type RX_SQI_TIMEOUT_MASK_R = crate::BitReader;
///Field `RX_SQI_TIMEOUT_MASK` writer - - 0: SYNC valid flag does not contribute to timeout disabling
pub type RX_SQI_TIMEOUT_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_OR_nAND_SELECT` reader - Select logical OR or logcial AND to apply on CS/PQI/SQI timeout mask
pub type RX_OR_N_AND_SELECT_R = crate::BitReader;
///Field `RX_OR_nAND_SELECT` writer - Select logical OR or logcial AND to apply on CS/PQI/SQI timeout mask
pub type RX_OR_N_AND_SELECT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:22 - RX timer timeout (relative duration in interpolated absolute time unit)
    #[inline(always)]
    pub fn rx_timeout(&self) -> RX_TIMEOUT_R {
        RX_TIMEOUT_R::new(self.bits & 0x007f_ffff)
    }
    ///Bit 28 - - 0: CS flag does not contribute to timeout disabling
    #[inline(always)]
    pub fn rx_cs_timeout_mask(&self) -> RX_CS_TIMEOUT_MASK_R {
        RX_CS_TIMEOUT_MASK_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - - 0: PREAMBLE valid flag does not contribute to timeout disabling
    #[inline(always)]
    pub fn rx_pqi_timeout_mask(&self) -> RX_PQI_TIMEOUT_MASK_R {
        RX_PQI_TIMEOUT_MASK_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - - 0: SYNC valid flag does not contribute to timeout disabling
    #[inline(always)]
    pub fn rx_sqi_timeout_mask(&self) -> RX_SQI_TIMEOUT_MASK_R {
        RX_SQI_TIMEOUT_MASK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Select logical OR or logcial AND to apply on CS/PQI/SQI timeout mask
    #[inline(always)]
    pub fn rx_or_n_and_select(&self) -> RX_OR_N_AND_SELECT_R {
        RX_OR_N_AND_SELECT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_TIMER")
            .field("rx_timeout", &self.rx_timeout())
            .field("rx_cs_timeout_mask", &self.rx_cs_timeout_mask())
            .field("rx_pqi_timeout_mask", &self.rx_pqi_timeout_mask())
            .field("rx_sqi_timeout_mask", &self.rx_sqi_timeout_mask())
            .field("rx_or_n_and_select", &self.rx_or_n_and_select())
            .finish()
    }
}
impl W {
    ///Bits 0:22 - RX timer timeout (relative duration in interpolated absolute time unit)
    #[inline(always)]
    pub fn rx_timeout(&mut self) -> RX_TIMEOUT_W<'_, RX_TIMERrs> {
        RX_TIMEOUT_W::new(self, 0)
    }
    ///Bit 28 - - 0: CS flag does not contribute to timeout disabling
    #[inline(always)]
    pub fn rx_cs_timeout_mask(&mut self) -> RX_CS_TIMEOUT_MASK_W<'_, RX_TIMERrs> {
        RX_CS_TIMEOUT_MASK_W::new(self, 28)
    }
    ///Bit 29 - - 0: PREAMBLE valid flag does not contribute to timeout disabling
    #[inline(always)]
    pub fn rx_pqi_timeout_mask(&mut self) -> RX_PQI_TIMEOUT_MASK_W<'_, RX_TIMERrs> {
        RX_PQI_TIMEOUT_MASK_W::new(self, 29)
    }
    ///Bit 30 - - 0: SYNC valid flag does not contribute to timeout disabling
    #[inline(always)]
    pub fn rx_sqi_timeout_mask(&mut self) -> RX_SQI_TIMEOUT_MASK_W<'_, RX_TIMERrs> {
        RX_SQI_TIMEOUT_MASK_W::new(self, 30)
    }
    ///Bit 31 - Select logical OR or logcial AND to apply on CS/PQI/SQI timeout mask
    #[inline(always)]
    pub fn rx_or_n_and_select(&mut self) -> RX_OR_N_AND_SELECT_W<'_, RX_TIMERrs> {
        RX_OR_N_AND_SELECT_W::new(self, 31)
    }
}
/**RX_TIMER register

You can [`read`](crate::Reg::read) this register and get [`rx_timer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_timer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:RX_TIMER)*/
pub struct RX_TIMERrs;
impl crate::RegisterSpec for RX_TIMERrs {
    type Ux = u32;
}
///`read()` method returns [`rx_timer::R`](R) reader structure
impl crate::Readable for RX_TIMERrs {}
///`write(|w| ..)` method takes [`rx_timer::W`](W) writer structure
impl crate::Writable for RX_TIMERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RX_TIMER to value 0
impl crate::Resettable for RX_TIMERrs {}
