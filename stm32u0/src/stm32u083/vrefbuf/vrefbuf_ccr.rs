///Register `VREFBUF_CCR` reader
pub type R = crate::R<VREFBUF_CCRrs>;
///Register `VREFBUF_CCR` writer
pub type W = crate::W<VREFBUF_CCRrs>;
///Field `TRIM` reader - None
pub type TRIM_R = crate::FieldReader;
///Field `TRIM` writer - None
pub type TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - None
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VREFBUF_CCR")
            .field("trim", &self.trim())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - None
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W<VREFBUF_CCRrs> {
        TRIM_W::new(self, 0)
    }
}
/**VREFBUF calibration control register

You can [`read`](crate::Reg::read) this register and get [`vrefbuf_ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrefbuf_ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#VREFBUF:VREFBUF_CCR)*/
pub struct VREFBUF_CCRrs;
impl crate::RegisterSpec for VREFBUF_CCRrs {
    type Ux = u32;
}
///`read()` method returns [`vrefbuf_ccr::R`](R) reader structure
impl crate::Readable for VREFBUF_CCRrs {}
///`write(|w| ..)` method takes [`vrefbuf_ccr::W`](W) writer structure
impl crate::Writable for VREFBUF_CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VREFBUF_CCR to value 0
impl crate::Resettable for VREFBUF_CCRrs {
    const RESET_VALUE: u32 = 0;
}
