///Register `VNPCCR` reader
pub type R = crate::R<VNPCCRrs>;
///Register `VNPCCR` writer
pub type W = crate::W<VNPCCRrs>;
///Field `NPSIZE` reader - Null packet size
pub type NPSIZE_R = crate::FieldReader<u16>;
///Field `NPSIZE` writer - Null packet size
pub type NPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - Null packet size
    #[inline(always)]
    pub fn npsize(&self) -> NPSIZE_R {
        NPSIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VNPCCR")
            .field("npsize", &self.npsize())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - Null packet size
    #[inline(always)]
    pub fn npsize(&mut self) -> NPSIZE_W<VNPCCRrs> {
        NPSIZE_W::new(self, 0)
    }
}
/**DSI Host video null packet current configuration register

You can [`read`](crate::Reg::read) this register and get [`vnpccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vnpccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#DSIHOST:VNPCCR)*/
pub struct VNPCCRrs;
impl crate::RegisterSpec for VNPCCRrs {
    type Ux = u32;
}
///`read()` method returns [`vnpccr::R`](R) reader structure
impl crate::Readable for VNPCCRrs {}
///`write(|w| ..)` method takes [`vnpccr::W`](W) writer structure
impl crate::Writable for VNPCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VNPCCR to value 0
impl crate::Resettable for VNPCCRrs {
    const RESET_VALUE: u32 = 0;
}
