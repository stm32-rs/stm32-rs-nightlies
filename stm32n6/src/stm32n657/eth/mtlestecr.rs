///Register `MTLESTECR` reader
pub type R = crate::R<MTLESTECRrs>;
///Register `MTLESTECR` writer
pub type W = crate::W<MTLESTECRrs>;
///Field `OVHD` reader - Overhead Bytes Value
pub type OVHD_R = crate::FieldReader;
///Field `OVHD` writer - Overhead Bytes Value
pub type OVHD_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - Overhead Bytes Value
    #[inline(always)]
    pub fn ovhd(&self) -> OVHD_R {
        OVHD_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLESTECR")
            .field("ovhd", &self.ovhd())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Overhead Bytes Value
    #[inline(always)]
    pub fn ovhd(&mut self) -> OVHD_W<'_, MTLESTECRrs> {
        OVHD_W::new(self, 0)
    }
}
/**EST Extended Control Register

You can [`read`](crate::Reg::read) this register and get [`mtlestecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlestecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:MTLESTECR)*/
pub struct MTLESTECRrs;
impl crate::RegisterSpec for MTLESTECRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlestecr::R`](R) reader structure
impl crate::Readable for MTLESTECRrs {}
///`write(|w| ..)` method takes [`mtlestecr::W`](W) writer structure
impl crate::Writable for MTLESTECRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLESTECR to value 0
impl crate::Resettable for MTLESTECRrs {}
