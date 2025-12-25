///Register `CM55PAHBWPR` reader
pub type R = crate::R<CM55PAHBWPRrs>;
///Register `CM55PAHBWPR` writer
pub type W = crate::W<CM55PAHBWPRrs>;
///Field `PAHB_ERROR_ACK` reader - Error capture in write posting buffer
pub type PAHB_ERROR_ACK_R = crate::BitReader;
///Field `PAHB_ERROR_ACK` writer - Error capture in write posting buffer
pub type PAHB_ERROR_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Error capture in write posting buffer
    #[inline(always)]
    pub fn pahb_error_ack(&self) -> PAHB_ERROR_ACK_R {
        PAHB_ERROR_ACK_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM55PAHBWPR")
            .field("pahb_error_ack", &self.pahb_error_ack())
            .finish()
    }
}
impl W {
    ///Bit 0 - Error capture in write posting buffer
    #[inline(always)]
    pub fn pahb_error_ack(&mut self) -> PAHB_ERROR_ACK_W<'_, CM55PAHBWPRrs> {
        PAHB_ERROR_ACK_W::new(self, 0)
    }
}
/**SYSCFG Cortex-M55 P-AHB write posting control register

You can [`read`](crate::Reg::read) this register and get [`cm55pahbwpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm55pahbwpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#SYSCFG:CM55PAHBWPR)*/
pub struct CM55PAHBWPRrs;
impl crate::RegisterSpec for CM55PAHBWPRrs {
    type Ux = u32;
}
///`read()` method returns [`cm55pahbwpr::R`](R) reader structure
impl crate::Readable for CM55PAHBWPRrs {}
///`write(|w| ..)` method takes [`cm55pahbwpr::W`](W) writer structure
impl crate::Writable for CM55PAHBWPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CM55PAHBWPR to value 0
impl crate::Resettable for CM55PAHBWPRrs {}
