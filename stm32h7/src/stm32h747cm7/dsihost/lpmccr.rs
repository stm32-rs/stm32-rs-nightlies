///Register `LPMCCR` reader
pub type R = crate::R<LPMCCRrs>;
///Register `LPMCCR` writer
pub type W = crate::W<LPMCCRrs>;
///Field `VLPSIZE` reader - VACT largest packet size
pub type VLPSIZE_R = crate::FieldReader;
///Field `VLPSIZE` writer - VACT largest packet size
pub type VLPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `LPSIZE` reader - Largest packet size
pub type LPSIZE_R = crate::FieldReader;
///Field `LPSIZE` writer - Largest packet size
pub type LPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
            .field("lpsize", &self.lpsize())
            .field("vlpsize", &self.vlpsize())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - VACT largest packet size
    #[inline(always)]
    pub fn vlpsize(&mut self) -> VLPSIZE_W<LPMCCRrs> {
        VLPSIZE_W::new(self, 0)
    }
    ///Bits 16:23 - Largest packet size
    #[inline(always)]
    pub fn lpsize(&mut self) -> LPSIZE_W<LPMCCRrs> {
        LPSIZE_W::new(self, 16)
    }
}
/**DSI Host low-power mode current configuration register

You can [`read`](crate::Reg::read) this register and get [`lpmccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#DSIHOST:LPMCCR)*/
pub struct LPMCCRrs;
impl crate::RegisterSpec for LPMCCRrs {
    type Ux = u32;
}
///`read()` method returns [`lpmccr::R`](R) reader structure
impl crate::Readable for LPMCCRrs {}
///`write(|w| ..)` method takes [`lpmccr::W`](W) writer structure
impl crate::Writable for LPMCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPMCCR to value 0
impl crate::Resettable for LPMCCRrs {
    const RESET_VALUE: u32 = 0;
}
