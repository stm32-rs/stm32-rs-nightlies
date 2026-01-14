///Register `NEXTHDPLCR` reader
pub type R = crate::R<NEXTHDPLCRrs>;
///Register `NEXTHDPLCR` writer
pub type W = crate::W<NEXTHDPLCRrs>;
///Field `NEXTHDPL` reader - index to point to a higher HDPL than the current one Index to add to the current HDPL to point (through OBK-HDPL) to the next secure storage areas (OBK-HDPL = HDPL + NEXTHDPL). See for more details.
pub type NEXTHDPL_R = crate::FieldReader;
///Field `NEXTHDPL` writer - index to point to a higher HDPL than the current one Index to add to the current HDPL to point (through OBK-HDPL) to the next secure storage areas (OBK-HDPL = HDPL + NEXTHDPL). See for more details.
pub type NEXTHDPL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - index to point to a higher HDPL than the current one Index to add to the current HDPL to point (through OBK-HDPL) to the next secure storage areas (OBK-HDPL = HDPL + NEXTHDPL). See for more details.
    #[inline(always)]
    pub fn nexthdpl(&self) -> NEXTHDPL_R {
        NEXTHDPL_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NEXTHDPLCR")
            .field("nexthdpl", &self.nexthdpl())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - index to point to a higher HDPL than the current one Index to add to the current HDPL to point (through OBK-HDPL) to the next secure storage areas (OBK-HDPL = HDPL + NEXTHDPL). See for more details.
    #[inline(always)]
    pub fn nexthdpl(&mut self) -> NEXTHDPL_W<'_, NEXTHDPLCRrs> {
        NEXTHDPL_W::new(self, 0)
    }
}
/**SBS next HDPL control register

You can [`read`](crate::Reg::read) this register and get [`nexthdplcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nexthdplcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#SBS:NEXTHDPLCR)*/
pub struct NEXTHDPLCRrs;
impl crate::RegisterSpec for NEXTHDPLCRrs {
    type Ux = u32;
}
///`read()` method returns [`nexthdplcr::R`](R) reader structure
impl crate::Readable for NEXTHDPLCRrs {}
///`write(|w| ..)` method takes [`nexthdplcr::W`](W) writer structure
impl crate::Writable for NEXTHDPLCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NEXTHDPLCR to value 0
impl crate::Resettable for NEXTHDPLCRrs {}
