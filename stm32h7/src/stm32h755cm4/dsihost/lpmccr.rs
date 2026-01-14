///Register `LPMCCR` reader
pub type R = crate::R<LPMCCRrs>;
///Field `VLPSIZE` reader - VACT largest packet size
pub type VLPSIZE_R = crate::FieldReader;
///Field `LPSIZE` reader - Largest packet size
pub type LPSIZE_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - VACT largest packet size
    #[inline(always)]
    pub fn vlpsize(&self) -> VLPSIZE_R {
        VLPSIZE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - Largest packet size
    #[inline(always)]
    pub fn lpsize(&self) -> LPSIZE_R {
        LPSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPMCCR")
            .field("vlpsize", &self.vlpsize())
            .field("lpsize", &self.lpsize())
            .finish()
    }
}
/**DSI Host low-power mode current configuration register

You can [`read`](crate::Reg::read) this register and get [`lpmccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#DSIHOST:LPMCCR)*/
pub struct LPMCCRrs;
impl crate::RegisterSpec for LPMCCRrs {
    type Ux = u32;
}
///`read()` method returns [`lpmccr::R`](R) reader structure
impl crate::Readable for LPMCCRrs {}
///`reset()` method sets LPMCCR to value 0
impl crate::Resettable for LPMCCRrs {}
